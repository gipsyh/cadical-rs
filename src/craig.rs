use logic_form::{Clause, Cnf, Lit, Var};
use satif::Satif;

use crate::{cadical_lit_to_lit, Solver};
use std::{ffi::c_int, os::raw::c_void};

extern "C" {
    fn cadical_craig_new(s: *mut c_void) -> *mut c_void;
    fn cadical_craig_free(c: *mut c_void);
    fn cadical_craig_label_var(c: *mut c_void, var: i32, t: u8);
    fn cadical_craig_label_clause(c: *mut c_void, id: i32, t: u8);
    fn cadical_craig_create_craig_interpolant(
        c: *mut c_void,
        next_var: *mut c_int,
        len: *mut c_int,
    ) -> *mut c_void;
    // fn cadical_solver_add_clause(s: *mut c_void, clause: *mut c_int, len: c_int);
    // fn cadical_solver_solve(s: *mut c_void, assumps: *mut c_int, len: c_int) -> c_int;
    // // fn cadical_solver_constrain(s: *mut c_void, constrain: *mut c_int, len: c_int);
    // // fn cadical_solver_simplify(s: *mut c_void);
    // // fn solver_set_polarity(s: *mut c_void, var: c_int, pol: c_int);
    // fn cadical_solver_model_value(s: *mut c_void, lit: c_int) -> c_int;
    // fn cadical_solver_conflict_has(s: *mut c_void, lit: c_int) -> bool;
    // fn cadical_craig_test();
}

pub enum VarLabel {
    A,
    B,
    Global,
}

impl Into<u8> for VarLabel {
    fn into(self) -> u8 {
        match self {
            VarLabel::A => 0,
            VarLabel::B => 1,
            VarLabel::Global => 2,
        }
    }
}

pub enum ClauseLabel {
    A,
    B,
}

impl Into<u8> for ClauseLabel {
    fn into(self) -> u8 {
        match self {
            ClauseLabel::A => 0,
            ClauseLabel::B => 1,
        }
    }
}

pub struct Craig {
    pub solver: Solver,
    craig: *mut c_void,
    num_clause: usize,
}

impl Craig {
    pub fn new(solver: Solver) -> Self {
        let craig = unsafe { cadical_craig_new(solver.solver) };
        Self {
            solver,
            craig,
            num_clause: 0,
        }
    }

    pub fn label_var(&mut self, var: Var, label: VarLabel) {
        let mut var: i32 = var.into();
        var += 1;
        while self.solver.num_var < var as _ {
            self.solver.new_var();
        }
        unsafe { cadical_craig_label_var(self.craig, var, label.into()) }
    }

    pub fn add_clause(&mut self, clause: &[Lit], label: ClauseLabel) {
        self.num_clause += 1;
        unsafe { cadical_craig_label_clause(self.craig, self.num_clause as _, label.into()) }
        self.solver.add_clause(clause);
    }

    pub fn interpolant(&mut self, next_var: usize) -> Cnf {
        unsafe {
            let mut cnf = Cnf::new();
            let mut len = 0;
            let mut next_var = next_var as i32;
            next_var += 1;
            let clauses: *mut usize = cadical_craig_create_craig_interpolant(
                self.craig,
                &mut next_var as *mut _,
                &mut len as *mut _,
            ) as _;
            let clauses = Vec::from_raw_parts(clauses, len as _, len as _);
            for i in (0..clauses.len()).step_by(2) {
                let data = clauses[i] as *mut i32;
                let len = clauses[i + 1];
                let cls: Vec<i32> = Vec::from_raw_parts(data, len, len);
                let cls: Vec<Lit> = cls.into_iter().map(|l| cadical_lit_to_lit(l)).collect();
                cnf.add_clause(Clause::from(cls));
            }
            cnf
        }
    }
}

// impl Drop for Craig {
//     fn drop(&mut self) {
//         todo!();
//         unsafe { cadical_craig_free(self.craig) }
//     }
// }

#[test]
fn test() {
    let solver = Solver::new();
    let mut craig = Craig::new(solver);
    craig.label_var(Var::new(0), VarLabel::A);
    craig.label_var(Var::new(1), VarLabel::Global);
    craig.label_var(Var::new(2), VarLabel::Global);
    craig.label_var(Var::new(3), VarLabel::B);
    craig.add_clause(
        &[Lit::new(Var(0), true), Lit::new(Var(1), false)],
        ClauseLabel::A,
    );
    craig.add_clause(
        &[Lit::new(Var(0), false), Lit::new(Var(2), false)],
        ClauseLabel::A,
    );
    craig.add_clause(&[Lit::new(Var(1), true)], ClauseLabel::A);
    craig.add_clause(
        &[Lit::new(Var(1), false), Lit::new(Var(2), true)],
        ClauseLabel::B,
    );
    craig.add_clause(
        &[Lit::new(Var(1), true), Lit::new(Var(3), true)],
        ClauseLabel::B,
    );
    craig.add_clause(&[Lit::new(Var(3), false)], ClauseLabel::B);
    dbg!(craig.solver.solve(&[]));
    dbg!(craig.interpolant(4));
}
