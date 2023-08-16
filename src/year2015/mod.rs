pub mod day25;

static YEAR: &str = "2015";

pub fn run_all()
{
    let start = super::begin();

    day25::solve();

    super::end(start, YEAR);
}
