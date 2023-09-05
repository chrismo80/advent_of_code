pub mod day01;
pub mod day02;
pub mod day06;
pub mod day07;

static YEAR: &str = "2021";

pub fn run_all()
{
    let start = super::begin();

    day01::solve();
    day02::solve();
    day06::solve();
    day07::solve();

    super::end(start, YEAR);
}
