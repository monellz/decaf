use crate::{TypeCk, TypeCkTrait};
use common::{BinOp, ErrorKind, ErrorKind::*, Loc, Ref, UnOp, LENGTH};
use std::ops::{Deref, DerefMut};
use syntax::ast::*;
use syntax::{ty::*, ScopeOwner, Symbol};

pub(crate) struct TypePass<'a>(pub TypeCk<'a>);

impl<'a> Deref for TypePass<'a> {
    type Target = TypeCk<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for TypePass<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> TypePass<'a> {
    pub fn program(&mut self, p: &'a Program<'a>) {
        for c in &p.class {
            self.class_def(c);
        }
    }

    fn class_def(&mut self, c: &'a ClassDef<'a>) {
        self.cur_class = Some(c);
        self.scoped(ScopeOwner::Class(c), |s| {
            for f in &c.field {
                if let FieldDef::FuncDef(f) = f {
                    s.cur_func = Some(f);
                    //TODO: non block return false?
                    let ret = s.scoped(ScopeOwner::Param(f), |s| if !f.abstract_ { s.block(&f.body.as_ref().expect("unwrap a non func body")) } else { false });
                    if !f.abstract_ && !ret && f.ret_ty() != Ty::void() {
                        //func is not abstract & no block ret & f.ret_ty not void
                        s.issue(f.body.as_ref().expect("unwrap a non func body").loc, ErrorKind::NoReturn)
                    }
                };
            }
        });
    }

    fn block(&mut self, b: &'a Block<'a>) -> bool {
        let mut ret = false;
        self.scoped(ScopeOwner::Local(b), |s| {
            for st in &b.stmt {
                ret = s.stmt(st);
            }
        });
        ret
    }

