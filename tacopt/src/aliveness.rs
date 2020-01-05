use crate::{
    bb::{FuncBB, BB, NextKind},
    flow::{Or, Flow, FlowElem},
};
use bitset::traits::*;
use common::{HashMap, HashSet};
use tac::{Operand, Tac, CallKind};

pub fn work(f: &mut FuncBB) {
    let mut id2def = HashMap::new();
    let mut id2use = HashMap::new();

    let mut max_reg_num = 0;
    let max = |x: u32, y: u32| if x > y { x } else { y };
    for (i, b) in f.bb.iter().enumerate().rev() {
        //println!("b.i = {:?}, b.prev = {:?}", i, b.prev);
        let mut def_b = HashSet::new();
        let mut use_b = HashSet::new();
        //branch
        match b.next {
            NextKind::Jif {cond, ..} => {use_b.insert(cond);},
            NextKind::Ret(o) => {
                if let Some(src) = o {
                    if let Operand::Reg(r) = src {
                        use_b.insert(r);
                    }
                }
            },
            _ => {}
        };


        let use_insert = |x: &[Operand], set: &mut HashSet<_>, max_rec: &mut u32| {
            for o in x {
                if let Operand::Reg(src) = o {
                    set.insert(*src);
                    *max_rec = max(*max_rec, *src);
                }
            }
        };
        for t in b.iter().rev() {
            match t.tac.get() {
                Tac::Bin {dst, lr, ..} => {
                    def_b.insert(dst);
                    use_b.remove(&dst);
                    use_insert(&lr, &mut use_b, &mut max_reg_num);
                    max_reg_num = max(max_reg_num, dst);
                }
                Tac::Un {dst, r, ..} => {
                    def_b.insert(dst);
                    use_b.remove(&dst);
                    use_insert(&r, &mut use_b, &mut max_reg_num);
                    max_reg_num = max(max_reg_num, dst);
                }
                Tac::Assign {dst, src} => {
                    def_b.insert(dst);
                    use_b.remove(&dst);
                    use_insert(&src, &mut use_b, &mut max_reg_num);
                    max_reg_num = max(max_reg_num, dst);
                }
                Tac::Param {src} => {
                    use_insert(&src, &mut use_b, &mut max_reg_num);
                }
                Tac::Call {dst, kind} => {
                    if let Some(dst) = dst {
                        def_b.insert(dst);
                        use_b.remove(&dst);
                        max_reg_num = max(max_reg_num, dst);
                    }

                    if let CallKind::Virtual(o, _) = kind {
                        use_insert(&o, &mut use_b, &mut max_reg_num);
                    }
                }
                Tac::Load {dst, base, ..} => {
                    def_b.insert(dst);
                    use_b.remove(&dst);
                    use_insert(&base, &mut use_b, &mut max_reg_num);
                    max_reg_num = max(max_reg_num, dst);
                }
                Tac::LoadStr {dst, ..} => {
                    def_b.insert(dst);
                    use_b.remove(&dst);
                    max_reg_num = max(max_reg_num, dst);
                }
                Tac::LoadVTbl {dst, ..} => {
                    def_b.insert(dst);
                    use_b.remove(&dst);   
                    max_reg_num = max(max_reg_num, dst);
                }
                Tac::LoadFunc {dst, ..} => {
                    def_b.insert(dst);
                    use_b.remove(&dst);
                    max_reg_num = max(max_reg_num, dst);
                }
                Tac::Store {src_base, ..} => {
                    use_insert(&src_base, &mut use_b, &mut max_reg_num);
                }
                Tac::Ret {..} | Tac::Jmp {..} | Tac::Jif {..} | Tac::Label {..} => {
                    unreachable!("ret/jmp/jif/label should not appear in bb")
                }
            };
        }
        

        id2def.insert(i, def_b);
        id2use.insert(i, use_b);
    }

    let reg_len = (max_reg_num + 1) as usize;

    let mut alive_flow = Flow::<Or>::new(f.bb.len() + 1, reg_len);
    let each = alive_flow.each();
    let FlowElem { gen, kill, out, .. } = alive_flow.split();
    for idx in 0..f.bb.len() {
        let off = (idx + 1) * each;

        let def_b = iter2bs(id2def.get(&idx).unwrap(), reg_len);
        let use_b = iter2bs(id2use.get(&idx).unwrap(), reg_len);

        //println!("idx[{}]: def_b = {:?}", idx, id2def.get(&idx));
        //println!("idx[{}]: use_b = {:?}", idx, id2use.get(&idx));

        //gen <-> live_use
        //kill <-> def
        gen[off..off + each].bsor(&use_b);
        kill[off..off + each].bsor(&def_b);
    }

    for x in out.iter_mut() {
        *x = 0;
    }

    let mut f_next = Vec::new();
    f.bb.iter().for_each(|b| {
        f_next.push(b.next());
    });
    alive_flow.solve(
        f.bb.iter().enumerate().map(|b| {
            (b.0 + 1, f_next[b.0].iter()
                                 .filter_map(|&o| o)
                                 .map(|x| x as usize + 1)
                                 .chain(if b.0 == f.bb.len() - 1 { Some(0) } else { None }))
        })
    );
    let FlowElem { in_, out, .. } = alive_flow.split();
    for idx in 0..f.bb.len() {
        let off = (idx + 1) * each;
        let mut in_set = HashSet::new();
        let mut out_set = HashSet::new();
        for i in 0..reg_len {
            if in_[off..off + each].bsget(i) {
                in_set.insert(i);
            }
            if out[off..off + each].bsget(i) {
                out_set.insert(i);
            }
        }
        //println!("idx[{}]: in = {:?}, out = {:?}", idx, in_set, out_set);
    }

    //do_optimize
    for (off, b) in f.bb.iter_mut().enumerate().map(|b| ((b.0 + 1) * each, b.1)) {
        do_optimize(b, &mut in_[off..off + each]);
    }
}

