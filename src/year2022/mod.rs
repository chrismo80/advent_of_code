pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day14;
pub mod day25;

static YEAR: &str = "2022";

pub fn run()
{
    let start = super::begin();

    for _ in 0..1 {
        day14::solve();
    }

    super::end(start, YEAR);
}

pub fn run_all()
{
    let start = super::begin();

    day01::solve();
    day02::solve();
    day03::solve();
    day04::solve();
    day05::solve();
    day06::solve();
    day07::solve();
    day08::solve();
    day09::solve();
    day10::solve();
    day11::solve();
    day12::solve();
    day14::solve();
    day25::solve();

    super::end(start, YEAR);
}
