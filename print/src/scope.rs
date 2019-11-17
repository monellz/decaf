use common::{IgnoreResult, IndentPrinter};
use std::fmt::Write;
use syntax::{ast::*, Scope};

fn show_scope(s: &Scope, p: &mut IndentPrinter) {
    let mut s = s.iter().map(|(_, &sym)| sym).collect::<Vec<_>>();
    s.sort_unstable_by_key(|x| x.loc());
    if s.is_empty() {
        write!(p, "<empty>").ignore();
    } else {
        for s in s {
            write!(p, "{:?}", s).ignore();
        }
    }
}

pub fn program(pr: &Program, p: &mut IndentPrinter) {
    write!(p, "GLOBAL SCOPE:").ignore();
    p.indent(|p| {
        show_scope(&pr.scope.borrow(), p);
        for c in &pr.class {
            class_def(c, p);
        }
    });
}

pub fn class_def(c: &ClassDef, p: &mut IndentPrinter) {
    write!(p, "CLASS SCOPE OF '{}':", c.name).ignore();
    p.indent(|p| {
        show_scope(&c.scope.borrow(), p);
        for f in &c.field {
            if let FieldDef::FuncDef(f) = f {
                func_def(f, p);
            }
        }
    });
}

pub fn func_def(f: &FuncDef, p: &mut IndentPrinter) {
    write!(p, "FORMAL SCOPE OF '{}':", f.name).ignore();
    p.indent(|p| {
        show_scope(&f.scope.borrow(), p);
        if let Some(_) = f.body { block(&f.body.as_ref().expect("print scope: unwrap a non body"), p); }
    });
}

pub fn expr(e: &Expr, p: &mut IndentPrinter) {
    if let ExprKind::Lambda(lam) = &e.kind {
        write!(p, "FORMAL SCOPE OF '{}':", lam.name).ignore();
        p.indent(|p| {
            show_scope(&lam.scope.borrow(), p);
            match &lam.kind {
                LambdaKind::Expr(_) => {
                    write!(p, "LOCAL SCOPE:").ignore();
                    p.indent(|p| write!(p, "<empty>").ignore());
                },
                LambdaKind::Block(b) => block(b, p),
            };
        });
    }
}

pub fn block(b: &Block, p: &mut IndentPrinter) {
    write!(p, "LOCAL SCOPE:").ignore();
    p.indent(|p| {
        show_scope(&b.scope.borrow(), p);
        for s in &b.stmt {
            match &s.kind {
                StmtKind::If(i) => {
                    block(&i.on_true, p);
                    if let Some(on_false) = &i.on_false {
                        block(on_false, p);
                    }
                }
                StmtKind::While(w) => block(&w.body, p),
                StmtKind::For(f) => block(&f.body, p),
                StmtKind::Block(b) => block(b, p),
                
                //suport lambda
                StmtKind::Assign(a) => {
                    expr(&a.dst, p);
                    expr(&a.src, p);
                },
                StmtKind::LocalVarDef(l) => {
                    if let Some((_, e)) = &l.init {
                        expr(e, p);
                    }
                }
                StmtKind::ExprEval(e) => expr(e, p),
                StmtKind::Return(ret) => {
                    if let Some(e) = &ret { expr(e, p); }
                },
                StmtKind::Print(v) => {
                    for e in v { expr(e, p); }
                },
                _ => {}
            }
        }
    });
}
