pub mod craig;
pub mod itp;
pub mod tracer;

use giputils::hash::GHashMap;
use logicrs::{Lit, LitVec, Var, satif::Satif};
use std::ffi::{c_int, c_void};

unsafe extern "C" {
    fn cadical_solver_new() -> *mut c_void;
    fn cadical_solver_free(s: *mut c_void);
    fn cadical_solver_add_clause(s: *mut c_void, clause: *mut c_int, len: c_int);
    fn cadical_solver_solve(s: *mut c_void, assumps: *mut c_int, len: c_int) -> c_int;
    fn cadical_solver_constrain(s: *mut c_void, constrain: *mut c_int, len: c_int);
    fn cadical_solver_simplify(s: *mut c_void) -> c_int;
    fn cadical_solver_freeze(s: *mut c_void, lit: c_int);
    fn cadical_set_polarity(s: *mut c_void, lit: c_int);
    fn cadical_unset_polarity(s: *mut c_void, lit: c_int);
    fn cadical_solver_model_value(s: *mut c_void, lit: c_int) -> c_int;
    fn cadical_solver_conflict_has(s: *mut c_void, lit: c_int) -> bool;
    fn cadical_solver_clauses(s: *mut c_void, len: *mut c_int) -> *mut c_void;
    fn cadical_set_seed(s: *mut c_void, seed: c_int);
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
    let v = Var::new(lit.unsigned_abs() as usize - 1);
    Lit::new(v, p)
}

pub struct Solver {
    solver: *mut c_void,
    num_var: usize,
    tracer_map: GHashMap<*const c_void, *const c_void>,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            solver: unsafe { cadical_solver_new() },
            num_var: 0,
            tracer_map: GHashMap::default(),
        }
    }
}

impl Satif for Solver {
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

    fn solve(&mut self, assumps: &[Lit]) -> bool {
        let assumps: Vec<i32> = assumps.iter().map(lit_to_cadical_lit).collect();
        match unsafe {
            cadical_solver_solve(self.solver, assumps.as_ptr() as _, assumps.len() as _)
        } {
            10 => true,
            20 => false,
            _ => todo!(),
        }
    }

    fn sat_value(&self, lit: Lit) -> Option<bool> {
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

    fn unsat_has(&self, lit: Lit) -> bool {
        let lit = lit_to_cadical_lit(&lit);
        unsafe { cadical_solver_conflict_has(self.solver, lit) }
    }

    fn simplify(&mut self) -> Option<bool> {
        match unsafe { cadical_solver_simplify(self.solver) } {
            10 => Some(true),
            20 => Some(false),
            _ => None,
        }
    }

    fn set_frozen(&mut self, var: Var, frozen: bool) {
        assert!(frozen);
        unsafe { cadical_solver_freeze(self.solver, lit_to_cadical_lit(&var.lit())) }
    }

    fn clauses(&self) -> Vec<LitVec> {
        let mut cnf = Vec::new();
        unsafe {
            let mut len = 0;
            let clauses: *mut usize = cadical_solver_clauses(self.solver, &mut len as *mut _) as _;
            if len > 0 {
                let clauses = Vec::from_raw_parts(clauses, len as _, len as _);
                for i in (0..clauses.len()).step_by(2) {
                    let data = clauses[i] as *mut i32;
                    let len = clauses[i + 1];
                    let cls: Vec<_> = (0..len).map(|i| *data.add(i)).collect();
                    cnf.push(LitVec::from_iter(cls.into_iter().map(cadical_lit_to_lit)));
                }
            }
        }
        cnf
    }

    fn set_seed(&mut self, seed: u64) {
        unsafe { cadical_set_seed(self.solver, seed as _) }
    }
}

impl Solver {
    pub fn solve_with_constrain(&mut self, assumps: &[Lit], constrain: &[Lit]) -> bool {
        let constrain: Vec<i32> = constrain.iter().map(lit_to_cadical_lit).collect();
        unsafe {
            cadical_solver_constrain(self.solver, constrain.as_ptr() as _, constrain.len() as _)
        };
        self.solve(assumps)
    }

    pub fn set_polarity(&mut self, var: Var, pol: Option<bool>) {
        match pol {
            Some(p) => {
                let p = var.lit().not_if(!p);
                unsafe { cadical_set_polarity(self.solver, lit_to_cadical_lit(&p)) }
            }
            None => unsafe { cadical_unset_polarity(self.solver, lit_to_cadical_lit(&var.lit())) },
        };
    }
}

impl Drop for Solver {
    fn drop(&mut self) {
        unsafe { cadical_solver_free(self.solver) };
    }
}

impl Default for Solver {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test() {
    use logicrs::LitVec;
    let mut solver = Solver::new();
    let lit0: Lit = solver.new_var().into();
    let lit1: Lit = solver.new_var().into();
    let lit2: Lit = solver.new_var().into();
    solver.add_clause(&LitVec::from([lit0, !lit2]));
    solver.add_clause(&LitVec::from([lit1, !lit2]));
    solver.add_clause(&LitVec::from([!lit0, !lit1, lit2]));
    if solver.solve(&[lit2]) {
        assert!(solver.sat_value(lit0).unwrap());
        assert!(solver.sat_value(lit1).unwrap());
        assert!(solver.sat_value(lit2).unwrap());
    } else {
        panic!()
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
