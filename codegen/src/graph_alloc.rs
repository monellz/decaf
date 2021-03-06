// http://www.cse.iitm.ac.in/~krishna/cs6013/george.pdf

use crate::Reg;
use common::{HashSet, IndexSet};
use std::marker::PhantomData;

pub trait AllocCtx: Sized {
    // number of registers to allocate
    const K: u32;

    // return (initial virtual registers, initial nodes)
    // pre-colored and normal registers are indexed in the same way, you can distinguish them by using different number ranges
    fn initial(&self) -> (Vec<u32>, Vec<Node>);

    // build inference graph, this should include building all edges and all possible moves
    // to be more specific, implementation should call allocator.add_edge to build edges
    // and initialize allocator.work_list_moves, allocator.nodes[..].move_list to build moves
    fn build(&self, allocator: &mut Allocator<Self>);

    // generate spill related code, no need to build inference graph here, because build() will be called again
    fn rewrite(&mut self, spilled_nodes: &HashSet<u32>);

    // use result[..].color to replace transform virtual registers in asm into physical registers
    fn finish(&mut self, result: &[Node]);
}

pub struct Node {
    pub degree: u32,
    pub alias: u32,
    pub color: Reg,
    pub adj_list: Vec<u32>,
    pub move_list: Vec<(u32, u32)>,
}

impl Node {
    pub fn new(color: Reg) -> Node {
        // pre-colored register's degree is set to a very big value(>= K + number of nodes is ok)
        let degree = if let Reg::PreColored(_) = color {
            std::u32::MAX
        } else {
            0
        };
        Node {
            degree,
            alias: 0,
            color,
            adj_list: Vec::new(),
            move_list: Vec::new(),
        }
    }

    pub fn pre_colored(&self) -> bool {
        match self.color {
            Reg::PreColored(_) => true,
            _ => false,
        }
    }

    pub fn expect_colored(&self) -> u32 {
        match self.color {
            Reg::PreColored(r) | Reg::Allocated(r) => r,
            Reg::Virtual(r) => panic!(
                "Register allocation not finished yet, now is virtual register {}.",
                r
            ),
        }
    }
}

// some fields the paper mentions are not really necessary, I leave them in comments
// some fields doesn't need to be a set, because only push(guaranteed unique) and iteration are required
// some fields need to be a set and need to record insertion order, use IndexSet
pub struct Allocator<A: AllocCtx> {
    pub nodes: Vec<Node>,
    // machine registers, preassigned a color
    // pre_colored: HashSet<u32>,
    // virtual registers, not preassigned a color and not yet processed by the algorithm
    initial: Vec<u32>,
    // list of low-degree non-move-related nodes
    simplify_work_list: HashSet<u32>,
    // low-degree move-related nodes
    freeze_work_list: HashSet<u32>,
    // high-degree nodes
    spill_work_list: HashSet<u32>,
    // nodes marked for spilling during this round; initially empty
    spilled_nodes: HashSet<u32>,
    // registers that have been coalesced;
    // when the move u = v is coalesced, one of u or v is added to this set, and the other is put back on some work list
    coalesced_nodes: HashSet<u32>,
    // nodes successfully colored
    // colored_nodes: Vec<u32>,
    // stack containing temporaries removed from the graph
    select_stack: IndexSet<u32>,
    // moves that have been coalesced
    // coalesced_moves: HashSet<(u32, u32)>,
    // moves whose source and target interfere
    // constrained_moves: HashSet<(u32, u32)>,
    // moves that will no longer be considered for coalescing
    // frozen_moves: HashSet<(u32, u32)>,
    // moves enabled for possible coalescing
    pub work_list_moves: HashSet<(u32, u32)>,
    // moves not yet ready for coalescing
    active_moves: HashSet<(u32, u32)>,
    adj_set: HashSet<(u32, u32)>,
    _p: PhantomData<A>,
}

