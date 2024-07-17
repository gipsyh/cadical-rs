use cadical::{itp::Interpolant, Solver};
use logic_form::{Lit, Var};
use satif::Satif;

fn main() {
    // struct CaDiCaLTracer;
    // impl Tracer for CaDiCaLTracer {
    //     fn add_original_clause(&mut self, id: usize, redundant: bool, c: &[Lit], restore: bool) {
    //         dbg!("add", id);
    //         println!("add o {:?}", c);
    //     }

    //     fn add_derived_clause(&mut self, _id: usize, _redundant: bool, c: &[Lit], p: &[usize]) {
    //         println!("add d{:?}", c);
    //         println!("add d p{:?}", p);
    //     }

    //     fn delete_clause(&mut self, id: usize, _redundant: bool, _c: &[Lit]) {
    //         dbg!("delete");
    //     }

    //     fn conclude_unsat(&mut self, conclusion: i32, p: &[usize]) {
    //         dbg!("aaaaa");
    //     }
    // }
    let mut itp = Box::new(Interpolant::new());
    let mut solver = Solver::new();
    solver.connect_tracer(&itp);
    // let cnf = from_dimacs_file("./counter.cnf");
    // for c in cnf {
    //     s.add_clause(&c);
    // }
    itp.label_clause(true);
    solver.add_clause(&[Lit::new(Var(1), false), Lit::new(Var(2), true)]);
    itp.label_clause(true);
    solver.add_clause(&[Lit::new(Var(1), true), Lit::new(Var(3), true)]);
    itp.label_clause(true);
    solver.add_clause(&[Lit::new(Var(3), false)]);
    itp.label_clause(false);
    solver.add_clause(&[Lit::new(Var(0), true), Lit::new(Var(1), false)]);
    itp.label_clause(false);
    solver.add_clause(&[Lit::new(Var(0), false), Lit::new(Var(2), false)]);
    itp.label_clause(false);
    solver.add_clause(&[Lit::new(Var(1), true)]);
    dbg!(solver.solve(&[]));
}
