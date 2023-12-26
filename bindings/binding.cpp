#include "cadical.hpp"
using namespace CaDiCaL;

extern "C" {
void *cadical_solver_new()
{
	return new Solver();
}

void cadical_solver_free(void *s)
{
	Solver *slv = (Solver *)s;
	delete slv;
}

void cadical_solver_add_clause(void *s, int *clause, int len)
{
	Solver *slv = (Solver *)s;
	for (int i = 0; i < len; ++i) {
		slv->add(clause[i]);
	}
	slv->add(0);
}

int cadical_solver_solve(void *s, int *assumps, int len)
{
	Solver *slv = (Solver *)s;
	for (int i = 0; i < len; ++i)
		slv->assume(assumps[i]);
	return slv->solve();
}

int cadical_solver_model_value(void *s, int lit)
{
	Solver *slv = (Solver *)s;
	return slv->val(lit);
}

bool cadical_solver_conflict_has(void *s, int lit)
{
	Solver *slv = (Solver *)s;
	return slv->failed(lit);
}

void cadical_solver_constrain(void *s, int *constrain, int len)
{
	Solver *slv = (Solver *)s;
	for (int i = 0; i < len; ++i) {
		slv->constrain(constrain[i]);
	}
	slv->constrain(0);
}

int cadical_solver_simplify(void *s)
{
	Solver *slv = (Solver *)s;
	return slv->simplify();
}
}
