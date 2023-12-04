pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

static YEAR: &str = "2023";

pub fn run_all()
{
    let start = super::begin();

    day01::solve();
    day02::solve();
    day03::solve();
    day04::solve();

    super::end(start, YEAR);
}
