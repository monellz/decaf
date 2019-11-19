use crate::{TypeCk, TypeCkTrait};
use common::{ErrorKind::*, HashMap, HashSet, Ref, MAIN_CLASS, MAIN_METHOD, NO_LOC};
use hashbrown::hash_map::Entry;
use std::{
    iter,
    ops::{Deref, DerefMut},
};
use syntax::{ast::*, ScopeOwner, Symbol, Ty, TyKind};

pub(crate) struct SymbolPass<'a>(pub TypeCk<'a>);

// some boilerplate code...
impl<'a> Deref for SymbolPass<'a> {
    type Target = TypeCk<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for SymbolPass<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> SymbolPass<'a> {
    pub fn program(&mut self, p: &'a Program<'a>) {
        // the global scope is already opened, so no need to open it here
        for c in &p.class {
            if let Some(prev) = self.scopes.lookup_class(c.name) {
                self.issue(
                    c.loc,
                    ConflictDeclaration {
                        prev: prev.loc,
                        name: c.name,
                    },
                )
            } else {
                self.scopes.declare(Symbol::Class(c));
            }
        }
        for c in &p.class {
            if let Some(p) = c.parent {
                c.parent_ref.set(self.scopes.lookup_class(p));
                if c.parent_ref.get().is_none() {
                    self.issue(c.loc, NoSuchClass(p))
                }
            }
        }
        // detect cyclic inheritance
        let mut vis = HashMap::new();
        for (idx, c) in p.class.iter().enumerate() {
            let mut c = *c;
            let mut last = c; // this assignment is useless, the value of `last` never comes from it when used
            loop {
                match vis.entry(Ref(c)) {
                    Entry::Vacant(v) => {
                        v.insert(idx);
                        if let Some(p) = c.parent_ref.get() {
                            (last = c, c = p);
                        } else {
                            break;
                        }
                    }
                    Entry::Occupied(o) => {
                        if *o.get() == idx {
                            self.issue(last.loc, CyclicInheritance)
                        }
                        break;
                    }
                }
            }
        }
        // errors related to inheritance are considered as fatal errors, return after these checks if a error occurred
        if !self.errors.0.is_empty() {
            return;
        }
        let mut checked = HashSet::new();
        let mut abs_func_set = HashMap::new();
        for c in &p.class {
            self.class_def(c, &mut checked, &mut abs_func_set);
            if c.name == MAIN_CLASS && !c.abstract_ {
                //main class should not be abstract
                p.main.set(Some(c));
            }
        }
        if p.main
            .get()
            .map(|c| match c.scope.borrow().get(MAIN_METHOD) {
                Some(Symbol::Func(main))
                    //main func should not be abstract
                    if !main.abstract_ && main.static_ && main.param.is_empty() && main.ret_ty() == Ty::void() =>
                {
                    false
                }
                _ => true,
            })
            .unwrap_or(true)
        {
            self.issue(NO_LOC, NoMainClass)
        }
    }

    fn class_def(&mut self, c: &'a ClassDef<'a>, checked: &mut HashSet<Ref<'a, ClassDef<'a>>>, abs_func_map: &mut HashMap<&'a str, HashSet<&'a str>>) -> HashSet<&'a str> {
        if !checked.insert(Ref(c)) {
            //if checked already has c, return its set
            return abs_func_map.get(c.name).expect("cannot find hashset").clone();
        }

        let mut abs_func_set = if let Some(p) = c.parent_ref.get() {
            //get parent's abstract func set
            self.class_def(p, checked, abs_func_map)
        } else {
            HashSet::new()
        };

        self.cur_class = Some(c);
        self.scoped(ScopeOwner::Class(c), |s| {
            for f in &c.field {
                match f {
                    FieldDef::FuncDef(f) => s.func_def(f, &mut abs_func_set),
                    FieldDef::VarDef(v) => s.var_def(v),
                };
            }
        });

        //println!("class: {}, set = {:?}", c.name, abs_func_set);
        if !abs_func_set.is_empty() && !c.abstract_ {
            //non abstract class has abstract func which is not implement
            self.issue(c.loc, NotOverrideAllAbstractFunc(c.name))
        }