fn do_optimize(b: &mut BB, live_out: &mut [u32]) {
    fn add(lr: &[Operand], out: &mut [u32]) {
        lr.iter().for_each(|o| {
            if let Operand::Reg(o) = o {
                out.bsset(*o);
            }
        });
    }
    //branch
    match b.next {
        NextKind::Jif {cond, ..} => {live_out.bsset(cond);},
        NextKind::Ret(o) => {
            if let Some(src) = o {
                if let Operand::Reg(r) = src {
                    live_out.bsset(r);
                }
            }
        },
        _ => {}
    };
    for t in b.iter().rev() {
        let tac = t.tac.get();
        match tac {
            Tac::Bin {dst, lr, ..} => {
                if !live_out.bsget(dst) {
                    b.del(t);
                } else {
                    live_out.bsdel(dst);
                    add(&lr, live_out);
                }
            }
            Tac::Un {dst, r, ..} => {
                if !live_out.bsget(dst) {
                    b.del(t);
                } else {
                    live_out.bsdel(dst);
                    add(&r, live_out);
                }
            }
            Tac::Assign {dst, src} => {
                if !live_out.bsget(dst) {
                    b.del(t);
                } else {
                    live_out.bsdel(dst);
                    add(&src, live_out);
                }
            }
            Tac::Call {dst, kind} => {
                if let Some(dst) = dst {
                    if !live_out.bsget(dst) {
                        t.tac.set(Tac::Call {dst: None, kind: kind});
                    } else {
                        live_out.bsdel(dst);
                    }
                }

                if let CallKind::Virtual(o, _) = kind {
                    add(&o, live_out);
                }
            }
            Tac::Load {dst, base, ..} => {
                if !live_out.bsget(dst) {
                    b.del(t);
                } else {
                    live_out.bsdel(dst);
                    add(&base, live_out);
                }
            }
            Tac::LoadStr {dst, ..} => {
                if !live_out.bsget(dst) {
                    b.del(t);
                } else {
                    live_out.bsdel(dst);
                }
            }
            Tac::LoadVTbl {dst, ..} => {
                if !live_out.bsget(dst) {
                    b.del(t);
                } else {
                    live_out.bsdel(dst);
                }
            }
            Tac::LoadFunc {dst, ..} => {
                if !live_out.bsget(dst) {
                    b.del(t);
                } else {
                    live_out.bsdel(dst);
                }
            }
            Tac::Param {src} => {
                add(&src, live_out);
            }
            Tac::Store {src_base, ..} => {
                add(&src_base, live_out);
            }
            Tac::Ret {..} | Tac::Jmp {..} | Tac::Jif {..} | Tac::Label {..} => {
                unreachable!();
            }
        }
    }
}
