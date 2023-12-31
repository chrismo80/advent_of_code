pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day06;
pub mod day13;
pub mod day15;
pub mod day16;
pub mod day20;
pub mod day24;

static YEAR: &str = "2016";

pub fn run_all()
{
    let start = super::begin();

    day01::solve();
    day02::solve();
    day03::solve();
    day04::solve();
    day06::solve();
    day13::solve();
    day15::solve();
    day16::solve();
    day20::solve();
    day24::solve();

    super::end(start, YEAR);
}
