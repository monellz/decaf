mod info;

use crate::info::*;
use ::tac::{
    self,
    Intrinsic::*,
    Operand::*,
    Tac::{self, *},
    *,
};
use common::{BinOp::*, HashMap, IndexMap, IndexSet, Ref, UnOp::*, MAIN_METHOD};
use syntax::{ast::*, ty::*, ScopeOwner};
use typed_arena::Arena;

#[derive(Default)]
struct TacGen<'a> {
    func: std::vec::Vec<TacFunc<'a>>,
    alloc: Option<&'a Arena<TacNode<'a>>>,

    func_idx: u32,

    cur_lambda: Option<&'a LambdaDef<'a>>,

    // `reg_num` and `label_num` are manually set at the beginning of every function
    reg_num: u32,
    label_num: u32,
    loop_stk: Vec<u32>,
    // Id & Index will behave differently when they are the lhs of an assignment
    // cur_assign contains the current assign rhs operand, or None if the current handling expr doesn't involve in assign
    cur_assign: Option<Operand>,
    str_pool: IndexSet<&'a str>,
    // `*_info` just works like extra fields to those structs, their specific meaning can be found at `struct *Info`
    var_info: HashMap<Ref<'a, VarDef<'a>>, VarInfo>,
    func_info: HashMap<Ref<'a, FuncDef<'a>>, FuncInfo>,
    class_info: HashMap<Ref<'a, ClassDef<'a>>, ClassInfo<'a>>,
}

pub fn work<'a>(p: &'a Program<'a>, alloc: &'a Arena<TacNode<'a>>) -> TacProgram<'a> {
    TacGen::default().program(p, alloc)
}

