use crate::tracer::Tracer;
use aig::{Aig, AigEdge};
use logic_form::{Clause, Lit, Var};
use std::collections::{HashMap, HashSet};

pub struct Interpolant {
    global_vars: HashMap<Var, AigEdge>,
    cls_labels: HashMap<usize, bool>,
    next_cls_label: Option<bool>,
    aig: Aig,
    itp: HashMap<usize, AigEdge>,
    clauses: HashMap<usize, Clause>,
    mark: HashSet<Var>,
}

impl Interpolant {
    pub fn new() -> Self {
        Self {
            global_vars: Default::default(),
            cls_labels: Default::default(),
            next_cls_label: Default::default(),
            aig: Default::default(),
            itp: Default::default(),
            clauses: Default::default(),
            mark: HashSet::default(),
        }
    }

    pub fn label_var(&mut self, var: Var) {
        let node = self.aig.new_leaf_node();
        self.aig.new_input(node);
        self.global_vars.insert(var, Into::<AigEdge>::into(node));
    }

    pub fn label_clause(&mut self, k: bool) {
        self.next_cls_label = Some(k)
    }
}

impl Tracer for Interpolant {
    fn add_original_clause(&mut self, id: usize, _redundant: bool, c: &[Lit], restore: bool) {
        if !restore {
            assert!(self
                .cls_labels
                .insert(id, self.next_cls_label.unwrap())
                .is_none());
        }
        let itp = if self.cls_labels[&id] {
            AigEdge::constant_edge(true)
        } else {
            let or = c.iter().filter_map(|l| {
                self.global_vars
                    .get(&l.var())
                    .map(|e| e.not_if(!l.polarity()))
            });
            self.aig.new_ors_node(or)
        };
        self.itp.insert(id, itp);
        self.clauses.insert(id, Clause::from(c));
    }

    fn add_derived_clause(&mut self, id: usize, _redundant: bool, c: &[Lit], p: &[usize]) {
        let conflict = p.last().unwrap();

        for l in self.clauses[conflict].iter() {
            self.mark.insert(l.var());
        }
        let mut itp = self.itp[conflict];
        for pi in (0..p.len() - 1).rev() {
            for l in self.clauses[&p[pi]].iter() {
                if self.mark.insert(l.var()) {
                    continue;
                }
                itp = if self.global_vars.contains_key(&l.var()) {
                    self.aig.new_and_node(itp, self.itp[&p[pi]])
                } else {
                    self.aig.new_or_node(itp, self.itp[&p[pi]])
                };
            }
        }
        self.mark.clear();
        self.itp.insert(id, itp);
        self.clauses.insert(id, Clause::from(c));
    }

    fn delete_clause(&mut self, id: usize, _redundant: bool, _c: &[Lit]) {
        self.itp.remove(&id);
        self.clauses.remove(&id);
    }

    fn conclude_unsat(&mut self, conclusion: i32, p: &[usize]) {
        if conclusion == 1 {
            assert!(p.len() == 1);
            dbg!(self.itp[&p[0]]);
            println!("{}", self.aig);
        } else {
            todo!();
        }
    }
}
