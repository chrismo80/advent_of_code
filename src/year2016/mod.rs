pub mod day01;
pub mod day02;
pub mod day24;

static YEAR: &str = "2016";

pub fn run()
{
    let start = super::begin();

    for _ in 0..1 {
        day02::solve();
    }

    super::end(start, YEAR);
}

pub fn run_all()
{
    let start = super::begin();

    day01::solve();
    day02::solve();
    day24::solve();

    super::end(start, YEAR);
}
