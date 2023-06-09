pub mod group;
pub mod helper;

pub fn print_from_lib() {
    println!("hello from lib");
    helper::print_from_helper();
    helper::print_again();
    group::g1::g1_hello();
}