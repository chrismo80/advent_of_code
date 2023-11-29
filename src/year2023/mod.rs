pub mod day01;

static YEAR: &str = "2023";

pub fn run_all()
{
    let start = super::begin();

    day01::solve();

    super::end(start, YEAR);
}
