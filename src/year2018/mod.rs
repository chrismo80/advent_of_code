pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day08;
pub mod day09;
pub mod day10;

static YEAR: &str = "2018";

pub fn run_all()
{
    let start = super::begin();

    day01::solve();
    day02::solve();
    day03::solve();
    day04::solve();
    day05::solve();
    day06::solve();
    day08::solve();
    day09::solve();
    day10::solve();

    super::end(start, YEAR);
}
