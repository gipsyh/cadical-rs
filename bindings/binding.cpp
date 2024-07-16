#include "cadical.hpp"
#include "craigtracer.hpp"
#include <cassert>
using namespace CaDiCaL;
#include "stdio.h"

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

int cadical_solver_fixed(void *s, int lit)
{
	Solver *slv = (Solver *)s;
	return slv->fixed(lit);
}

void cadical_solver_freeze(void *s, int lit)
{
	Solver *slv = (Solver *)s;
	slv->freeze(lit);
}

struct ClauseIter : ClauseIterator {
	bool clause(const std::vector<int> &c)
	{
		std::vector<int> *cls = new std::vector<int>;
		for (auto &lit : c)
			cls->push_back(lit);
		clauses->push_back(cls->data());
		clauses->push_back((void *)cls->size());
		return true;
	}

	std::vector<void *> *clauses;
};

void *cadical_solver_clauses(void *s, int *len)
{
	ClauseIter clause_iter;
	Solver *slv = (Solver *)s;
	clause_iter.clauses = new std::vector<void *>();
	slv->traverse_clauses(clause_iter);
	*len = clause_iter.clauses->size();
	return clause_iter.clauses->data();
}

void cadical_set_polarity(void *s, int lit)
{
	Solver *slv = (Solver *)s;
	slv->phase(lit);
}

void cadical_unset_polarity(void *s, int lit)
{
	Solver *slv = (Solver *)s;
	slv->unphase(lit);
}

void *cadical_craig_new(void *s)
{
	Solver *solver = (Solver *)s;
	CaDiCraig::CraigTracer *tracer = new CaDiCraig::CraigTracer();
	solver->connect_proof_tracer(tracer, true);
	tracer->set_craig_construction(CaDiCraig::CraigConstruction::ALL);
	return tracer;
}

void cadical_craig_free(void *s, void *c)
{
	Solver *solver = (Solver *)s;
	CaDiCraig::CraigTracer *tracer = (CaDiCraig::CraigTracer *)c;
	solver->disconnect_proof_tracer(tracer);
	delete tracer;
}

void cadical_craig_label_var(void *c, int var, uint8_t t)
{
	// printf("var_label: %d %d\n", var, t);
	CaDiCraig::CraigTracer *tracer = (CaDiCraig::CraigTracer *)c;
	tracer->label_variable(var, (CaDiCraig::CraigVarType)t);
}

void cadical_craig_label_clause(void *c, int id, uint8_t t)
{
	// printf("clause_label: %d %d\n", id, t);
	CaDiCraig::CraigTracer *tracer = (CaDiCraig::CraigTracer *)c;
	tracer->label_clause(id, (CaDiCraig::CraigClauseType)t);
}

void *cadical_craig_create_craig_interpolant(void *c, int *next_var, int *len)
{
	CaDiCraig::CraigTracer *tracer = (CaDiCraig::CraigTracer *)c;
	std::vector<std::vector<int> > clauses;
	CaDiCraig::CraigCnfType result =
		tracer->create_craig_interpolant(CaDiCraig::CraigInterpolant::ASYMMETRIC, clauses, *next_var);
	assert(result == CaDiCraig::CraigCnfType::NORMAL);

	std::vector<void *> *res = new std::vector<void *>();
	for (int i = 0; i < clauses.size(); ++i) {
		std::vector<int> *cls = new std::vector<int>;
		for (int j = 0; j < clauses[i].size(); ++j)
			cls->push_back(clauses[i][j]);
		res->push_back(cls->data());
		res->push_back((void *)cls->size());
	}
	*len = res->size();
	return res->data();
}

void cadical_craig_test()
{
	CaDiCaL::Solver *solver = new CaDiCaL::Solver();
	CaDiCraig::CraigTracer *tracer = (CaDiCraig::CraigTracer *)cadical_craig_new(solver);

	cadical_craig_label_var(tracer, 1, 0);
	cadical_craig_label_var(tracer, 2, 2);
	cadical_craig_label_var(tracer, 3, 2);
	cadical_craig_label_var(tracer, 4, 1);
	cadical_craig_label_clause(tracer, 1, 0);
	cadical_craig_label_clause(tracer, 2, 0);
	cadical_craig_label_clause(tracer, 3, 0);
	cadical_craig_label_clause(tracer, 4, 1);
	cadical_craig_label_clause(tracer, 5, 1);
	cadical_craig_label_clause(tracer, 6, 1);
	solver->add(1);
	solver->add(-2);
	solver->add(0);
	solver->add(-1);
	solver->add(-3);
	solver->add(0);
	solver->add(2);
	solver->add(0);
	solver->add(-2);
	solver->add(3);
	solver->add(0);
	solver->add(2);
	solver->add(4);
	solver->add(0);
	solver->add(-4);
	solver->add(0);
	assert(solver->solve() == CaDiCaL::Status::UNSATISFIABLE);

	int next_var = 5;
	std::vector<std::vector<int> > clauses;
	CaDiCraig::CraigCnfType result =
		tracer->create_craig_interpolant(CaDiCraig::CraigInterpolant::ASYMMETRIC, clauses, next_var);
	assert(result == CaDiCraig::CraigCnfType::NORMAL);
	printf("%d %d\n\n", clauses.size(), next_var);
	for (int i = 0; i < clauses.size(); ++i) {
		for (int j = 0; j < clauses[i].size(); ++j)
			printf("%d ", clauses[i][j]);
		printf("\n");
	}
	// assert (clauses == std::vector<std::vector<int>>{{-1}});
	// assert (next_var == 2);

	cadical_craig_free(solver, tracer);
	delete solver;
}
}
