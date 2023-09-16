pub mod day01;
pub mod day02;
pub mod day04;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day13;

static YEAR: &str = "2021";

pub fn run_all()
{
    let start = super::begin();

    day01::solve();
    day02::solve();
    day04::solve();
    day06::solve();
    day07::solve();
    day08::solve();
    day13::solve();

    super::end(start, YEAR);
}
