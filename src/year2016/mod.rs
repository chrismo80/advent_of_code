pub use crate::extensions::parallel_foreach::*;
pub use crate::path_finding::grid::*;

pub mod day24;

pub fn run()
{
    let start = begin();

    for _ in 0..1 {
        day24::solve();
    }

    end(start);
}

pub fn run_all()
{
    let start = begin();

    day24::solve();

    end(start);
}

fn begin() -> std::time::Instant
{
    println!("\n\nDay\tPart 1\t\tPart 2\n---------------------------------------");

    std::time::Instant::now()
}

fn end(start: std::time::Instant)
{
    println!(
        "---------------------------------------\nYear 2016: {:.1} ms\n\n",
        start.elapsed().as_micros() as f32 / 1000.0
    );
}