impl<'a> TacGen<'a> {
    fn program(mut self, p: &Program<'a>, alloc: &'a Arena<TacNode<'a>>) -> TacProgram<'a> {
        self.alloc = Some(alloc);
        let mut tp = TacProgram::default();
        for (idx, &c) in p.class.iter().enumerate() {
            self.define_str(c.name);
            self.resolve_field(c);
            self.class_info.get_mut(&Ref(c)).unwrap().idx = idx as u32;
            tp.func.push(self.build_new(c, alloc));
            //let t = self.build_new(c, alloc);
            //self.func.push(t);
        }
        {
            //let mut idx = tp.func.len() as u32; // their are already some `_Xxx._new` functions in tp.func, so can't start from 0
            //let mut idx = self.func.len() as u32; // their are already some `_Xxx._new` functions in tp.func, so can't start from 0
            self.func_idx = tp.func.len() as u32; // their are already some `_Xxx._new` functions in tp.func, so can't start from 0

            for &c in &p.class {
                for &f in &c.field {
                    if let FieldDef::FuncDef(f) = f {
                        //self.func_info.get_mut(&Ref(f)).unwrap().idx = idx;
                        self.func_info.get_mut(&Ref(f)).unwrap().idx = self.func_idx;
                        //idx += 1;
                        self.func_idx += 1;
                    }
                }
            }
        }
        for &c in &p.class {
            for f in &c.field {
                if let FieldDef::FuncDef(fu) = f {
                    let this = if fu.static_ { 0 } else { 1 };
                    for (idx, p) in fu.param.iter().enumerate() {
                        self.var_info.insert(
                            Ref(p),
                            VarInfo {
                                off: idx as u32 + this,
                            },
                        );
                    }
                    // these regs are occupied by parameters
                    self.reg_num = fu.param.len() as u32 + this;
                    self.label_num = 0;
                    let name = if Ref(c) == Ref(p.main.get().unwrap()) && fu.name == MAIN_METHOD {
                        MAIN_METHOD.into()
                    } else {
                        format!("_{}.{}", c.name, fu.name)
                    };
                    let mut f = TacFunc::empty(alloc, name, self.reg_num);

                    //for abstract func
                    if !fu.abstract_ {
                        self.block(&fu.body.as_ref().expect("in tacgen/lib.s"), &mut f);
                    }
                    f.reg_num = self.reg_num;
                    // add an return at the end of return-void function
                    if fu.ret_ty() == Ty::void() {
                        f.push(Tac::Ret { src: None });
                    }
                    tp.func.push(f);
                    //self.func.push(f);
                }
            }
        }
        for &c in &p.class {
            tp.vtbl.push(tac::VTbl {
                parent: c.parent_ref.get().map(|p| self.class_info[&Ref(p)].idx),
                class: c.name,
                func: self.class_info[&Ref(c)]
                    .vtbl
                    .iter()
                    .map(|(_, &f)| self.func_info[&Ref(f)].idx)
                    .collect(),
            });
        }
        tp.str_pool = self.str_pool;
        tp.func.append(&mut self.func);
        self.alloc = None;
        tp
    }

    fn block(&mut self, b: &'a Block<'a>, f: &mut TacFunc<'a>) {
        for s in &b.stmt {
            self.stmt(s, f);
        }
    }

    fn stmt(&mut self, s: &'a Stmt<'a>, f: &mut TacFunc<'a>) {
        use StmtKind::*;
        match &s.kind {
            Assign(a) => {
                self.cur_assign = Some(self.expr(&a.src, f));
                self.expr(&a.dst, f);
            }
            LocalVarDef(v) => {
                let reg = self.reg();
                self.var_info.insert(Ref(v), VarInfo { off: reg });
                let init = v
                    .init
                    .as_ref()
                    .map(|(_, e)| self.expr(e, f))
                    .unwrap_or(Const(0));
                f.push(Tac::Assign {
                    dst: reg,
                    src: [init],
                });
            }
            ExprEval(e) => {
                self.expr(e, f);
            }
            Skip(_) => {}
            If(i) => {
                let before_else = self.label();
                let cond = self.expr(&i.cond, f);
                f.push(Jif {
                    label: before_else,
                    z: true,
                    cond: [cond],
                });
                self.block(&i.on_true, f);
                if let Some(of) = &i.on_false {
                    let after_else = self.label();
                    f.push(Jmp { label: after_else });
                    f.push(Label { label: before_else });
                    self.block(of, f);
                    f.push(Label { label: after_else });
                } else {
                    f.push(Label { label: before_else });
                }
            }
            While(w) => {
                //   jump before_cond
                // before_body:
                //   body
                // before_cond:
                //   compute cond
                //   if cond jump before_body
                // after_body: (for break's use)
                let (before_cond, before_body, after_body) =
                    (self.label(), self.label(), self.label());
                self.loop_stk.push(after_body);
                f.push(Jmp { label: before_cond });
                f.push(Label { label: before_body });
                self.block(&w.body, f);
                f.push(Label { label: before_cond });
                let cond = self.expr(&w.cond, f);
                f.push(Jif {
                    label: before_body,
                    z: false,
                    cond: [cond],
                });
                f.push(Label { label: after_body });
                self.loop_stk.pop();
            }
            For(fo) => {
                // init
                //   jump before_cond
                // before_body:
                //   body
                //   update
                // before_cond:
                //   compute cond
                //   if cond jump before_body
                // after_body: (for break's use)
                let (before_cond, before_body, after_body) =
                    (self.label(), self.label(), self.label());
                self.loop_stk.push(after_body);
                self.stmt(&fo.init, f);
                f.push(Jmp { label: before_cond });
                f.push(Label { label: before_body });
                self.block(&fo.body, f);
                self.stmt(&fo.update, f);
                f.push(Label { label: before_cond });
                let cond = self.expr(&fo.cond, f);
                f.push(Jif {
                    label: before_body,
                    z: false,
                    cond: [cond],
                });
                f.push(Label { label: after_body });
                self.loop_stk.pop();
            }
            Return(r) => {
                let src = r.as_ref().map(|e| [self.expr(e, f)]);
                f.push(Ret { src });
            }
            Print(p) => {
                for e in p {
                    let reg = self.expr(e, f);
                    f.push(Param { src: [reg] });
                    match e.ty.get() {
                        t if t == Ty::int() => { self.intrinsic(_PrintInt, f); }
                        t if t == Ty::bool() => { self.intrinsic(_PrintBool, f); }
                        t if t == Ty::string() => { self.intrinsic(_PrintString, f); }
                        t => unreachable!("Shouldn't meet type {:?} in Print in these phase, type checking should have reported error.", t),
                    }
                }
            }
            Break(_) => {
                f.push(Jmp {
                    label: *self.loop_stk.last().unwrap(),
                });
            }
            Block(b) => self.block(b, f),
        }
    }

    fn expr(&mut self, e: &'a Expr<'a>, f: &mut TacFunc<'a>) -> Operand {
        use ExprKind::*;
        let assign = self.cur_assign.take();
        match &e.kind {
            Lambda(lam) => {
                //generate lambda func
                let prev_lambda = self.cur_lambda;
                self.cur_lambda = Some(lam);
                let prev_var_info = self.var_info.clone();
                let prev_reg_num = self.reg_num;
                let prev_label_num = self.label_num;
                self.label_num = 0;
                self.reg_num = lam.param.len() as u32 + 1;
                for (idx, p) in lam.param.iter().enumerate() {
                    self.var_info.insert(
                        Ref(p),
                        VarInfo {
                            off: idx as u32 + 1,
                        },
                    );
                }
                let name = format!("_lambda_{}_{}", lam.loc.0, lam.loc.1);
                let mut tmp_f = TacFunc::empty(self.alloc.unwrap(), name, (lam.param.len() + 1) as u32);
                tmp_f.reg_num = self.reg_num;

                for i in 0..lam.captured_var.borrow().len() {
                    let dst = self.reg();
                    tmp_f.push(Load {
                        dst,
                        base: [Reg(0)],
                        off: INT_SIZE * (i + 2) as i32,
                        hint: MemHint::Immutable,
                    });
                    self.var_info.insert(
                        lam.captured_var.borrow()[i],
                        VarInfo {
                            off: dst,
                        }
                    );
                }

                match &lam.kind {
                    LambdaKind::Block(lam_b) => {
                        self.block(&lam_b, &mut tmp_f);
                        if lam.ret_ty() == Ty::void() {
                            tmp_f.push(Tac::Ret {
                                src: None,
                            });
                        }
                    }
                    LambdaKind::Expr(lam_e) => {
                        let ret_reg = self.expr(lam_e, &mut tmp_f);
                        tmp_f.push(Tac::Ret {
                            src: Some([ret_reg]),
                        });
                    },
                };
                self.func.push(tmp_f);
                self.cur_lambda = prev_lambda;
                self.var_info = prev_var_info;
                self.reg_num = prev_reg_num;
                self.label_num = prev_label_num;


                let lambda_ptr = self.reg();
                let lambda_idx = self.func_idx + self.func.len() as u32 - 1;

                let capture_num = lam.captured_var.borrow().len() as i32 * 4 + 8;
                let addr = self.reg();
                f.push(Param {
                    src: [Const(capture_num)],
                }).push(Tac::Call {
                    dst: Some(addr),
                    kind: CallKind::Intrinsic(Intrinsic::_Alloc),
                }).push(Tac::LoadFunc {
                    dst: lambda_ptr,
                    f: lambda_idx as u32,
                }).push(Tac::Store {
                    src_base: [Reg(lambda_ptr), Reg(addr)],
                    off: 0,
                    hint: MemHint::Immutable,
                });
                if let Some(_) = self.cur_lambda {
                    let r = self.reg();
                    f.push(Tac::Load {
                        dst: r,
                        base: [Reg(0)],
                        off: INT_SIZE,
                        hint: MemHint::Immutable,
                    }).push(Tac::Store {
                        src_base: [Reg(r), Reg(addr)],
                        off: INT_SIZE,
                        hint: MemHint::Immutable,
                    });
                }  else {
                    f.push(Tac::Store {
                        src_base: [Reg(0), Reg(addr)],
                        off: INT_SIZE,
                        hint: MemHint::Immutable,
                    });
                }
                
                lam.captured_var.borrow().iter().enumerate().for_each(|(i, var)| {
                    f.push(Tac::Store {
                        src_base: [Reg(self.var_info[&Ref(var)].off), Reg(addr)],
                        off: ((i + 2) as i32 * INT_SIZE) as i32,
                        hint: MemHint::Immutable,
                    });
                });

                Reg(addr)
            }
            VarSel(v) => {
                match v.var.get() {
                    VarSelContent::Var(var) => {
                        /*
                        let mut captured = false;
                        let captured_reg = self.reg();
                        if let Some(lam) = self.cur_lambda {
                            for i in 0..lam.captured_var.borrow().len() {
                                if lam.captured_var.borrow()[i] == Ref(var) {
                                    f.push(Load {
                                        dst: captured_reg,
                                        base: [Reg(0)],
                                        off: INT_SIZE * (i + 2) as i32,
                                        hint: MemHint::Immutable,
                                    });
                                    captured = true;
                                    break;
                                }
                            }
                        }
                        if captured { return Reg(captured_reg); }
                        */

                        let off = self.var_info[&Ref(var)].off; // may be register id or offset in class
                        match var.owner.get().unwrap() {
                            ScopeOwner::Local(_)
                            | ScopeOwner::Param(_)
                            | ScopeOwner::LambdaParam(_)
                            | ScopeOwner::LambdaExprLocal(_) => {
                                if let Some(src) = assign {
                                    // `off` is register
                                    f.push(Tac::Assign {
                                        dst: off,
                                        src: [src],
                                    });
                                    // the return value won't be used, so just return a meaningless Reg(0), the below Reg(0)s are the same
                                    Reg(0)
                                } else {
                                    Reg(off)
                                }
                            }
                            ScopeOwner::Class(_) => {
                                // `off` is offset
                                // `this` is at argument 0
                                let owner = v.owner.as_ref().map(|o| self.expr(o, f));
                                let owner = if let None = owner {
                                    if let Some(_) = self.cur_lambda {
                                        //println!("cur_lambda is {:?}", lam.loc);
                                        let r = self.reg();
                                        f.push(Load {
                                            dst: r,
                                            base: [Reg(0)],
                                            off: INT_SIZE,
                                            hint: MemHint::Immutable,
                                        });
                                        Reg(r)
                                    } else {
                                        Reg(0)
                                    }
                                    //Reg(0)
                                } else {
                                    owner.unwrap()
                                };
                                if let Some(src) = assign {
                                    f.push(Store {
                                        src_base: [src, owner],
                                        off: off as i32 * INT_SIZE,
                                        hint: MemHint::Obj,
                                    });
                                    Reg(0)
                                } else {
                                    let dst = self.reg();
                                    f.push(Load {
                                        dst,
                                        base: [owner],
                                        off: off as i32 * INT_SIZE,
                                        hint: MemHint::Obj,
                                    });
                                    Reg(dst)
                                }
                            }
                            ScopeOwner::Global(_) => {
                                unreachable!("Impossible to declare a variable in global scope.")
                            }
                        }
                    }
                    VarSelContent::Func(var_f) => {
                        match &v.owner {
                            Some(o) if o.ty.get().is_arr() => {
                                let arr = self.expr(o, f);
                                self.length(arr, f)
                            }
                            _ => {
                                //let prev_reg_num = self.reg_num;
                                //let prev_label_num = self.label_num;
                                //self.label_num = 0;


                                let this = if var_f.static_ { 0 } else { 1 };
                                let reg = self.reg();
                                let hint = CallHint {
                                    arg_obj: var_f.param.iter().any(|a| a.ty.get().is_class()) || !var_f.static_,
                                    arg_arr: var_f.param.iter().any(|a| a.ty.get().arr > 0),
                                };

                                //self.reg_num = var_f.param.len() as u32 + this;
                                let name = format!("_{}_{}_{}", var_f.name, e.loc.0, e.loc.1);
                                let mut tmp_f = TacFunc::empty(self.alloc.unwrap(), name, (var_f.param.len() + this as usize) as u32);
                                //tmp_f.reg_num = self.reg_num;
                                let tmp_ret = if var_f.ret_ty() != Ty::void() {
                                    Some(self.reg())
                                } else {
                                    None
                                };
                                let ret = if var_f.static_ {
                                    for i in 0..var_f.param.len() {
                                        tmp_f.push(Param { src: [Reg((i + 1) as u32)] });
                                    }
                                    tmp_f.push(Tac::Call {
                                        dst: tmp_ret,
                                        kind: CallKind::Static(self.func_info[&Ref(var_f)].idx, hint),
                                    }).push(
                                        if var_f.ret_ty() != Ty::void() {
                                            Tac::Ret { src: Some([Reg(tmp_ret.unwrap())])}
                                        } else {
                                            Tac::Ret { src: None }
                                        }
                                    );
                                    self.func.push(tmp_f);
                                    //self.reg_num = prev_reg_num;
                                    //self.label_num = prev_label_num;

                                    let arr = self.intrinsic(
                                        _Alloc,
                                        f.push(Param { src: [Const(INT_SIZE)] }),
                                    ).unwrap();
                                    f.push(LoadFunc {
                                        dst: reg,
                                        f: self.func_idx + self.func.len() as u32 - 1,
                                    }).push(Store {
                                        src_base: [Reg(reg), Reg(arr)],
                                        off: 0,
                                        hint: MemHint::Immutable,//TODO??h
                                    });
                                    Reg(arr)
                                } else {
                                    let owner = v.owner.as_ref().map(|o| self.expr(o, f)).unwrap_or(Reg(0));
                                    let class_ptr_reg = self.reg();
                                    tmp_f.push(Load {
                                        dst: class_ptr_reg,
                                        base: [Reg(0)],
                                        off: INT_SIZE,
                                        hint: MemHint::Immutable,
                                    }).push(Param { src: [Reg(class_ptr_reg)] });
                                    for i in 0..var_f.param.len() {
                                        tmp_f.push(Param { src: [Reg((i + 1) as u32)] });
                                    }
                                    let slot = self.reg();
                                    let off = self.func_info[&Ref(var_f)].off;
                                    tmp_f.push(Load {
                                        dst: slot,
                                        base: [Reg(class_ptr_reg)],
                                        off: 0,
                                        hint: MemHint::Immutable,
                                    }).push(Load {
                                        dst: slot,
                                        base: [Reg(slot)],
                                        off: off as i32 * INT_SIZE,
                                        hint: MemHint::Immutable,
                                    }).push(Tac::Call {
                                        dst: tmp_ret,
                                        kind: CallKind::Virtual([Reg(slot)], hint),
                                    }).push(
                                        if var_f.ret_ty() != Ty::void() {
                                            Tac::Ret { src: Some([Reg(tmp_ret.unwrap())])}
                                        } else {
                                            Tac::Ret { src: None }
                                        }
                                    );
                                    self.func.push(tmp_f);
                                    //self.reg_num = prev_reg_num;
                                    //self.label_num = prev_label_num;


                                    let arr = self.intrinsic(
                                        _Alloc,
                                        f.push(Param { src: [Const(INT_SIZE + INT_SIZE)] }),
                                    ).unwrap();
                                    f.push(LoadFunc {
                                        dst: reg,
                                        f: self.func_idx + self.func.len() as u32 - 1,
                                    }).push(Store { //store the func obj
                                        src_base: [Reg(reg), Reg(arr)],
                                        off: 0,
                                        hint: MemHint::Immutable, //TODO????
                                    }).push(Store {
                                        src_base: [owner, Reg(arr)],
                                        off: INT_SIZE,
                                        hint: MemHint::Obj,
                                    });

                                    Reg(arr)
                                };
                                //unimplemented!();
                                ret
                            }
                        }
                    }
                    /*
                        let owner = v.owner.as_ref().map(|o| self.expr(o, f)).unwrap_or(Reg(0));
                        let reg = self.reg();
                        let hint = CallHint {
                            arg_obj: var_f.param.iter().any(|a| a.ty.get().is_class()) || !var_f.static_,
                            arg_arr: var_f.param.iter().any(|a| a.ty.get().arr > 0),
                        };
                        let name = format!("_{}_{}_{}", var_f.name, e.loc.0, e.loc.1);
                        let mut tmp_f = TacFunc::empty(self.alloc.unwrap(), name, var_f.param.len() as u32);
                        let tmp_ret = if var_f.ret_ty() != Ty::void() {
                            Some(self.reg())
                        } else {
                            None
                        };
                        if var_f.static_ {
                            for i in 0..var_f.param.len() {
                                tmp_f.push(Param { src: [Reg((i + 1) as u32)] });
                            }
                            println!("func name = {:?}, idx = {:?}", var_f.name, self.func_info[&Ref(var_f)].idx);
                            tmp_f.push(Tac::Call {
                                dst: tmp_ret,
                                kind: CallKind::Static(self.func_info[&Ref(var_f)].idx, hint),
                            }).push(
                                if var_f.ret_ty() != Ty::void() {
                                    Tac::Ret { src: Some([Reg(tmp_ret.unwrap())])}
                                } else {
                                    Tac::Ret { src: None }
                                }
                            );
                            self.func.push(tmp_f);

                            let arr = self.intrinsic(
                                _Alloc,
                                f.push(Param { src: [Const(INT_SIZE)] }),
                            ).unwrap();
                            f.push(LoadFunc {
                                dst: reg,
                                f: self.func_idx + self.func.len() as u32 - 1,
                            }).push(Store {
                                src_base: [Reg(reg), Reg(arr)],
                                off: 0,
                                hint: MemHint::Immutable,//TODO??h
                            });
                            Reg(arr)
                        } else {
                            let class_ptr_reg = self.reg();
                            println!("in varsel var_f.name = {:?} class_ptr_reg = {} e.loc = {:?}", var_f.name, class_ptr_reg, e.loc);
                            tmp_f.push(Load {
                                dst: class_ptr_reg,
                                base: [Reg(0)],
                                off: INT_SIZE,
                                hint: MemHint::Immutable,
                            }).push(Param { src: [Reg(class_ptr_reg)] });
                            for i in 0..var_f.param.len() {
                                tmp_f.push(Param { src: [Reg((i + 1) as u32)] });
                            }
                            let slot = self.reg();
                            let off = self.func_info[&Ref(var_f)].off;
                            tmp_f.push(Load {
                                dst: slot,
                                base: [Reg(class_ptr_reg)],
                                off: 0,
                                hint: MemHint::Immutable,
                            }).push(Load {
                                dst: slot,
                                base: [Reg(slot)],
                                off: off as i32 * INT_SIZE,
                                hint: MemHint::Immutable,
                            }).push(Tac::Call {
                                dst: tmp_ret,
                                kind: CallKind::Virtual([Reg(slot)], hint),
                            }).push(
                                if var_f.ret_ty() != Ty::void() {
                                    Tac::Ret { src: Some([Reg(tmp_ret.unwrap())])}
                                } else {
                                    Tac::Ret { src: None }
                                }
                            );
                            self.func.push(tmp_f);

                            let arr = self.intrinsic(
                                _Alloc,
                                f.push(Param { src: [Const(INT_SIZE + INT_SIZE)] }),
                            ).unwrap();
                            f.push(LoadFunc {
                                dst: reg,
                                f: self.func_idx + self.func.len() as u32 - 1,
                            }).push(Store { //store the func obj
                                src_base: [Reg(reg), Reg(arr)],
                                off: 0,
                                hint: MemHint::Immutable, //TODO????
                            }).push(Store {
                                src_base: [owner, Reg(arr)],
                                off: INT_SIZE,
                                hint: MemHint::Obj,
                            });

                            Reg(arr)
                        }
                        //unimplemented!();
                    }
                    */
                    VarSelContent::Empty => {
                        unreachable!()
                    }
                }
                /*
                let var = match v.var.get() {
                    VarSelContent::Var(v) => v,
                    VarSelContent::Func(v_f) => {
                        println!("v_f name = {:?}", v.name);
                        unimplemented!();
                    },
                    VarSelContent::Empty => unreachable!(),
                };
                let off = self.var_info[&Ref(var)].off; // may be register id or offset in class
                match var.owner.get().unwrap() {
                    ScopeOwner::Local(_)
                    | ScopeOwner::Param(_)
                    | ScopeOwner::LambdaParam(_)
                    | ScopeOwner::LambdaExprLocal(_) => {
                        if let Some(src) = assign {
                            // `off` is register
                            f.push(Tac::Assign {
                                dst: off,
                                src: [src],
                            });
                            // the return value won't be used, so just return a meaningless Reg(0), the below Reg(0)s are the same
                            Reg(0)
                        } else {
                            Reg(off)
                        }
                    }
                    ScopeOwner::Class(_) => {
                        // `off` is offset
                        // `this` is at argument 0
                        let owner = v.owner.as_ref().map(|o| self.expr(o, f)).unwrap_or(Reg(0));
                        if let Some(src) = assign {
                            f.push(Store {
                                src_base: [src, owner],
                                off: off as i32 * INT_SIZE,
                                hint: MemHint::Obj,
                            });
                            Reg(0)
                        } else {
                            let dst = self.reg();
                            f.push(Load {
                                dst,
                                base: [owner],
                                off: off as i32 * INT_SIZE,
                                hint: MemHint::Obj,
                            });
                            Reg(dst)
                        }
                    }
                    ScopeOwner::Global(_) => {
                        unreachable!("Impossible to declare a variable in global scope.")
                    }
                }
                */
            }
            IndexSel(i) => {
                let (arr, idx) = (self.expr(&i.arr, f), self.expr(&i.idx, f));
                let (ok, len, cmp) = (self.reg(), self.length(arr, f), self.reg());
                let (err, after) = (self.label(), self.label());
                f.push(Bin {
                    op: Ge,
                    dst: ok,
                    lr: [idx, Const(0)],
                })
                .push(Bin {
                    op: Lt,
                    dst: cmp,
                    lr: [idx, len],
                })
                .push(Bin {
                    op: And,
                    dst: ok,
                    lr: [Reg(ok), Reg(cmp)],
                })
                .push(Jif {
                    label: err,
                    z: true,
                    cond: [Reg(ok)],
                });
                // range check passed if reach here
                let off = self.reg();
                f.push(Bin {
                    op: Mul,
                    dst: off,
                    lr: [idx, Const(INT_SIZE)],
                })
                .push(Bin {
                    op: Add,
                    dst: off,
                    lr: [Reg(off), arr],
                });
                let ret = if let Some(src) = assign {
                    f.push(Store {
                        src_base: [src, Reg(off)],
                        off: 0,
                        hint: MemHint::Arr,
                    });
                    Reg(0)
                } else {
                    let dst = self.reg();
                    f.push(Load {
                        dst,
                        base: [Reg(off)],
                        off: 0,
                        hint: MemHint::Arr,
                    });
                    Reg(dst)
                };
                f.push(Jmp { label: after });
                self.re(INDEX_OUT_OF_BOUND, f.push(Label { label: err }));
                f.push(Label { label: after });
                ret
            }
            IntLit(i) => Const(*i),
            BoolLit(b) => Const(*b as i32),
            StringLit(s) => {
                let dst = self.reg();
                f.push(LoadStr {
                    dst,
                    s: self.define_str(s),
                });
                Reg(dst)
            }
            NullLit(_) => Const(0),
            Call(c) => {
                //func obj
                let func_reg = self.expr(c.func.as_ref(), f);
                let func_ptr = self.reg();

                let args = c.arg.iter().map(|a| self.expr(a, f)).collect::<Vec<_>>();
                let hint = CallHint {
                    arg_obj: c.arg.iter().any(|a| a.ty.get().is_class()),
                    arg_arr: c.arg.iter().any(|a| a.ty.get().arr > 0),
                };

                let ret = if let TyKind::Func(ty) = c.func.as_ref().ty.get().kind {
                    if ty[0] != Ty::void() {
                        Some(self.reg())
                    } else {
                        None
                    }
                } else {
                    unreachable!("only func ty is callable")
                };
                

                f.push(Load {
                    dst: func_ptr,
                    base: [func_reg],
                    off: 0,
                    hint: MemHint::Obj, //TODO???
                }).push( Param {
                    src: [func_reg]
                });
                for a in args {
                    f.push(Param { src: [a] });
                }
                f.push(Tac::Call {
                    dst: ret,
                    kind: CallKind::Virtual([Reg(func_ptr)], hint),
                });
                Reg(ret.unwrap_or(0))


                /*
                if let ExprKind::VarSel(v) = &c.func.kind {
                    match &v.owner {
                        Some(o) if o.ty.get().is_arr() => {
                            let arr = self.expr(o, f);
                            self.length(arr, f)
                        }
                        _ => {
                            let fu = c.func_ref.get().expect("in Call");
                            println!("varsel v.name = {:?}, loc = {:?}, fu.name = {:?}", v.name, e.loc, fu.name);
                            let ret = if fu.ret_ty() != Ty::void() {
                                Some(self.reg())
                            } else {
                                None
                            };
                            let args = c.arg.iter().map(|a| self.expr(a, f)).collect::<Vec<_>>();
                            let hint = CallHint {
                                arg_obj: c.arg.iter().any(|a| a.ty.get().is_class()) || !fu.static_,
                                arg_arr: c.arg.iter().any(|a| a.ty.get().arr > 0),
                            };
                            if fu.static_ {
                                for a in args {
                                    f.push(Param { src: [a] });
                                }
                                println!("in call varsel fu.name = {:?}, idx = {:?}", fu.name, self.func_info[&Ref(fu)].idx);
                                f.push(Tac::Call {
                                    dst: ret,
                                    kind: CallKind::Static(self.func_info[&Ref(fu)].idx, hint),
                                });
                            } else {
                                // Reg(0) is `this`
                                let owner = v.owner.as_ref().map(|o| self.expr(o, f)).unwrap_or(Reg(0));
                                f.push(Param { src: [owner] });
                                for a in args {
                                    f.push(Param { src: [a] });
                                }
                                let slot = self.reg();
                                let off = self.func_info[&Ref(fu)].off;
                                f.push(Load {
                                    dst: slot,
                                    base: [owner],
                                    off: 0,
                                    hint: MemHint::Immutable,
                                })
                                .push(Load {
                                    dst: slot,
                                    base: [Reg(slot)],
                                    off: off as i32 * INT_SIZE,
                                    hint: MemHint::Immutable,
                                });
                                f.push(Tac::Call {
                                    dst: ret,
                                    kind: CallKind::Virtual([Reg(slot)], hint),
                                });
                            }
                            Reg(ret.unwrap_or(0)) // if ret is None, the result can't be assigned to others, so 0 will not be used
                        }
                    }
                } else {
                    //func obj
                    let func_reg = self.expr(c.func.as_ref(), f);
                    let func_ptr = self.reg();

                    let args = c.arg.iter().map(|a| self.expr(a, f)).collect::<Vec<_>>();
                    let hint = CallHint {
                        arg_obj: c.arg.iter().any(|a| a.ty.get().is_class()),
                        arg_arr: c.arg.iter().any(|a| a.ty.get().arr > 0),
                    };

                    let ret = if let TyKind::Func(ty) = c.func.as_ref().ty.get().kind {
                        if ty[0] != Ty::void() {
                            Some(self.reg())
                        } else {
                            None
                        }
                    } else {
                        unreachable!("only func ty is callable")
                    };
                    

                    f.push(Load {
                        dst: func_ptr,
                        base: [func_reg],
                        off: 0,
                        hint: MemHint::Obj, //TODO???
                    }).push( Param {
                        src: [func_reg]
                    });
                    for a in args {
                        f.push(Param { src: [a] });
                    }
                    f.push(Tac::Call {
                        dst: ret,
                        kind: CallKind::Virtual([Reg(func_ptr)], hint),
                    });
                    println!("func {:?} done, idx = {}, {}", e.loc, self.func_idx, self.func.len());
                    Reg(ret.unwrap_or(0))
                }
                */
                

                /* //original
                let v = if let ExprKind::VarSel(v) = &c.func.kind {
                    v
                } else {
                    println!("func kind is {:?} at {:?}", c.func.as_ref().ty, c.func.as_ref().loc);
                    unimplemented!()
                };
                match &v.owner {
                    Some(o) if o.ty.get().is_arr() => {
                        let arr = self.expr(o, f);
                        self.length(arr, f)
                    }
                    _ => {
                        let fu = c.func_ref.get().expect("in Call");
                        let ret = if fu.ret_ty() != Ty::void() {
                            Some(self.reg())
                        } else {
                            None
                        };
                        let args = c.arg.iter().map(|a| self.expr(a, f)).collect::<Vec<_>>();
                        let hint = CallHint {
                            arg_obj: c.arg.iter().any(|a| a.ty.get().is_class()) || !fu.static_,
                            arg_arr: c.arg.iter().any(|a| a.ty.get().arr > 0),
                        };
                        if fu.static_ {
                            for a in args {
                                f.push(Param { src: [a] });
                            }
                            f.push(Tac::Call {
                                dst: ret,
                                kind: CallKind::Static(self.func_info[&Ref(fu)].idx, hint),
                            });
                        } else {
                            // Reg(0) is `this`
                            let owner = v.owner.as_ref().map(|o| self.expr(o, f)).unwrap_or(Reg(0));
                            f.push(Param { src: [owner] });
                            for a in args {
                                f.push(Param { src: [a] });
                            }
                            let slot = self.reg();
                            let off = self.func_info[&Ref(fu)].off;
                            f.push(Load {
                                dst: slot,
                                base: [owner],
                                off: 0,
                                hint: MemHint::Immutable,
                            })
                            .push(Load {
                                dst: slot,
                                base: [Reg(slot)],
                                off: off as i32 * INT_SIZE,
                                hint: MemHint::Immutable,
                            });
                            f.push(Tac::Call {
                                dst: ret,
                                kind: CallKind::Virtual([Reg(slot)], hint),
                            });
                        }
                        Reg(ret.unwrap_or(0)) // if ret is None, the result can't be assigned to others, so 0 will not be used
                    }
                }
                */
            }
            Unary(u) => {
                let (r, dst) = (self.expr(&u.r, f), self.reg());
                f.push(Un {
                    op: u.op,
                    dst,
                    r: [r],
                });
                Reg(dst)
            }
            Binary(b) => {
                let (l, r) = (self.expr(&b.l, f), self.expr(&b.r, f));
                match b.op {
                    Eq | Ne if b.l.ty.get() == Ty::string() => {
                        f.push(Param { src: [l] }).push(Param { src: [r] });
                        let dst = self.intrinsic(_StringEqual, f).unwrap();
                        if b.op == Ne {
                            f.push(Un {
                                op: Not,
                                dst,
                                r: [Reg(dst)],
                            });
                        }
                        Reg(dst)
                    }
                    Div | Mod => {
                        //runtime error for division by zero error
                        let (dst, r_reg) = (self.reg(), self.reg());
                        let ok = self.reg();
                        let (err, after) = (self.label(), self.label()); 
                        f.push(Bin {
                            op: Ne,
                            dst: ok,
                            lr: [r, Const(0)],
                        })
                        .push(Jif {
                            label: err,
                            z: true,
                            cond: [Reg(ok)],
                        })
                        .push(Tac::Assign {
                            dst: r_reg,
                            src: [r],
                        })
                        .push(Bin {
                            op: b.op,
                            dst,
                            lr: [l, Reg(r_reg)],
                        });
                        f.push(Jmp { label: after });
                        self.re(DIVISION_BY_ZERO, f.push(Label { label: err }));
                        f.push(Label { label: after });
                        Reg(dst)
                    }
                    op => {
                        let dst = self.reg();
                        f.push(Bin {
                            op,
                            dst,
                            lr: [l, r],
                        });
                        Reg(dst)
                    }
                }
            }
            This(_) => {
                if let Some(_) = self.cur_lambda {
                    let dst = self.reg();
                    f.push(Load {
                        dst,
                        base: [Reg(0)],
                        off: INT_SIZE,
                        hint: MemHint::Immutable,
                    });
                    Reg(dst)
                } else {
                    Reg(0)
                }
            }
            ReadInt(_) => Reg(self.intrinsic(_ReadInt, f).unwrap()),
            ReadLine(_) => Reg(self.intrinsic(_ReadLine, f).unwrap()),
            NewClass(n) => {
                let dst = self.reg();
                // by design, a class's new func in functions have the same index as its vtbl in vtbls
                f.push(Tac::Call {
                    dst: Some(dst),
                    kind: CallKind::Static(
                        self.class_info[&Ref(n.class.get().unwrap())].idx,
                        CallHint {
                            arg_obj: false,
                            arg_arr: false,
                        },
                    ),
                });
                Reg(dst)
            }
            NewArray(n) => {
                let len = self.expr(&n.len, f);
                let (ok, before_cond, before_body) = (self.label(), self.label(), self.label());
                let (cmp, ptr) = (self.reg(), self.reg());
                f.push(Bin {
                    op: Lt,
                    dst: cmp,
                    lr: [len, Const(0)],
                })
                .push(Jif {
                    label: ok,
                    z: true,
                    cond: [Reg(cmp)],
                });
                self.re(NEW_ARR_NEG, f);
                f.push(Label { label: ok });
                let arr = self
                    .intrinsic(
                        _Alloc,
                        f.push(Bin {
                            op: Mul,
                            dst: ptr,
                            lr: [len, Const(INT_SIZE)],
                        })
                        .push(Bin {
                            op: Add,
                            dst: ptr,
                            lr: [Reg(ptr), Const(INT_SIZE)],
                        }) // now ptr = bytes to allocate
                        .push(Param { src: [Reg(ptr)] }),
                    )
                    .unwrap();
                f.push(Bin {
                    op: Add,
                    dst: ptr,
                    lr: [Reg(arr), Reg(ptr)],
                }); // now ptr = end of array
                f.push(Bin {
                    op: Add,
                    dst: arr,
                    lr: [Reg(arr), Const(INT_SIZE)],
                }); // now arr = begin of array([0])
                f.push(Jmp { label: before_cond }) // loop(reversely), set all to 0
                    .push(Label { label: before_body })
                    .push(Bin {
                        op: Sub,
                        dst: ptr,
                        lr: [Reg(ptr), Const(INT_SIZE)],
                    })
                    .push(Store {
                        src_base: [Const(0), Reg(ptr)],
                        off: 0,
                        hint: MemHint::Arr,
                    })
                    .push(Label { label: before_cond })
                    .push(Bin {
                        op: Eq,
                        dst: cmp,
                        lr: [Reg(ptr), Reg(arr)],
                    })
                    .push(Jif {
                        label: before_body,
                        z: true,
                        cond: [Reg(cmp)],
                    }); // when ptr == arr, loop end
                f.push(Store {
                    src_base: [len, Reg(arr)],
                    off: -INT_SIZE,
                    hint: MemHint::Immutable,
                }); // arr[-1] = len
                Reg(arr)
            }
            ClassTest(t) => {
                let obj = self.expr(&t.expr, f);
                self.check_cast(obj, self.class_info[&Ref(t.class.get().unwrap())].idx, f)
            }
            ClassCast(t) => {
                let obj = self.expr(&t.expr, f);
                let check =
                    self.check_cast(obj, self.class_info[&Ref(t.class.get().unwrap())].idx, f);
                let (msg, vtbl, ok) = (self.reg(), self.reg(), self.label());
                f.push(Jif {
                    label: ok,
                    z: false,
                    cond: [check],
                });
                let s = self.define_str(BAD_CAST1); // borrow checker...
                self.intrinsic(
                    _PrintString,
                    f.push(LoadStr { dst: msg, s })
                        .push(Param { src: [Reg(msg)] }),
                );
                self.intrinsic(
                    _PrintString,
                    f.push(Load {
                        dst: vtbl,
                        base: [obj],
                        off: 0,
                        hint: MemHint::Immutable,
                    })
                    .push(Load {
                        dst: msg,
                        base: [Reg(vtbl)],
                        off: INT_SIZE as i32,
                        hint: MemHint::Immutable,
                    })
                    .push(Param { src: [Reg(msg)] }),
                );
                let s = self.define_str(BAD_CAST2);
                self.intrinsic(
                    _PrintString,
                    f.push(LoadStr { dst: msg, s })
                        .push(Param { src: [Reg(msg)] }),
                );
                let s = self.define_str(t.name);
                self.intrinsic(
                    _PrintString,
                    f.push(LoadStr { dst: msg, s })
                        .push(Param { src: [Reg(msg)] }),
                );
                let s = self.define_str(BAD_CAST3);
                self.intrinsic(
                    _PrintString,
                    f.push(LoadStr { dst: msg, s })
                        .push(Param { src: [Reg(msg)] }),
                );
                self.intrinsic(_Halt, f);
                f.push(Label { label: ok });
                obj
            }
        }
    }
}

impl<'a> TacGen<'a> {
    // define a string in str pool and return its id, this id can be used in Tac::LoadStr
    fn define_str(&mut self, s: &'a str) -> u32 {
        self.str_pool.insert_full(s).0 as u32
    }

