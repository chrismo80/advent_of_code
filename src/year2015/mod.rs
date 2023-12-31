pub mod day01;
pub mod day02;
pub mod day03;
pub mod day05;
pub mod day06;
pub mod day09;
pub mod day10;
pub mod day14;
pub mod day16;
pub mod day25;

static YEAR: &str = "2015";

pub fn run_all()
{
    let start = super::begin();

    day01::solve();
    day02::solve();
    day03::solve();
    day05::solve();
    day06::solve();
    day09::solve();
    day10::solve();
    day14::solve();
    day16::solve();
    day25::solve();

    super::end(start, YEAR);
}
