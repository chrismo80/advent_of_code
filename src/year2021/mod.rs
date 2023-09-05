pub mod day01;
pub mod day02;

static YEAR: &str = "2021";

pub fn run_all()
{
    let start = super::begin();

    day01::solve();
    day02::solve();

    super::end(start, YEAR);
}