    fn reg(&mut self) -> u32 {
        (self.reg_num, self.reg_num += 1).0
    }

    fn label(&mut self) -> u32 {
        (self.label_num, self.label_num += 1).0
    }

    // if you don't need to modify the returned register, it is more recommended to use Const(i)
    fn int(&mut self, i: i32, f: &mut TacFunc<'a>) -> u32 {
        let dst = self.reg();
        f.push(Tac::Assign {
            dst,
            src: [Const(i)],
        });
        dst
    }

    // perform an intrinsic call, return value is Some if this intrinsic call has return value
    fn intrinsic(&mut self, i: Intrinsic, f: &mut TacFunc<'a>) -> Option<u32> {
        let dst = if i.has_ret() { Some(self.reg()) } else { None };
        f.push(Tac::Call {
            dst,
            kind: CallKind::Intrinsic(i),
        });
        dst
    }

    // read the length of `arr` (caller should guarantee `arr` is really an array)
    fn length(&mut self, arr: Operand, f: &mut TacFunc<'a>) -> Operand {
        let dst = self.reg();
        f.push(Load {
            dst,
            base: [arr],
            off: -(INT_SIZE as i32),
            hint: MemHint::Immutable,
        });
        Reg(dst)
    }

    // re is short for for runtime error; this function prints a message and call halt
    fn re(&mut self, msg: &'static str, f: &mut TacFunc<'a>) {
        let src = self.reg();
        let s = self.define_str(msg);
        self.intrinsic(
            _PrintString,
            f.push(LoadStr { dst: src, s })
                .push(Param { src: [Reg(src)] }),
        );
        self.intrinsic(_Halt, f);
    }

