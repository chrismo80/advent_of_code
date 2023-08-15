pub mod day02;
pub mod day04;
pub mod day13;

static YEAR: &str = "2020";

pub fn run_all()
{
    let start = super::begin();

    day02::solve();
    day04::solve();
    day13::solve();

    super::end(start, YEAR);
}
