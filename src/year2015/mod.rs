pub mod day14;
pub mod day25;

static YEAR: &str = "2015";

pub fn run_all()
{
    let start = super::begin();

    day14::solve();
    day25::solve();

    super::end(start, YEAR);
}