        abs_func_map.insert(c.name, abs_func_set.clone());
        abs_func_set
    }

    fn func_def(&mut self, f: &'a FuncDef<'a>, class_abs_func_set: &mut HashSet<&'a str>) {
        let ret_ty = self.ty(&f.ret, false);
        self.scoped(ScopeOwner::Param(f), |s| {
            if !f.static_ {
                s.scopes.declare(Symbol::This(f));
            }
            for v in &f.param {
                s.var_def(v);
            }

            if !f.abstract_ { s.block(&f.body.as_ref().expect("unwrap a none func body")); }
            else { class_abs_func_set.insert(f.name); }

        });
        let ret_param_ty = iter::once(ret_ty).chain(f.param.iter().map(|v| v.ty.get()));
        let ret_param_ty = self.alloc.ty.alloc_extend(ret_param_ty);
        f.ret_param_ty.set(Some(ret_param_ty));
        f.class.set(self.cur_class);
        let ok = if let Some((sym, owner)) = self.scopes.lookup(f.name) {
            match (self.scopes.cur_owner(), owner) {
                (ScopeOwner::Class(c), ScopeOwner::Class(p)) if Ref(c) != Ref(p) => match sym {
                    Symbol::Func(pf) => {
                        if (f.static_ || pf.static_) || (f.abstract_ && !pf.abstract_) {
                            //subclass cannot abstract override superclass non-abstract func
                            self.issue(
                                f.loc,
                                ConflictDeclaration {
                                    prev: pf.loc,
                                    name: f.name,
                                },
                            )
                        } else if !Ty::mk_func(f).assignable_to(Ty::mk_func(pf)) {
                            self.issue(
                                f.loc,
                                OverrideMismatch {
                                    func: f.name,
                                    p: p.name,
                                },
                            )
                        } else {
                            //override checked
                            if !f.abstract_ && !f.static_ {
                                class_abs_func_set.remove(f.name);
                            }
                            true
                        }
                    }
                    _ => self.issue(
                        f.loc,
                        ConflictDeclaration {
                            prev: sym.loc(),
                            name: f.name,
                        },
                    ),
                },
                _ => self.issue(
                    f.loc,
                    ConflictDeclaration {
                        prev: sym.loc(),
                        name: f.name,
                    },
                ),
            }
        } else {
            true
        };
        if ok {
            self.scopes.declare(Symbol::Func(f));
        }
    }

    fn var_def(&mut self, v: &'a VarDef<'a>) {
        //type inference is delayed to type_pass
        if let Some(syn_ty) = &v.syn_ty {
            v.ty.set(self.ty(&syn_ty, false));
            if v.ty.get() == Ty::void() {
                self.issue(v.loc, VoidVar(v.name))
            }
        }
        /*
        v.ty.set(self.ty(&v.syn_ty.as_ref().expect("unwrap a non syn_ty"), false));
        if v.ty.get() == Ty::void() {
            self.issue(v.loc, VoidVar(v.name))
        }
        */
        let ok = if let Some((sym, owner)) = self.scopes.lookup(v.name) {
            match (self.scopes.cur_owner(), owner) {
                (ScopeOwner::Class(c1), ScopeOwner::Class(c2))
                    if Ref(c1) != Ref(c2) && sym.is_var() =>
                {
                    self.issue(v.loc, OverrideVar(v.name))
                }
                (ScopeOwner::Class(_), ScopeOwner::Class(_))
                | (_, ScopeOwner::Param(_))
                | (_, ScopeOwner::Local(_))
                | (_, ScopeOwner::LambdaParam(_)) => self.issue(
                    v.loc,
                    ConflictDeclaration {
                        prev: sym.loc(),
                        name: v.name,
                    },
                ),
                _ => true,
            }
        } else {
            true
        };
        if ok {
            v.owner.set(Some(self.scopes.cur_owner()));
            self.scopes.declare(Symbol::Var(v));
        }
    }

    fn block(&mut self, b: &'a Block<'a>) {
        self.scoped(ScopeOwner::Local(b), |s| {
            for st in &b.stmt {
                s.stmt(st);
            }
        });
    }

    fn stmt(&mut self, s: &'a Stmt<'a>) {
        match &s.kind {
            StmtKind::LocalVarDef(v) => {
                self.var_def(v);
                if let Some((_, e)) = &v.init {
                    self.expr(e);
                }
            },
            StmtKind::If(i) => {
                self.block(&i.on_true);
                if let Some(of) = &i.on_false {
                    self.block(of);
                }
            }
            StmtKind::While(w) => self.block(&w.body),
            StmtKind::For(f) => self.scoped(ScopeOwner::Local(&f.body), |s| {
                s.stmt(&f.init);
                s.stmt(&f.update);
                for st in &f.body.stmt {
                    s.stmt(st);
                }
            }),
            StmtKind::Block(b) => self.block(b),

            //add support for lambda symbol
            StmtKind::Assign(a) => {
                self.expr(&a.dst);
                self.expr(&a.src);
            },
            StmtKind::ExprEval(e) => self.expr(e),
            StmtKind::Return(r) => {
                if let Some(e) = r {
                    self.expr(e);
                }
            },
            StmtKind::Print(p) => {
                for e in p {
                    self.expr(e);
                }
            },
            _ => {},
        };
    }


    fn expr(&mut self, e: &'a Expr<'a>) {
        use ExprKind::*;
        match &e.kind {
            Lambda(lam) => {
                //add lambda to the symbol table
                self.scoped(ScopeOwner::LambdaParam(lam), |s| {
                    for v in &lam.param {
                        println!("   def v = {}, at {:?} and lam = {:?}", v.name, e.loc, lam.name);
                        s.var_def(v);
                    }
                    match &lam.kind {
                        LambdaKind::Expr(e) => {
                            s.scoped(ScopeOwner::LambdaExprLocal(lam), |s| {
                                s.expr(e);
                            });
                        },
                        LambdaKind::Block(block) => s.block(block),
                    };
                });               
                println!("declar lam = {:?}", lam.name);
                match self.scopes.cur_owner() {
                    ScopeOwner::Local(_) => println!("  in local"),
                    ScopeOwner::Param(_) => println!("  in param"),
                    ScopeOwner::LambdaParam(_) => println!("  in lambda param"),
                    ScopeOwner::LambdaExprLocal(_) => println!(" in lambda expr local"),
                    _ => println!("???"),
                };
                self.scopes.declare(Symbol::Lambda(lam));
            },
            IndexSel(i) => {
                println!("into index sel at {:?}", e.loc);
                self.expr(&i.arr);
                self.expr(&i.idx);
            },
            Call(c) => {
                self.expr(&c.func);
                for e in &c.arg {
                    self.expr(e);
                }
            },
            Unary(u) => self.expr(&u.r),
            Binary(b) => {
                self.expr(&b.l);
                self.expr(&b.r);
            },
            NewArray(n) => self.expr(&n.len),
            ClassTest(c) => self.expr(&c.expr),
            ClassCast(c) => self.expr(&c.expr),
            _ => {},
        };
    }

}