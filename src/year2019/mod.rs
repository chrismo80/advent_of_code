pub mod day02;

static YEAR: &str = "2019";

pub fn run_all()
{
    let start = super::begin();

    day02::solve();

    super::end(start, YEAR);
}
