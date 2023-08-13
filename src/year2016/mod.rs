pub mod day24;

static YEAR: &str = "2016";

pub fn run()
{
    let start = super::begin();

    for _ in 0..1 {
        day24::solve();
    }

    super::end(start, YEAR);
}

pub fn run_all()
{
    let start = super::begin();

    day24::solve();

    super::end(start, YEAR);
}
