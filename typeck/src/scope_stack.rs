use common::Loc;
use std::iter;
use syntax::{ClassDef, Program, ScopeOwner, Symbol};

pub(crate) struct ScopeStack<'a> {
    // `global` must be ScopeOwner::Global, but we will not depend on this, so just define it as ScopeOwner
    global: ScopeOwner<'a>,
    stack: Vec<ScopeOwner<'a>>,
}

impl<'a> ScopeStack<'a> {
    pub fn new(p: &'a Program<'a>) -> Self {
        Self {
            global: ScopeOwner::Global(p),
            stack: vec![],
        }
    }

    pub fn lookup(&self, name: &'a str) -> Option<(Symbol<'a>, ScopeOwner<'a>)> {
        self.stack
            .iter()
            .rev()
            .chain(iter::once(&self.global))
            .filter_map(|&owner| owner.scope().get(name).map(|&sym| (sym, owner)))
            .next()
    }

    // do lookup, but will ignore those local symbols whose loc >= the given loc
    //pub fn lookup_before(&self, name: &'a str, loc: Loc) -> Option<Symbol<'a>> {
    /*
    pub fn lookup_before(&self, name: &'a str, finish_loc: Loc) -> Option<Symbol<'a>> {
        self.stack
            .iter()
            .rev()
            .chain(iter::once(&self.global))
            .filter_map(|&owner| {
                owner
                    .scope()
                    .get(name)
                    .cloned()
                    //.filter(|sym| !(owner.is_local() && sym.loc() >= loc))
                    .filter(|sym| !(owner.is_local() && sym.finish_loc() >= finish_loc))
            })
            .next()
    }
    */
    pub fn lookup_before(&self, name: &'a str, finish_loc: Loc) -> (Option<Symbol<'a>>, bool) {
        let mut out_of_lambda = false;
        for &owner in self.stack.iter().rev().chain(iter::once(&self.global)) {
            if let Some(sym) = owner.scope().get(name).cloned() {
                if !(owner.is_local() && sym.finish_loc() >= finish_loc) {
                    return (Some(sym), out_of_lambda);
                }
            }

            if !out_of_lambda && owner.is_lambda_param() {
                out_of_lambda = true;
            }
        }
        return (None, out_of_lambda);
        /*
        let out_of_lambda = false;
        self.stack.iter().rev().chain(iter::once(&self.global)).filter_map(|&owner| {
            owner.scope().get(name).cloned().filter(|sym| {
                !(owner.is_local() && sym.finish_loc() >= finish_loc)
            })
        }).next()
        */
    }

    pub fn declare(&mut self, sym: Symbol<'a>) {
        self.cur_owner().scope_mut().insert(sym.name(), sym);
    }

    // if `owner` is ScopeOwner::Class, then will recursively open all its ancestors
    pub fn open(&mut self, owner: ScopeOwner<'a>) {
        if let ScopeOwner::Class(c) = owner {
            if let Some(p) = c.parent_ref.get() {
                self.open(ScopeOwner::Class(p));
            }
        }
        self.stack.push(owner);
    }

    // the global scope is not affected
    pub fn close(&mut self) {
        let owner = self.stack.pop().expect("unwrap a non stack top item");
        if let ScopeOwner::Class(_) = owner {
            self.stack.clear(); // all scopes in the stack is its ancestors
        }
    }

    pub fn cur_owner(&self) -> ScopeOwner<'a> {
        *self.stack.last().unwrap_or(&self.global)
    }

    pub fn lookup_class(&self, name: &'a str) -> Option<&'a ClassDef<'a>> {
        self.global.scope().get(name).map(|class| match class {
            Symbol::Class(c) => *c,
            _ => unreachable!("Global scope should only contain classes."),
        })
    }
}