    fn check_cast(&mut self, obj: Operand, vtbl_idx: u32, f: &mut TacFunc<'a>) -> Operand {
        // ret = 0
        // while (cur)
        //   ret = (cur == target)
        //   if ret = 1
        //     break
        //   cur = cur->parent
        let (ret, cur, target) = (self.int(0, f), self.reg(), self.reg());
        let (before_cond, after_body) = (self.label(), self.label());
        f.push(LoadVTbl {
            dst: target,
            v: vtbl_idx,
        });
        f.push(Load {
            dst: cur,
            base: [obj],
            off: 0,
            hint: MemHint::Immutable,
        });
        f.push(Label { label: before_cond });
        f.push(Jif {
            label: after_body,
            z: true,
            cond: [Reg(cur)],
        });
        f.push(Bin {
            op: Eq,
            dst: ret,
            lr: [Reg(cur), Reg(target)],
        })
        .push(Jif {
            label: after_body,
            z: false,
            cond: [Reg(ret)],
        });
        f.push(Load {
            dst: cur,
            base: [Reg(cur)],
            off: 0,
            hint: MemHint::Immutable,
        });
        f.push(Jmp { label: before_cond });
        f.push(Label { label: after_body });
        Reg(ret)
    }
}

impl<'a> TacGen<'a> {
    // `idx` in ClassInfo & FuncInfo is not determined here, just set them to a meaningless value (0)
    // all functions (static & virtual) are inserted into self.func_info
    // this function relies on the fact that no cyclic inheritance exist, which is guaranteed in typeck
    fn resolve_field(&mut self, c: &'a ClassDef<'a>) {
        if !self.class_info.contains_key(&Ref(c)) {
            let (mut field_num, mut vtbl) = if let Some(p) = c.parent_ref.get() {
                self.resolve_field(p);
                let p = &self.class_info[&Ref(p)];
                (p.field_num, p.vtbl.clone())
            } else {
                (1, IndexMap::default())
            };
            for f in &c.field {
                match f {
                    FieldDef::FuncDef(f) => {
                        if !f.static_ {
                            if let Some((idx, _, p_f)) = vtbl.get_full_mut(f.name) {
                                // + 2, because 0 is parent vtbl, 1 is class name
                                self.func_info.insert(
                                    Ref(f),
                                    FuncInfo {
                                        off: idx as u32 + 2,
                                        idx: 0,
                                    },
                                );
                                *p_f = f; // override
                            } else {
                                self.func_info.insert(
                                    Ref(f),
                                    FuncInfo {
                                        off: vtbl.len() as u32 + 2,
                                        idx: 0,
                                    },
                                );
                                vtbl.insert(f.name, f);
                            }
                        } else {
                            // `off` is useless for static functions
                            self.func_info.insert(Ref(f), FuncInfo { off: 0, idx: 0 });
                        }
                    }
                    FieldDef::VarDef(v) => {
                        self.var_info.insert(Ref(v), VarInfo { off: field_num });
                        field_num += 1;
                    }
                }
            }
            self.class_info.insert(
                Ref(c),
                ClassInfo {
                    field_num,
                    idx: 0,
                    vtbl,
                },
            );
        }
    }

    fn build_new(&mut self, c: &'a ClassDef<'a>, alloc: &'a Arena<TacNode<'a>>) -> TacFunc<'a> {
        self.reg_num = 0;
        let ClassInfo { field_num, idx, .. } = self.class_info[&Ref(c)];
        let mut f = TacFunc::empty(alloc, format!("_{}._new", c.name), 0);
        f.push(Param {
            src: [Const(field_num as i32 * INT_SIZE)],
        });
        let ret = self.intrinsic(_Alloc, &mut f).unwrap();
        let vtbl = self.reg();
        f.push(LoadVTbl { dst: vtbl, v: idx });
        f.push(Store {
            src_base: [Reg(vtbl), Reg(ret)],
            off: 0,
            hint: MemHint::Immutable,
        });
        for i in 1..field_num {
            f.push(Store {
                src_base: [Const(0), Reg(ret)],
                off: i as i32 * INT_SIZE,
                hint: MemHint::Obj,
            });
        }
        f.push(Ret {
            src: Some([Reg(ret)]),
        });
        f.reg_num = self.reg_num;
        f
    }
}