impl<A: AllocCtx> Allocator<A> {
    pub fn work(ctx: &mut A) {
        // unluckily cannot use #[derive(Default)] because A may not be Default, even though PhantomData<A> is
        // I still don't know why rust has such a requirement
        let mut a = Allocator {
            nodes: Vec::new(),
            initial: Vec::new(),
            simplify_work_list: HashSet::new(),
            freeze_work_list: HashSet::new(),
            spill_work_list: HashSet::new(),
            spilled_nodes: HashSet::new(),
            coalesced_nodes: HashSet::new(),
            select_stack: IndexSet::default(),
            work_list_moves: HashSet::new(),
            active_moves: HashSet::new(),
            adj_set: HashSet::new(),
            _p: PhantomData,
        };
        // actually no information in `a` is preserved for the next loop
        // because in this simple variant of this algo, all coalesces are discarded if spill happens
        // so the only reason for creating `a` outside the loop is to reuse some memory
        // should remember to clear all fields after each iteration step (`initial` and `nodes` doesn't have to be cleared because they will be reassigned)
        let nodes = loop {
            let (initial, nodes) = ctx.initial();
            a.initial = initial;
            a.nodes = nodes;
            ctx.build(&mut a);
            a.mk_work_list();
            loop {
                match () {
                    // just to avoid many if-else
                    _ if !a.simplify_work_list.is_empty() => a.simplify(),
                    _ if !a.work_list_moves.is_empty() => a.coalesce(),
                    _ if !a.freeze_work_list.is_empty() => a.freeze(),
                    _ if !a.spill_work_list.is_empty() => a.select_spill(),
                    _ => break,
                }
            }
            a.assign_color();
            if !a.spilled_nodes.is_empty() {
                a.rewrite_program(ctx);
            } else {
                break a.nodes;
            }
        };
        ctx.finish(&nodes);
    }

    pub fn add_edge(&mut self, u: u32, v: u32) {
        if u != v && !self.adj_set.contains(&(u, v)) {
            self.adj_set.insert((u, v));
            self.adj_set.insert((v, u));
            let (u, v) = (u as usize, v as usize);
            // pre colored register can be the dest of edge, but not the src(or it's adj_list may be too big)
            // its degree will not grow, but can decrease starting from std::u32::MAX(still won't have any effect, can never have a degree < K)
            if !self.nodes[u].pre_colored() {
                self.nodes[u].adj_list.push(v as u32);
                self.nodes[u].degree += 1;
            }
            if !self.nodes[v].pre_colored() {
                self.nodes[v].adj_list.push(u as u32);
                self.nodes[v].degree += 1;
            }
        }
    }

    // the paper defines many functions that return a set of nodes, we don't really need to allocate space for a set, using an iterator is better
    // however rust's lifetime requirement almost make it impossible to define such functions that return an iterator
    // because it must borrow self as a whole, so you can't modify any other fields, even though they are not involved in this iterator
    // the solution is to inline these functions manually, then rustc knows that it will borrows some fields of self

    fn adjacent(&self, n: u32) -> HashSet<u32> {
        let mut adjlist: HashSet<_> = self.nodes[n as usize].adj_list.iter().cloned().collect();
        adjlist.retain(|x| !self.select_stack.contains(x) && !self.coalesced_nodes.contains(x));
        adjlist
    }

    fn node_moves(&self, n: u32) -> HashSet<(u32, u32)> {
        let mut move_list: HashSet<_> = self.nodes[n as usize].move_list.iter().cloned().collect();
        move_list.retain(|x| self.active_moves.contains(x) || self.work_list_moves.contains(x));
        move_list
    }

    fn move_related(&self, n: u32) -> bool {
        !self.node_moves(n).is_empty()
    }

    fn mk_work_list(&mut self) {
        let initial = self.initial.clone();
        for n in initial {
            if self.nodes[n as usize].degree >= A::K {
                self.spill_work_list.insert(n);
            } else if self.move_related(n) {
                self.freeze_work_list.insert(n);
            } else {
                self.simplify_work_list.insert(n);
            }
        }
        self.initial.clear();
    }

    fn simplify(&mut self) {
        let n = *self.simplify_work_list.iter().next().unwrap();
        self.simplify_work_list.take(&n);
        self.select_stack.insert(n);
        for m in self.adjacent(n) {
            self.decrement_degree(m);
        }
    }

    fn decrement_degree(&mut self, m: u32) {
        let d = self.nodes[m as usize].degree;
        self.nodes[m as usize].degree = d - 1;
        if d == A::K {
            let mut moves = self.adjacent(m);
            moves.insert(m);
            self.enable_moves(moves);

            self.spill_work_list.remove(&m);
            if self.move_related(m) {
                self.freeze_work_list.insert(m);
            } else {
                self.simplify_work_list.insert(m);
            }
        }
    }

    fn enable_moves(&mut self, nodes: HashSet<u32>) {
        for n in nodes {
            for m in self.node_moves(n) {
                if self.active_moves.contains(&m) {
                    self.active_moves.remove(&m);
                    self.work_list_moves.insert(m);
                } 
            }
        }
    }

