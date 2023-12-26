#include "cadical.hpp"
using namespace CaDiCaL;

extern "C" {
void *solver_new()
{
	return new Solver();
}

void solver_free(void *s)
{
	Solver *slv = (Solver *)s;
	delete slv;
}

void solver_add_clause(void *s, int *clause, int len)
{
	Solver *slv = (Solver *)s;
	for (int i = 0; i < len; ++i) {
		slv->add(clause[i]);
	}
	slv->add(0);
}

int solver_solve(void *s, int *assumps, int len)
{
	Solver *slv = (Solver *)s;
	for (int i = 0; i < len; ++i)
		slv->assume(assumps[i]);
	return slv->solve();
}

int solver_model_value(void *s, int lit)
{
	Solver *slv = (Solver *)s;
	return slv->val(lit);
}

bool solver_conflict_has(void *s, int lit)
{
	Solver *slv = (Solver *)s;
	return slv->failed(lit);
}

void solver_constrain(void *s, int *constrain, int len)
{
	Solver *slv = (Solver *)s;
	for (int i = 0; i < len; ++i) {
		slv->constrain(constrain[i]);
	}
	slv->constrain(0);
}

// bool solver_simplify(void *s)
// {
// 	BindingSolver *slv = s;
// 	return slv->simplify();
// }

// void solver_release_var(void *s, int lit)
// {
// 	BindingSolver *slv = s;
// 	slv->releaseVar(toLit(lit));
// }

// void solver_set_polarity(void *s, int var, int pol)
// {
// 	BindingSolver *slv = s;
// 	slv->setPolarity(var, toLbool(pol));
// }

// void solver_set_random_seed(void *s, double seed)
// {
// 	BindingSolver *slv = s;
// 	slv->random_seed = seed;
// }

// void solver_set_rnd_init_act(void *s, bool enable)
// {
// 	BindingSolver *slv = s;
// 	slv->rnd_init_act = enable;
// }
// }

// class BindingSimpSolver : public SimpSolver {
//     public:
// 	bool add_clause(int *clause, int len)
// 	{
// 		add_tmp.clear();
// 		add_tmp.growTo(len);
// 		Lit *cls = (Lit *)clause;
// 		for (int i = 0; i < len; ++i)
// 			add_tmp[i] = cls[i];
// 		return addClause_(add_tmp);
// 	}
};

// extern "C" {
// void *simp_solver_new()
// {
// 	return new BindingSimpSolver();
// }

// void simp_solver_free(void *s) {
// 	BindingSimpSolver *slv = s;
// 	delete slv;
// }

// int simp_solver_new_var(void *s)
// {
// 	BindingSimpSolver *slv = s;
// 	return slv->newVar();
// }

// int simp_solver_num_var(void *s)
// {
// 	BindingSimpSolver *slv = s;
// 	return slv->nVars();
// }

// bool simp_solver_add_clause(void *s, int *clause, int len)
// {
// 	BindingSimpSolver *slv = s;
// 	return slv->add_clause(clause, len);
// }

// void simp_solver_set_frozen(void *s, int var, bool frozen)
// {
// 	BindingSimpSolver *slv = s;
// 	slv->setFrozen(var, frozen);
// }

// bool simp_solver_eliminate(void *s, bool turn_off_elim)
// {
// 	BindingSimpSolver *slv = s;
// 	return slv->eliminate(turn_off_elim);
// }

// void *simp_solver_clauses(void *s, int *len)
// {
// 	BindingSimpSolver *slv = s;
// 	std::vector<void *> *clauses = new std::vector<void *>();
// 	for (Minisat::ClauseIterator c = slv->clausesBegin(); c != slv->clausesEnd(); ++c) {
// 		const Minisat::Clause &cls = *c;
// 		std::vector<Lit> *cls_ = new std::vector<Lit>;
// 		for (int i = 0; i < cls.size(); ++i)
// 			cls_->push_back(cls[i]);
// 		clauses->push_back(cls_->data());
// 		clauses->push_back((void *)cls_->size());
// 	}
// 	for (Minisat::TrailIterator c = slv->trailBegin(); c != slv->trailEnd(); ++c) {
// 		std::vector<Lit> *cls_ = new std::vector<Lit>;
// 		cls_->push_back(*c);
// 		clauses->push_back(cls_->data());
// 		clauses->push_back((void *)cls_->size());
// 	}
// 	*len = clauses->size();
// 	return clauses->data();
// }
// }
