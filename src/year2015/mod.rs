pub mod day09;
pub mod day14;
pub mod day16;
pub mod day25;

static YEAR: &str = "2015";

pub fn run_all()
{
    let start = super::begin();

    day09::solve();
    day14::solve();
    day16::solve();
    day25::solve();

    super::end(start, YEAR);
}