    // return whether this stmt has a return value
    fn stmt(&mut self, s: &'a Stmt<'a>) -> bool {
        match &s.kind {
            StmtKind::Assign(a) => {
                let (l, r) = (self.expr(&a.dst), self.expr(&a.src));
                if !r.assignable_to(l) {
                    self.issue(s.loc, IncompatibleBinary { l, op: "=", r })
                }

                if let ExprKind::VarSel(vs) = &a.dst.kind {
                    if let Some(owner) = &vs.owner {
                        self.cur_used = true;
                        let owner = self.expr(owner);
                        self.cur_used = false;
                        match owner {
                            Ty { arr: 0, kind: TyKind::Object(Ref(c))} => {
                                if let Some(sym) = c.lookup(vs.name) {
                                    if let Symbol::Func(f) = sym {
                                        if !(self.cur_func.unwrap().static_ && !f.static_) {
                                            self.issue(s.loc, AssignToClassMethod(vs.name))
                                        }
                                    } 
                                } else {
                                    unreachable!();
                                }
                            },
                            Ty { arr: 0, kind: TyKind::Class(Ref(c))} => {
                                //println!("vs.name = {} c.name = {}", vs.name, c.name);
                                if let Some(sym) = c.lookup(vs.name) {
                                    if let Symbol::Func(f) = sym {
                                        if f.static_ {
                                            self.issue(s.loc, AssignToClassMethod(vs.name))
                                        }
                                    }
                                }
                            },
                            _ => unreachable!(),
                        }
                    } else {
                        if let Some(sym) = self.scopes.lookup_before(vs.name, a.dst.loc) {
                            match sym {
                                Symbol::Func(_) => self.issue(s.loc, AssignToClassMethod(vs.name)),
                                Symbol::Var(v) => {
                                    if let Some(lam) = self.cur_lambda {
                                        if let LambdaKind::Block(b) = &lam.kind {
                                            let is_in_block = b.scope.borrow().contains_key(vs.name);
                                            let is_in_param = lam.scope.borrow().contains_key(vs.name);
                                            if !is_in_block && !is_in_param {
                                                //TODO: how to simplify it??
                                                //TODO: consider the euqality??
                                                use ScopeOwner::*;
                                                match v.owner.get().unwrap() {
                                                    Local(_) | Param(_) | LambdaParam(_) | Global(_) => {
                                                        //no-class scope
                                                        self.issue(s.loc, AssignToCapturedVariable)
                                                    }
                                                    Class(_) => {}, 
                                                };
                                            }
                                        }
                                    }
                                },
                                _ => {},
                            }
                        } else {
                            //TODO????
                            //unreachable!("weird");
                        }
                    }
                } else {
                    //println!("  not varsel loc = {:?}", a.dst.loc);
                }

                false
            }
            StmtKind::LocalVarDef(v) => {
                self.cur_var_def = Some(v);
                if let Some((loc, e)) = &v.init {
                    match &v.syn_ty {
                        Some(_) => {
                            let (l, r) = (v.ty.get(), self.expr(e));
                            if !r.assignable_to(l) {
                                self.issue(*loc, IncompatibleBinary { l, op: "=", r })
                            }
                        },
                        None => {
                            //type inference check
                            let r = self.expr(e);
                            if let TyKind::Void = r.kind {
                                self.issue(v.loc, InferVoid(v.name))
                            } else {
                                v.ty.set(r);
                            }
                        }

                    }
                    /*
                    let (l, r) = (v.ty.get(), self.expr(e));
                    if !r.assignable_to(l) {
                        self.issue(*loc, IncompatibleBinary { l, op: "=", r })
                    }
                    */
                }
                self.cur_var_def = None;
                false
            }
            StmtKind::ExprEval(e) => {
                self.expr(e);
                false
            }
            StmtKind::Skip(_) => false,
            StmtKind::If(i) => {
                self.check_bool(&i.cond);
                // `&` is not short-circuit evaluated
                self.block(&i.on_true) & i.on_false.as_ref().map(|b| self.block(b)).unwrap_or(false)
            }
            StmtKind::While(w) => {
                self.check_bool(&w.cond);
                self.loop_cnt += 1;
                self.block(&w.body);
                self.loop_cnt -= 1;
                false
            }
            StmtKind::For(f) => self.scoped(ScopeOwner::Local(&f.body), |s| {
                s.stmt(&f.init);
                s.check_bool(&f.cond);
                s.stmt(&f.update);
                for st in &f.body.stmt {
                    s.stmt(st);
                } // not calling block(), because the scope is already opened
                false
            }),
            StmtKind::Return(r) => {
                let expect = if let Some(lam) = self.cur_lambda {
                    lam.ret_ty()
                } else {
                    self.cur_func.unwrap().ret_ty()
                };
                //println!("loc = {:?}, self.cur_func.name = {} cur_lambda = {}", s.loc, self.cur_func.unwrap().name, self.cur_lambda.unwrap().name);
                let actual = r.as_ref().map(|e| self.expr(e)).unwrap_or(Ty::void());
                if !actual.assignable_to(expect) {
                    self.issue(s.loc, ReturnMismatch { actual, expect })
                }
                actual != Ty::void()
            }
            StmtKind::Print(p) => {
                for (i, e) in p.iter().enumerate() {
                    let ty = self.expr(e);
                    if ty != Ty::bool() && ty != Ty::int() && ty != Ty::string() {
                        ty.error_or(|| {
                            self.issue(
                                e.loc,
                                BadPrintArg {
                                    loc: i as u32 + 1,
                                    ty,
                                },
                            )
                        })
                    }
                }
                false
            }
            StmtKind::Break(_) => {
                if self.loop_cnt == 0 {
                    self.issue(s.loc, BreakOutOfLoop)
                }
                false
            }
            StmtKind::Block(b) => self.block(b),
        }
    }

