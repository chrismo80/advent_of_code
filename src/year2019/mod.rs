pub mod day02;
pub mod day05;
pub mod day07;
pub mod day09;
pub mod day11;
pub mod day13;
pub mod intcode_computer;

static YEAR: &str = "2019";

pub fn run_all()
{
    let start = super::begin();

    day02::solve();
    day05::solve();
    day07::solve();
    day09::solve();
    day11::solve();
    day13::solve();

    super::end(start, YEAR);
}
