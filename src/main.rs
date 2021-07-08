extern crate lib_demo;
mod enums;
mod error_handling;
mod generics;
mod lifetime;
mod ownership;
mod stack_heap;
mod structs;
mod traits;
mod unit_test;
mod vars;

fn main() {
    println!("Hello, world!");
    vars::run();
    stack_heap::run();
    ownership::run();
    generics::run();
    lifetime::run();
    structs::run();
    enums::run();
    traits::run();
    error_handling::run();
    vars::sub_a::func_a();
    vars::sub_b::func_b();
    lib_demo::print_random_number();
}
