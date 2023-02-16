/** Integration tests are placed on a diferent folder called `tests` located in the same level as the src folder. Inside, you create
 *  your files for integrated tests like this one. Note that each file in this directory is its own crate. To run this file in
 *  particular, use the `cargo test --test integration_test` command. Also, only projects with a lib.rs file can use this tests/ 
 *  folder function, as this library exposes functions that other crates can use. 
 */
use rust::add_two;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, add_two(2));
}