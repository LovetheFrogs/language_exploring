mod hello_world;
mod guessing_game;
mod functions;
mod flow_control;
mod ownership;
mod structs;
mod enums;
mod match_control_flow;
mod if_let_control_flow;
mod collections;
mod error_handling;
mod generics_traits_lifetimes;
mod testing;
mod iterators_closures;
mod cargo_crates;
mod smart_pointers;
mod concurency_threads;
mod object_oriented_rust;
mod patterns_matching;

/** Function add_two present in src/testing/mod.rs moved here for usage in tests/integration tests.
 */
pub fn add_two(a: i32) -> i32 {
    a + 2
}