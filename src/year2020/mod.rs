pub mod day13;

static YEAR: &str = "2020";

pub fn run_all()
{
    let start = super::begin();

    day13::solve();

    super::end(start, YEAR);
}
