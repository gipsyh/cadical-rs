cmake_minimum_required(VERSION 3.10)
project(for-rIC3 LANGUAGES CXX)
set(CMAKE_CXX_FLAGS_RELEASE "-O3")

set(S cadical/src/)
add_library(satif-cadical
  ${S}analyze.cpp ${S}arena.cpp ${S}assume.cpp ${S}averages.cpp
  ${S}backtrack.cpp ${S}backward.cpp ${S}bins.cpp ${S}block.cpp
  ${S}ccadical.cpp ${S}checker.cpp ${S}clause.cpp ${S}collect.cpp
  ${S}compact.cpp ${S}condition.cpp ${S}config.cpp ${S}constrain.cpp
  ${S}contract.cpp ${S}cover.cpp ${S}decide.cpp ${S}decompose.cpp
  ${S}deduplicate.cpp ${S}drattracer.cpp ${S}elim.cpp ${S}ema.cpp
  ${S}extend.cpp ${S}external.cpp ${S}external_propagate.cpp
  ${S}flags.cpp ${S}flip.cpp ${S}format.cpp ${S}frattracer.cpp ${S}gates.cpp
  ${S}idruptracer.cpp ${S}instantiate.cpp ${S}internal.cpp ${S}ipasir.cpp
  ${S}lidruptracer.cpp ${S}limit.cpp ${S}logging.cpp ${S}lookahead.cpp
  ${S}lratbuilder.cpp ${S}lratchecker.cpp ${S}lrattracer.cpp ${S}lucky.cpp
  ${S}message.cpp ${S}minimize.cpp ${S}occs.cpp ${S}options.cpp ${S}parse.cpp
  ${S}phases.cpp ${S}probe.cpp ${S}profile.cpp ${S}proof.cpp ${S}propagate.cpp
  ${S}queue.cpp ${S}random.cpp ${S}reap.cpp ${S}reduce.cpp ${S}rephase.cpp
  ${S}report.cpp ${S}resources.cpp ${S}restart.cpp ${S}restore.cpp
  ${S}score.cpp ${S}shrink.cpp ${S}solution.cpp ${S}solver.cpp
  ${S}stats.cpp ${S}subsume.cpp ${S}ternary.cpp
  ${S}transred.cpp ${S}util.cpp ${S}var.cpp ${S}veripbtracer.cpp
  ${S}vivify.cpp ${S}walk.cpp ${S}watch.cpp
  cadical/contrib/craigtracer.cpp
  bindings/binding.cpp
  win-port/file.cpp win-port/terminal.cpp
)

target_compile_definitions(satif-cadical PRIVATE QUIET NDEBUG)
if(WIN32)
  target_compile_definitions(satif-cadical PRIVATE
    getc_unlocked=getc putc_unlocked=putc)
endif()
target_include_directories(satif-cadical PRIVATE ${S} cadical/contrib)
install(TARGETS satif-cadical ARCHIVE DESTINATION lib LIBRARY DESTINATION lib)
