use logic_form::{Lit, Var};
use std::{
    ffi::{c_int, c_void},
    fmt::{self, Debug},
    marker::PhantomData,
};

extern "C" {
    fn cadical_solver_new() -> *mut c_void;
    fn cadical_solver_free(s: *mut c_void);
    fn cadical_solver_add_clause(s: *mut c_void, clause: *mut c_int, len: c_int);
    fn cadical_solver_solve(s: *mut c_void, assumps: *mut c_int, len: c_int) -> c_int;
    fn cadical_solver_constrain(s: *mut c_void, constrain: *mut c_int, len: c_int);
    fn cadical_solver_simplify(s: *mut c_void);
    // fn solver_set_polarity(s: *mut c_void, var: c_int, pol: c_int);
    fn cadical_solver_model_value(s: *mut c_void, lit: c_int) -> c_int;
    fn cadical_solver_conflict_has(s: *mut c_void, lit: c_int) -> bool;
}

fn lit_to_cadical_lit(lit: &Lit) -> i32 {
    let mut res = Into::<usize>::into(lit.var()) as i32 + 1;
    if !lit.polarity() {
        res = -res;
    }
    res
}

pub struct Model<'a> {
    solver: *mut c_void,
    _pd: PhantomData<&'a ()>,
}

impl Model<'_> {
    pub fn lit_value(&self, lit: Lit) -> Option<bool> {
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

pub struct Conflict<'a> {
    solver: *mut c_void,
    _pd: PhantomData<&'a ()>,
}

impl Conflict<'_> {
    pub fn has(&self, lit: Lit) -> bool {
        let lit = lit_to_cadical_lit(&lit);
        unsafe { cadical_solver_conflict_has(self.solver, lit) }
    }
}

pub enum SatResult<'a> {
    Sat(Model<'a>),
    Unsat(Conflict<'a>),
}

impl Debug for SatResult<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sat(_) => "Sat".fmt(f),
            Self::Unsat(_) => "Unsat".fmt(f),
        }
    }
}

pub struct Solver {
    solver: *mut c_void,
    num_var: usize,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            solver: unsafe { cadical_solver_new() },
            num_var: 0,
        }
    }

    pub fn new_var(&mut self) -> Var {
        self.num_var += 1;
        Var::new(self.num_var - 1)
    }

    pub fn num_var(&self) -> usize {
        self.num_var
    }

    pub fn add_clause(&mut self, clause: &[Lit]) {
        let clause: Vec<i32> = clause.iter().map(lit_to_cadical_lit).collect();
        unsafe { cadical_solver_add_clause(self.solver, clause.as_ptr() as _, clause.len() as _) }
    }

    pub fn solve<'a>(&'a mut self, assumps: &[Lit]) -> SatResult<'a> {
        let assumps: Vec<i32> = assumps.iter().map(lit_to_cadical_lit).collect();
        match unsafe {
            cadical_solver_solve(self.solver, assumps.as_ptr() as _, assumps.len() as _)
        } {
            10 => SatResult::Sat(Model {
                solver: self.solver,
                _pd: PhantomData,
            }),
            20 => SatResult::Unsat(Conflict {
                solver: self.solver,
                _pd: PhantomData,
            }),
            _ => todo!(),
        }
    }

    pub fn solve_with_constrain<'a>(
        &'a mut self,
        assumps: &[Lit],
        constrain: &[Lit],
    ) -> SatResult<'a> {
        let constrain: Vec<i32> = constrain.iter().map(lit_to_cadical_lit).collect();
        unsafe {
            cadical_solver_constrain(self.solver, constrain.as_ptr() as _, constrain.len() as _)
        };
        self.solve(assumps)
    }

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
    pub unsafe fn get_model(&self) -> Model<'static> {
        Model {
            solver: self.solver,
            _pd: PhantomData,
        }
    }

    /// # Safety
    /// unsafe get unsat core
    pub unsafe fn get_conflict(&self) -> Conflict<'static> {
        Conflict {
            solver: self.solver,
            _pd: PhantomData,
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
    solver.simplify();
    match solver.solve_with_constrain(&[lit2], &[!lit0]) {
        SatResult::Sat(_) => panic!(),
        SatResult::Unsat(conflict) => {
            assert!(conflict.has(lit2));
        }
    }
    match solver.solve(&[lit2]) {
        SatResult::Sat(_) => {}
        SatResult::Unsat(_) => todo!(),
    }
}
