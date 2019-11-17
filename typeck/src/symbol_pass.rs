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
                | (_, ScopeOwner::Local(_)) => self.issue(
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
            StmtKind::LocalVarDef(v) => self.var_def(v),
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
            StmtKind::Assign(a) => self.expr(&a.src),
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
                        s.var_def(v);
                    }
                    //TODO! not set the ret_ty
                    match &lam.kind {
                        LambdaKind::Expr(_) => {},
                        LambdaKind::Block(block) => s.block(&block),
                    };
                });               
                self.scopes.declare(Symbol::Lambda(lam));
            },
            _ => {},
        };
    }

    fn get_lambda_ret_ty(e: &'a LambdaDef) -> &'a Ty<'a> {

        unimplemented!()
    }

    fn get_upper_ty(&mut self, ty_list: &[&'a Ty<'a>], loc: common::Loc) -> &'a Ty<'a> {
        let ret_ty = self.alloc.ty.alloc(Ty::null());
        for (idx, t) in ty_list.iter().enumerate() {
            if let TyKind::Null = t.kind { continue; }
            else {
                match t.kind {
                    TyKind::Int | TyKind::Bool | TyKind::String | TyKind::Void | _ if t.arr > 0 => {
                        for other_t in ty_list {
                            if Ref(other_t) != Ref(t) {
                                let _: u32 = self.issue(loc, IncompatibleReturnType);
                                *ret_ty = **t;
                                break;
                            }
                        }
                        return ret_ty;
                    },
                    TyKind::Class(c) => {
                        let p = self.alloc.ty.alloc(**t);
                        let mut p_c = &*c;
                        let mut all_assignable = true;
                        loop {
                            for other_t in ty_list {
                                if !other_t.assignable_to(*p) {
                                    //set p to p's parent and reloop
                                    all_assignable = false;
                                    if let Some(p_f_c) = p_c.parent_ref.get() {
                                        p_c = p_f_c;
                                        *p = Ty::mk_class(&p_f_c);
                                        break;
                                    } else {
                                        let _: u32 = self.issue(loc, IncompatibleReturnType);
                                        return p;
                                    }
                                }
                            }
                            if all_assignable {
                                return p;
                            }
                        }
                    },
                    TyKind::Func(args) => {
                        for (other_idx, other_t) in ty_list.iter().enumerate() {
                            if idx == other_idx { continue; }
                            else {
                                //check`
                                match other_t.kind {
                                    TyKind::Func(other_args) if other_args.len() == args.len() => {
                                        //check pass
                                    },
                                    _ => {
                                        let _: u32 = self.issue(loc, IncompatibleReturnType);
                                        return ret_ty;
                                    }
                                };
                            }
                        }

                        let mut finnal_ty = Vec::new();
                        let mut ret_args = Vec::new();
                        for cur_ty in ty_list {
                            ret_args.push(match &cur_ty.kind {
                                TyKind::Func(cur_ty_args) => &cur_ty_args[0],
                                _ => unreachable!(),
                            });
                        }
                        finnal_ty.push(*self.get_upper_ty(ret_args.as_slice(), loc));


                        for args_idx in 1..args.len() {
                            let mut cur_args = Vec::with_capacity(ty_list.len() - 1);
                            for cur_ty in ty_list {
                                cur_args.push(match &cur_ty.kind {
                                    TyKind::Func(cur_ty_args) => &cur_ty_args[args_idx],
                                    _ => unreachable!(),
                                });
                            }
                            finnal_ty.push(*self.get_lower_ty(cur_args.as_slice(), loc));
                        }

                        *ret_ty = Ty { arr: t.arr, kind: TyKind::Func(self.alloc.ty.alloc_extend(finnal_ty)) };
                        return ret_ty;
                    },
                    _ => unreachable!(),
                };
            }
        }
        assert!(*ret_ty != Ty::null(), "default ret_ty must be modified");
        ret_ty
    }

    fn get_lower_ty(&mut self, ty_list: &[&'a Ty<'a>], loc: common::Loc) -> &'a Ty<'a> {
        let ret_ty = self.alloc.ty.alloc(Ty::null());
        for (idx, t) in ty_list.iter().enumerate() {
            if let TyKind::Null = t.kind { continue; }
            else {
                match t.kind {
                    TyKind::Int | TyKind::Bool | TyKind::String | TyKind::Void | _ if t.arr > 0 => {
                        for other_t in ty_list {
                            if Ref(other_t) != Ref(t) {
                                let _: u32 = self.issue(loc, IncompatibleReturnType);
                                *ret_ty = **t;
                                break;
                            }
                        }
                        return ret_ty;
                    },
                    TyKind::Class(c) => {
                        //check that all the ty is class
                        for cur_ty in ty_list {
                            match cur_ty.kind {
                                TyKind::Class(_) => continue,
                                _ => {
                                    let _: u32 = self.issue(loc, IncompatibleReturnType);
                                    return ret_ty;
                                },
                            };
                        }

                        let p = self.alloc.ty.alloc(**t);
                        let mut p_c = &*c;
                        let mut all_assignable = true;
                        for cur_ty in ty_list {
                            //check whether cur_ty is the lower bound
                            let p = self.alloc.ty.alloc(**cur_ty);
                            let mut p_c = match &cur_ty.kind {
                                TyKind::Class(p_c) => &**p_c,
                                _ => unreachable!(),
                            };
                            let mut non_check_cnt = ty_list.len() as i32;
                            loop {
                                for checking_ty in ty_list {
                                    if Ref(p) == Ref(checking_ty) {
                                        non_check_cnt -= 1;
                                    }
                                }
                                if non_check_cnt > 0 {
                                    //back to parent
                                    if let Some(p_f_c) = p_c.parent_ref.get() {
                                        p_c = p_f_c;
                                        *p = Ty::mk_class(&p_f_c);
                                    } else {
                                        //no parent
                                        break;
                                    }
                                } else {
                                    //checked
                                    *ret_ty = **cur_ty;
                                    return ret_ty;
                                }
                            }
                        }

                        let _:u32 = self.issue(loc, IncompatibleReturnType);
                        return ret_ty;
                    },
                    TyKind::Func(args) => {
                        for (other_idx, other_t) in ty_list.iter().enumerate() {
                            if idx == other_idx { continue; }
                            else {
                                //check`
                                match other_t.kind {
                                    TyKind::Func(other_args) if other_args.len() == args.len() => {
                                        //check pass
                                    },
                                    _ => {
                                        let _: u32 = self.issue(loc, IncompatibleReturnType);
                                        return ret_ty;
                                    }
                                };
                            }
                        }

                        let mut finnal_ty = Vec::new();
                        let mut ret_args = Vec::new();
                        for cur_ty in ty_list {
                            ret_args.push(match &cur_ty.kind {
                                TyKind::Func(cur_ty_args) => &cur_ty_args[0],
                                _ => unreachable!(),
                            });
                        }
                        finnal_ty.push(*self.get_lower_ty(ret_args.as_slice(), loc));


                        for args_idx in 1..args.len() {
                            let mut cur_args = Vec::with_capacity(ty_list.len() - 1);
                            for cur_ty in ty_list {
                                cur_args.push(match &cur_ty.kind {
                                    TyKind::Func(cur_ty_args) => &cur_ty_args[args_idx],
                                    _ => unreachable!(),
                                });
                            }
                            finnal_ty.push(*self.get_upper_ty(cur_args.as_slice(), loc));
                        }

                        *ret_ty = Ty { arr: t.arr, kind: TyKind::Func(self.alloc.ty.alloc_extend(finnal_ty)) };
                        return ret_ty;
                    },
                    _ => unreachable!(),
                };
            }
        }
        assert!(*ret_ty != Ty::null(), "default ret_ty must be modified");
        ret_ty
    }
}
