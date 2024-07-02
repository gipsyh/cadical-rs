pub mod craig;

use logic_form::{Lit, Var};
use satif::{SatResult, Satif, SatifSat, SatifUnsat};
use std::ffi::{c_int, c_void};

extern "C" {
    fn cadical_solver_new() -> *mut c_void;
    fn cadical_solver_free(s: *mut c_void);
    fn cadical_solver_add_clause(s: *mut c_void, clause: *mut c_int, len: c_int);
    fn cadical_solver_solve(s: *mut c_void, assumps: *mut c_int, len: c_int) -> c_int;
    // fn cadical_solver_constrain(s: *mut c_void, constrain: *mut c_int, len: c_int);
    fn cadical_solver_simplify(s: *mut c_void);
    // fn solver_set_polarity(s: *mut c_void, var: c_int, pol: c_int);
    fn cadical_solver_model_value(s: *mut c_void, lit: c_int) -> c_int;
    fn cadical_solver_conflict_has(s: *mut c_void, lit: c_int) -> bool;
    fn cadical_craig_test();
}

fn lit_to_cadical_lit(lit: &Lit) -> i32 {
    let mut res = Into::<usize>::into(lit.var()) as i32 + 1;
    if !lit.polarity() {
        res = -res;
    }
    res
}

fn cadical_lit_to_lit(lit: i32) -> Lit {
    let p = lit > 0;
    let v = Var::new(lit.abs() as usize - 1);
    Lit::new(v, p)
}

pub struct Sat {
    solver: *mut c_void,
}

impl SatifSat for Sat {
    fn lit_value(&self, lit: Lit) -> Option<bool> {
        let lit = lit_to_cadical_lit(&lit);
        let res = unsafe { cadical_solver_model_value(self.solver, lit) };
        if res == lit {
            Some(true)
        } else if res == -lit {
            Some(false)
        } else {
            None
        }
    }
}

pub struct Unsat {
    solver: *mut c_void,
}

impl SatifUnsat for Unsat {
    fn has(&self, lit: Lit) -> bool {
        let lit = lit_to_cadical_lit(&lit);
        unsafe { cadical_solver_conflict_has(self.solver, lit) }
    }
}

pub struct Solver {
    solver: *mut c_void,
    num_var: usize,
}

impl Satif for Solver {
    type Sat = Sat;
    type Unsat = Unsat;

    #[inline]
    fn new() -> Self {
        Self {
            solver: unsafe { cadical_solver_new() },
            num_var: 0,
        }
    }

    #[inline]
    fn new_var(&mut self) -> Var {
        self.num_var += 1;
        Var::new(self.num_var - 1)
    }

    #[inline]
    fn num_var(&self) -> usize {
        self.num_var
    }

    #[inline]
    fn add_clause(&mut self, clause: &[Lit]) {
        let clause: Vec<i32> = clause.iter().map(lit_to_cadical_lit).collect();
        unsafe { cadical_solver_add_clause(self.solver, clause.as_ptr() as _, clause.len() as _) }
    }

    fn solve(&mut self, assumps: &[Lit]) -> SatResult<Self::Sat, Self::Unsat> {
        let assumps: Vec<i32> = assumps.iter().map(lit_to_cadical_lit).collect();
        match unsafe {
            cadical_solver_solve(self.solver, assumps.as_ptr() as _, assumps.len() as _)
        } {
            10 => SatResult::Sat(Sat {
                solver: self.solver,
            }),
            20 => SatResult::Unsat(Unsat {
                solver: self.solver,
            }),
            _ => todo!(),
        }
    }
}

impl Solver {
    // pub fn solve_with_constrain<'a>(
    //     &'a mut self,
    //     assumps: &[Lit],
    //     constrain: &[Lit],
    // ) -> SatResult<'a> {
    //     let constrain: Vec<i32> = constrain.iter().map(lit_to_cadical_lit).collect();
    //     unsafe {
    //         cadical_solver_constrain(self.solver, constrain.as_ptr() as _, constrain.len() as _)
    //     };
    //     self.solve(assumps)
    // }

    pub fn simplify(&mut self) {
        unsafe { cadical_solver_simplify(self.solver) };
    }

    // pub fn set_polarity(&mut self, var: Var, pol: Option<bool>) {
    //     let pol = match pol {
    //         Some(true) => 0,
    //         Some(false) => 1,
    //         None => 2,
    //     };
    //     unsafe { solver_set_polarity(self.solver, var.into(), pol) }
    // }

    // pub fn set_random_seed(&mut self, seed: f64) {
    //     unsafe { solver_set_random_seed(self.solver, seed) }
    // }

    // pub fn set_rnd_init_act(&mut self, enable: bool) {
    //     unsafe { solver_set_rnd_init_act(self.solver, enable) }
    // }

    /// # Safety
    /// unsafe get sat model
    pub unsafe fn get_model(&self) -> Sat {
        Sat {
            solver: self.solver,
        }
    }

    /// # Safety
    /// unsafe get unsat core
    pub unsafe fn get_conflict(&self) -> Unsat {
        Unsat {
            solver: self.solver,
        }
    }
}

impl Drop for Solver {
    fn drop(&mut self) {
        unsafe { cadical_solver_free(self.solver) }
    }
}

impl Default for Solver {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test() {
    use logic_form::Clause;
    let mut solver = Solver::new();
    let lit0: Lit = solver.new_var().into();
    let lit1: Lit = solver.new_var().into();
    let lit2: Lit = solver.new_var().into();
    solver.add_clause(&Clause::from([lit0, !lit2]));
    solver.add_clause(&Clause::from([lit1, !lit2]));
    solver.add_clause(&Clause::from([!lit0, !lit1, lit2]));
    match solver.solve(&[lit2]) {
        SatResult::Sat(model) => {
            assert!(model.lit_value(lit0).unwrap());
            assert!(model.lit_value(lit1).unwrap());
            assert!(model.lit_value(lit2).unwrap());
        }
        SatResult::Unsat(_) => todo!(),
    }
    // solver.simplify();
    // match solver.solve_with_constrain(&[lit2], &[!lit0]) {
    //     SatResult::Sat(_) => panic!(),
    //     SatResult::Unsat(conflict) => {
    //         assert!(conflict.has(lit2));
    //     }
    // }
    // match solver.solve(&[lit2]) {
    //     SatResult::Sat(_) => {}
    //     SatResult::Unsat(_) => todo!(),
    // }
}

#[test]
fn test_craig() {
    unsafe { cadical_craig_test() };
}