    fn coalesce(&mut self) {
        let m = *self.work_list_moves.iter().next().unwrap();
        let x = self.get_alias(m.0);
        let y = self.get_alias(m.1);
        let (u, v) = if self.nodes[y as usize].pre_colored() {
            (y, x)
        } else {
            (x, y)
        };
        self.work_list_moves.remove(&m);
        if u == v {
            //coalescedMoves = coalescedMoves \union {m}
            self.add_work_list(u);
        } else if self.nodes[v as usize].pre_colored() || self.adj_set.contains(&(u, v)){
            //coalescedMoves = coalescedMoves \union {m}
            self.add_work_list(u);
            self.add_work_list(v);
        } else {
            let cond = self.nodes[u as usize].pre_colored() && self.adjacent(v).iter().fold(true, |acc, &t| acc && self.ok(t, u));
            let tmp_union = self.adjacent(u).into_iter().chain(self.adjacent(v).into_iter()).collect();
            let cond = cond || (!self.nodes[u as usize].pre_colored() && self.conservative(tmp_union));
            if cond {
                //coalescedMoves = coalescedMoves \union {m}
                self.combine(u, v);
                self.add_work_list(u);
            } else {
                self.active_moves.insert(m);        
            }
        }
    }

    fn add_work_list(&mut self, u: u32) {
        if !self.nodes[u as usize].pre_colored() && !self.move_related(u) && self.nodes[u as usize].degree < A::K {
            self.freeze_work_list.remove(&u);
            self.simplify_work_list.insert(u);
        }
    }

    fn ok(&self, t: u32, r: u32) -> bool {
        self.nodes[t as usize].degree < A::K || self.nodes[t as usize].pre_colored() || self.adj_set.contains(&(t, r))
    }

    fn conservative(&self, nodes: HashSet<u32>) -> bool {
        let mut k = 0;
        for n in nodes {
            if self.nodes[n as usize].degree >= A::K { k += 1; }
        }
        k < A::K
    }

    fn get_alias(&self, n: u32) -> u32 {
        if self.coalesced_nodes.contains(&n) { self.get_alias(self.nodes[n as usize].alias) }
        else { n }
    }

    fn combine(&mut self, u: u32, v: u32) {
        if self.freeze_work_list.contains(&v) {
            self.freeze_work_list.remove(&v);
        } else {
            self.spill_work_list.remove(&v);
        }

        self.coalesced_nodes.insert(v);
        self.nodes[v as usize].alias = u;
        //nodeMoves[u] = nodeMoves[u] \union nodeMoves[v]
        for t in self.adjacent(v) {
            self.add_edge(t, u);
            self.decrement_degree(t);
        }

        if self.nodes[u as usize].degree >= A::K && self.freeze_work_list.contains(&u) {
            self.freeze_work_list.remove(&u);
            self.spill_work_list.insert(u);
        }
    }

    fn freeze(&mut self) {
        let u = *self.freeze_work_list.iter().next().unwrap();
        self.freeze_work_list.remove(&u);
        self.simplify_work_list.insert(u);
        self.freeze_moves(u);
    }

    fn freeze_moves(&mut self, u: u32) {
        for m in self.node_moves(u) {
            if self.active_moves.contains(&m) {
                self.active_moves.remove(&m);
            } else {
                self.work_list_moves.remove(&m);
            }
            let (_, v) = m;
            //frozenMoves = frozenMoves \union {m}
            if self.node_moves(v).is_empty() && self.nodes[v as usize].degree < A::K {
                self.freeze_work_list.remove(&v);
                self.simplify_work_list.insert(v);
            }
        }
    }

    fn select_spill(&mut self) {
        //???? favorite heuristic
        let m = *self.spill_work_list.iter().next().unwrap();
        self.spill_work_list.remove(&m);
        self.simplify_work_list.insert(m);
        self.freeze_moves(m);
    }

    fn assign_color(&mut self) {
        let mut available = HashSet::with_capacity(A::K as usize);
        for &n in self.select_stack.iter().rev() {
            // pop all, need to traverse reversely
            available.clear();
            for i in 0..A::K {
                available.insert(i);
            }
            for &w in &self.nodes[n as usize].adj_list {
                let a = self.get_alias(w);
                match self.nodes[a as usize].color {
                    Reg::PreColored(r) | Reg::Allocated(r) => {
                        available.remove(&r);
                    }
                    Reg::Virtual(_) => {}
                };
            }
            // PreColored nodes should never be added to select_stack
            // so this color assignment will not give a PreColored node a wrong color
            if let Some(r) = available.iter().nth(0) {
                self.nodes[n as usize].color = Reg::Allocated(*r);
            } else {
                self.spilled_nodes.insert(n);
            }
        }
        self.select_stack.clear();
        for &n in &self.coalesced_nodes {
            self.nodes[n as usize].color = self.nodes[self.get_alias(n) as usize].color;
        }
    }

    fn rewrite_program(&mut self, ctx: &mut A) {
        ctx.rewrite(&self.spilled_nodes);
        self.spilled_nodes.clear();
        self.coalesced_nodes.clear();
        self.active_moves.clear();
        self.adj_set.clear();
    }
}