    // e.ty is set to the return value
    fn expr(&mut self, e: &'a Expr<'a>) -> Ty<'a> {
        use ExprKind::*;
        let ty = match &e.kind {
            VarSel(v) => self.var_sel(v, e.loc),
            IndexSel(i) => {
                let (arr, idx) = (self.expr(&i.arr), self.expr(&i.idx));
                if idx != Ty::int() {
                    idx.error_or(|| self.issue(e.loc, IndexNotInt))
                }
                match arr {
                    Ty { arr, kind } if arr > 0 => Ty { arr: arr - 1, kind },
                    e => e.error_or(|| self.issue(i.arr.loc, IndexNotArray)),
                }
            },
            Lambda(lam) => {
                match &lam.kind {
                    LambdaKind::Expr(e) => {
                        if let None = lam.ret_param_ty.get() {
                            self.cur_lambda = Some(lam);
                            let ret_ty = self.scoped(ScopeOwner::LambdaParam(lam), |s| {
                                s.expr(e)
                            });
                            self.cur_lambda = None;
                            let ret_param_ty = std::iter::once(ret_ty).chain(lam.param.iter().map(|v| v.ty.get()));
                            let ret_param_ty = self.alloc.ty.alloc_extend(ret_param_ty);
                            lam.ret_param_ty.set(Some(ret_param_ty));
                        } 
                        Ty::mk_lambda(lam)
                    },
                    LambdaKind::Block(b) => {
                        if let None = lam.ret_param_ty.get() {
                            let mut ty_list = Vec::new();    
                            //println!("start to cal ret_ty loc = {:?} name = {}", e.loc, lam.name);
                            let prev_lambda = self.cur_lambda;
                            self.cur_lambda = Some(lam);
                            self.scoped(ScopeOwner::LambdaParam(lam), |s| {
                                s.scoped(ScopeOwner::Local(b), |inner_s| {
                                    inner_s.ret_ty_in_block(b, &mut ty_list);
                                });
                            });
                            self.cur_lambda = prev_lambda;
                            if ty_list.len() == 0 { ty_list.push(self.alloc.ty.alloc(Ty::void())); }
                            let ret_ty = self.get_upper_ty(ty_list.as_slice(), e.loc);
                            let ret_param_ty = std::iter::once(*ret_ty).chain(lam.param.iter().map(|v| v.ty.get()));
                            let ret_param_ty = self.alloc.ty.alloc_extend(ret_param_ty);
                            lam.ret_param_ty.set(Some(ret_param_ty));
                        }
 
                        let prev_lambda = self.cur_lambda;
                        self.cur_lambda = Some(lam);
                        self.scoped(ScopeOwner::LambdaParam(lam), |s| {
                            s.block(&b);
                        });
                        self.cur_lambda = prev_lambda;
                        Ty::mk_lambda(lam)
                    },
                }
                //unimplemented!()
            },
            IntLit(_) | ReadInt(_) => Ty::int(),
            BoolLit(_) => Ty::bool(),
            StringLit(_) | ReadLine(_) => Ty::string(),
            NullLit(_) => Ty::null(),
            Call(c) => {
                self.call(c, e.loc)
            },
            Unary(u) => {
                let r = self.expr(&u.r);
                let (ty, op) = match u.op {
                    UnOp::Neg => (Ty::int(), "-"),
                    UnOp::Not => (Ty::bool(), "!"),
                };
                if r != ty {
                    r.error_or(|| self.issue(e.loc, IncompatibleUnary { op, r }))
                }
                ty
            }
            Binary(b) => {
                use BinOp::*;
                let (l, r) = (self.expr(&b.l), self.expr(&b.r));
                if l == Ty::error() || r == Ty::error() {
                    // not using wildcard match, so that if we add new operators in the future, compiler can tell us
                    match b.op {
                        Add | Sub | Mul | Div | Mod => Ty::int(),
                        And | Or | Eq | Ne | Lt | Le | Gt | Ge => Ty::bool(),
                    }
                } else {
                    let (ret, ok) = match b.op {
                        Add | Sub | Mul | Div | Mod => {
                            (Ty::int(), l == Ty::int() && r == Ty::int())
                        }
                        Lt | Le | Gt | Ge => (Ty::bool(), l == Ty::int() && r == Ty::int()),
                        Eq | Ne => (Ty::bool(), l.assignable_to(r) || r.assignable_to(l)),
                        And | Or => (Ty::bool(), l == Ty::bool() && r == Ty::bool()),
                    };
                    if !ok {
                        self.issue(
                            e.loc,
                            IncompatibleBinary {
                                l,
                                op: b.op.to_op_str(),
                                r,
                            },
                        )
                    }
                    ret
                }
            }
            This(_) => {
                if self.cur_func.unwrap().static_ {
                    self.issue(e.loc, ThisInStatic)
                }
                Ty::mk_obj(self.cur_class.unwrap())
            }
            NewClass(n) => {
                if let Some(c) = self.scopes.lookup_class(n.name) {
                    //cannot instantiate abstract class
                    if c.abstract_ {
                        self.issue(e.loc, InstantiateAbstractClass(n.name))
                    }
                    n.class.set(Some(c));
                    Ty::mk_obj(c)
                } else {
                    self.issue(e.loc, NoSuchClass(n.name))
                }
            }
            NewArray(n) => {
                let len = self.expr(&n.len);
                if len != Ty::int() {
                    len.error_or(|| self.issue(n.len.loc, NewArrayNotInt))
                }
                self.ty(&n.elem, true)
            }
            ClassTest(c) => {
                let src = self.expr(&c.expr);
                if !src.is_object() {
                    src.error_or(|| self.issue(e.loc, NotObject(src)))
                }
                if let Some(cl) = self.scopes.lookup_class(c.name) {
                    c.class.set(Some(cl));
                    Ty::bool()
                } else {
                    self.issue(e.loc, NoSuchClass(c.name))
                }
            }
            ClassCast(c) => {
                let src = self.expr(&c.expr);
                if !src.is_object() {
                    src.error_or(|| self.issue(e.loc, NotObject(src)))
                }
                if let Some(cl) = self.scopes.lookup_class(c.name) {
                    c.class.set(Some(cl));
                    Ty::mk_obj(cl)
                } else {
                    self.issue(e.loc, NoSuchClass(c.name))
                }
            }
        };
        e.ty.set(ty);
        ty
    }

    fn var_sel(&mut self, v: &'a VarSel<'a>, loc: Loc) -> Ty<'a> {
        // (no owner)not_found_var / ClassName(no field) / (no owner)method => UndeclaredVar
        // object.not_found_var => NoSuchField
        // (no owner)field_var && cur function is static => RefInStatic
        // <not object>.a (e.g.: Class.a, 1.a) / object.method => BadFieldAccess
        // object.field_var, where object's class is not self or any of ancestors => PrivateFieldAccess

        if let Some(owner) = &v.owner {
            self.cur_used = true;
            let owner = self.expr(owner);
            self.cur_used = false;
            match owner {
                Ty {
                    arr: 0,
                    kind: TyKind::Object(Ref(c)),
                } => {
                    if let Some(sym) = c.lookup(v.name) {
                        match sym {
                            Symbol::Var(var) => {
                                v.var.set(Some(var));
                                // only allow self & descendents to access field
                                if !self.cur_class.unwrap().extends(c) {
                                    self.issue(
                                        loc,
                                        PrivateFieldAccess {
                                            name: v.name,
                                            owner,
                                        },
                                    )
                                }
                                var.ty.get()
                            },
                            Symbol::Func(f) => {
                                //TODO? Fundef should be added to VarSel???
                                if self.cur_func.unwrap().static_ && !f.static_ {
                                    let cur_func_name = self.cur_func.unwrap().name;
                                    self.issue(loc, RefInStatic { field: f.name, func: cur_func_name })
                                }
                                Ty::mk_func(f)
                            },
                            Symbol::Lambda(_) => {
                                unreachable!("it seems impossible")
                            },
                            _ => {
                                self.issue(
                                    loc,
                                    BadFieldAccess {
                                        name: v.name,
                                        owner,
                                    },
                                )
                            }
                        }
                    } else {
                        self.issue(
                            loc,
                            NoSuchField {
                                name: v.name,
                                owner,
                            },
                        )
                    }
                },
                Ty {
                    arr: 0,
                    kind: TyKind::Class(Ref(c)),
                } => {
                    //var should be static
                    if let Some(sym) = c.lookup(v.name) {
                        match sym {
                            Symbol::Func(f) => {
                                if !f.static_ {
                                    self.issue(loc, BadFieldAccess { name: f.name, owner})
                                }
                                Ty::mk_func(f)
                            },
                            Symbol::Lambda(_) => {
                                unreachable!("a field of class is lambda? impossible");
                            }
                            _ => {
                                self.issue(
                                    loc,
                                    BadFieldAccess {
                                        name: v.name,
                                        owner,
                                    },
                                )
                            }
                        }
                    } else {
                        self.issue(
                            loc,
                            NoSuchField {
                                name: v.name,
                                owner,
                            },
                        )
                    }
                },
                e => e.error_or(|| {
                    self.issue(
                        loc,
                        BadFieldAccess {
                            name: v.name,
                            owner,
                        },
                    )
                }),
            }
        } else {
            // if this stmt is in an VarDef, it cannot access the variable that is being declared
            if let Some(sym) = self
                .scopes
                //.lookup_before(v.name, self.cur_var_def.map(|v| v.loc).unwrap_or(loc))
                .lookup_before(v.name, loc)
            {
                match sym {
                    Symbol::Var(var) => {
                        v.var.set(Some(var));
                        if var.owner.get().unwrap().is_class() {
                            let cur = self.cur_func.unwrap();
                            if cur.static_ {
                                self.issue(
                                    loc,
                                    RefInStatic {
                                        field: v.name,
                                        func: cur.name,
                                    },
                                )
                            }
                        }
                        var.ty.get()
                    }
                    Symbol::Class(c) if self.cur_used => Ty::mk_class(c),
                    
                    Symbol::Func(f) => {
                        let cur = self.cur_func.unwrap();
                        if cur.static_ && !f.static_ {
                            self.issue(loc, RefInStatic { field: f.name, func: cur.name})
                        }
                        Ty::mk_func(f)
                    },
                    _ => {
                        self.issue(loc, UndeclaredVar(v.name))
                    }
                }
            } else {
                self.issue(loc, UndeclaredVar(v.name))
            }
        }
    }

    fn call(&mut self, c: &'a Call<'a>, loc: Loc) -> Ty<'a> {
        let v = if let ExprKind::VarSel(v) = &c.func.kind {
            v
        } else {
            unimplemented!()
        };
        let owner = if let Some(owner) = &v.owner {
            self.cur_used = true;
            let owner = self.expr(owner);
            self.cur_used = false;
            if owner == Ty::error() {
                return Ty::error();
            }
            if v.name == LENGTH && owner.is_arr() {
                if !c.arg.is_empty() {
                    self.issue(loc, LengthWithArgument(c.arg.len() as u32))
                }
                return Ty::int();
            }
            owner
        } else {
            Ty::mk_obj(self.cur_class.unwrap())
        };
        match owner {
            Ty {
                arr: 0,
                kind: TyKind::Object(Ref(cl)),
            }
            | Ty {
                arr: 0,
                kind: TyKind::Class(Ref(cl)),
            } => {
                if let Some(sym) = cl.lookup(v.name) {
                    match sym {
                        Symbol::Func(f) => {
                            c.func_ref.set(Some(f));
                            if owner.is_class() && !f.static_ {
                                // Class.not_static_method()
                                self.issue(
                                    loc,
                                    BadFieldAccess {
                                        name: v.name,
                                        owner,
                                    },
                                )
                            }
                            if v.owner.is_none() {
                                let cur = self.cur_func.unwrap();
                                if cur.static_ && !f.static_ {
                                    self.issue(
                                        loc,
                                        RefInStatic {
                                            field: f.name,
                                            func: cur.name,
                                        },
                                    )
                                }
                            }
                            self.check_arg_param(&c.arg, f.ret_param_ty.get().unwrap(), f.name, loc)
                        }
                        _ => self.issue(
                            loc,
                            NotFunc {
                                name: v.name,
                                owner,
                            },
                        ),
                    }
                } else {
                    self.issue(
                        loc,
                        NoSuchField {
                            name: v.name,
                            owner,
                        },
                    )
                }
            }
            _ => {
                self.issue(
                    loc,
                    BadFieldAccess {
                        name: v.name,
                        owner,
                    },
                )
            }
        }
    }

    fn ret_ty_in_block(&mut self, b: &'a Block<'a>, ty_list: &mut Vec<&'a Ty<'a>>) {
        self.scoped(ScopeOwner::Local(b), |s| {
            for st in &b.stmt {
                s.ret_ty_in_stmt(st, ty_list);
            }
        });
    }
    fn ret_ty_in_stmt(&mut self, s: &'a Stmt<'a>, ty_list: &mut Vec<&'a Ty<'a>>) {
        match &s.kind {
            StmtKind::Return(r) => {
                let actual_ret = r.as_ref().map(|e| self.expr(e)).unwrap_or(Ty::void());
                let actual_ret = self.alloc.ty.alloc(actual_ret);
                ty_list.push(actual_ret);
            }
            StmtKind::If(i) => {
                //TODO  cond?
                //self.get_all_return_ty(&i.cond, ty_list)
                self.ret_ty_in_block(&i.on_true, ty_list);
                if let Some(b_false) = &i.on_false {
                    self.ret_ty_in_block(b_false, ty_list);
                }
            },
            StmtKind::While(w) => self.ret_ty_in_block(&w.body, ty_list),
            StmtKind::For(f) => self.ret_ty_in_block(&f.body, ty_list),
            StmtKind::Block(b) => self.ret_ty_in_block(&b, ty_list),
            _ => {},
        };
    }

    fn get_upper_ty(&mut self, ty_list: &[&'a Ty<'a>], loc: common::Loc) -> &'a Ty<'a> {
        let ret_ty = self.alloc.ty.alloc(Ty::null());
        for (idx, t) in ty_list.iter().enumerate() {
            if let TyKind::Null = t.kind { continue; }
            else {
                match t.kind {
                    TyKind::Int | TyKind::Bool | TyKind::String | TyKind::Void => {
                        for other_t in ty_list {
                            if Ref(other_t) != Ref(t) {
                                let _: u32 = self.issue(loc, IncompatibleReturnType);
                                break;
                            }
                        }
                        *ret_ty = **t;
                        return ret_ty;
                    },
                    _ if t.arr > 0 => {
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
                    e => {
                        println!("{:?}", t);
                        unreachable!();
                    }
                };
            }
        }
        //assert!(*ret_ty != Ty::null(), "default ret_ty must be modified");
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

impl<'a> TypePass<'a> {
    fn check_bool(&mut self, e: &'a Expr<'a>) {
        let ty = self.expr(e);
        if ty != Ty::bool() {
            ty.error_or(|| self.issue(e.loc, TestNotBool))
        }
    }

    fn check_arg_param(
        &mut self,
        arg: &'a [Expr<'a>],
        ret_param: &[Ty<'a>],
        name: &'a str,
        loc: Loc,
    ) -> Ty<'a> {
        let (ret, param) = (ret_param[0], &ret_param[1..]);
        if param.len() != arg.len() {
            self.issue(
                loc,
                ArgcMismatch {
                    name,
                    expect: param.len() as u32,
                    actual: arg.len() as u32,
                },
            )
        }
        for (idx, arg0) in arg.iter().enumerate() {
            let arg = self.expr(arg0);
            if let Some(&param) = param.get(idx) {
                if !arg.assignable_to(param) {
                    self.issue(
                        arg0.loc,
                        ArgMismatch {
                            loc: idx as u32 + 1,
                            arg,
                            param,
                        },
                    )
                }
            }
        }
        ret
    }
}
