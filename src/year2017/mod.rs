pub mod day01;
pub mod day02;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day12;
pub mod day19;

static YEAR: &str = "2017";

pub fn run_all()
{
    let start = super::begin();

    day01::solve();
    day02::solve();
    day04::solve();
    day05::solve();
    day06::solve();
    day07::solve();
    day12::solve();
    day19::solve();

    super::end(start, YEAR);
}
