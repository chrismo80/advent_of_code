mod common;

#[test]
#[ignore]
fn do_it()
{
    common::setup();

    advent_of_code::examples::macros::main();
    advent_of_code::examples::structs::main();
    advent_of_code::examples::count_items::main();
    advent_of_code::examples::errors::main();
}
