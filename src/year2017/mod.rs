pub mod day01;
pub mod day02;
pub mod day04;
pub mod day05;
pub mod day06;

static YEAR: &str = "2017";

pub fn run_all()
{
    let start = super::begin();

    day01::solve();
    day02::solve();
    day04::solve();
    day05::solve();
    day06::solve();

    super::end(start, YEAR);
}
