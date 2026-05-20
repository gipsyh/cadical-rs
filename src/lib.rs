use giputils::StopCtrl;
use logicrs::{Lit, LitVec, Var, VarMap, satif::Satif};
use std::ffi::{c_int, c_void};

unsafe extern "C" {
    fn cadical_solver_new() -> *mut c_void;
    fn cadical_solver_new_var(s: *mut c_void) -> c_int;
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
    fn cadical_terminate(s: *mut c_void);
}

pub struct CaDiCaL {
    solver: *mut c_void,
    num_var: usize,
    var2cadical_var: VarMap<Var>,
    cadical_var2var: VarMap<Option<Var>>,
}

impl CaDiCaL {
    pub fn new() -> Self {
        Self {
            solver: unsafe { cadical_solver_new() },
            num_var: 0,
            var2cadical_var: VarMap::new(),
            cadical_var2var: VarMap::new(),
        }
    }

    #[inline]
    fn var_to_cadical_var(&self, var: Var) -> Var {
        assert!(usize::from(var) < self.num_var);
        self.var2cadical_var[var]
    }

    #[inline]
    fn lit_to_cadical_lit(&self, lit: &Lit) -> i32 {
        let mut res = usize::from(self.var_to_cadical_var(lit.var())) as i32 + 1;
        if !lit.polarity() {
            res = -res;
        }
        res
    }

    #[inline]
    fn cadical_var_to_var(&self, cadical_var: Var) -> Option<Var> {
        self.cadical_var2var
            .get(usize::from(cadical_var))
            .and_then(|var| *var)
    }

    #[inline]
    fn cadical_lit_to_lit(&self, lit: i32) -> Option<Lit> {
        let p = lit > 0;
        let cadical_var = Var::new(lit.unsigned_abs() as usize - 1);
        self.cadical_var_to_var(cadical_var)
            .map(|var| Lit::new(var, p))
    }
}

impl Satif for CaDiCaL {
    #[inline]
    fn new_var(&mut self) -> Var {
        let cadical_var = unsafe { cadical_solver_new_var(self.solver) };
        assert!(cadical_var > 0);
        let cadical_var = Var::new(cadical_var as usize - 1);
        let var = Var::new(self.num_var);

        self.var2cadical_var.reserve(var);
        self.var2cadical_var[var] = cadical_var;

        self.cadical_var2var.reserve(cadical_var);
        debug_assert!(self.cadical_var2var[cadical_var].is_none());
        self.cadical_var2var[cadical_var] = Some(var);

        self.num_var += 1;
        var
    }

    #[inline]
    fn num_var(&self) -> usize {
        self.num_var
    }

    #[inline]
    fn add_clause(&mut self, clause: &[Lit]) {
        let clause: Vec<i32> = clause
            .iter()
            .map(|lit| self.lit_to_cadical_lit(lit))
            .collect();
        unsafe { cadical_solver_add_clause(self.solver, clause.as_ptr() as _, clause.len() as _) }
    }

    fn solve(&mut self, assumps: &[Lit]) -> bool {
        let assumps: Vec<i32> = assumps
            .iter()
            .map(|lit| self.lit_to_cadical_lit(lit))
            .collect();
        match unsafe {
            cadical_solver_solve(self.solver, assumps.as_ptr() as _, assumps.len() as _)
        } {
            10 => true,
            20 => false,
            _ => todo!(),
        }
    }

    fn solve_with_constraint(&mut self, assumps: &[Lit], constraint: Vec<LitVec>) -> bool {
        self.try_solve(assumps, constraint).unwrap()
    }

    fn try_solve(&mut self, assumps: &[Lit], constraint: Vec<LitVec>) -> Option<bool> {
        if constraint.len() > 1 {
            panic!("cadical does not support multiple temporary constraints");
        }
        let assumps: Vec<i32> = assumps
            .iter()
            .map(|lit| self.lit_to_cadical_lit(lit))
            .collect();
        if !constraint.is_empty() {
            let constraint: Vec<i32> = constraint[0]
                .iter()
                .map(|lit| self.lit_to_cadical_lit(lit))
                .collect();
            unsafe {
                cadical_solver_constrain(
                    self.solver,
                    constraint.as_ptr() as _,
                    constraint.len() as _,
                )
            }
        };
        match unsafe {
            cadical_solver_solve(self.solver, assumps.as_ptr() as _, assumps.len() as _)
        } {
            10 => Some(true),
            20 => Some(false),
            _ => None,
        }
    }

    fn sat_value(&self, lit: Lit) -> Option<bool> {
        let lit = self.lit_to_cadical_lit(&lit);
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
        let lit = self.lit_to_cadical_lit(&lit);
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
        unsafe { cadical_solver_freeze(self.solver, self.lit_to_cadical_lit(&var.lit())) }
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
                    cnf.push(LitVec::from_iter(cls.into_iter().map(|lit| {
                        self.cadical_lit_to_lit(lit).unwrap_or_else(|| {
                            panic!("cadical clause contains unmapped variable: {lit}")
                        })
                    })));
                }
            }
        }
        cnf
    }

    fn set_seed(&mut self, seed: u64) {
        unsafe { cadical_set_seed(self.solver, seed as _) }
    }

    fn get_stop_ctrl(&mut self) -> Box<dyn StopCtrl> {
        Box::new(CaDiCaLStopCtrl {
            solver: self.solver,
        })
    }
}

impl CaDiCaL {
    pub fn set_polarity(&mut self, var: Var, pol: Option<bool>) {
        match pol {
            Some(p) => {
                let p = var.lit().not_if(!p);
                unsafe { cadical_set_polarity(self.solver, self.lit_to_cadical_lit(&p)) }
            }
            None => unsafe {
                cadical_unset_polarity(self.solver, self.lit_to_cadical_lit(&var.lit()))
            },
        };
    }
}

impl Drop for CaDiCaL {
    fn drop(&mut self) {
        unsafe { cadical_solver_free(self.solver) };
    }
}

impl Default for CaDiCaL {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Sync for CaDiCaL {}

unsafe impl Send for CaDiCaL {}

struct CaDiCaLStopCtrl {
    solver: *mut c_void,
}

impl StopCtrl for CaDiCaLStopCtrl {
    fn stop(&mut self) {
        unsafe { cadical_terminate(self.solver) }
    }
}

#[test]
fn test() {
    use logicrs::LitVec;
    let mut solver = CaDiCaL::new();
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
    assert!(!solver.solve_with_constraint(&[lit2], vec![LitVec::from([!lit0])]));
    assert!(solver.unsat_has(lit2));
}
