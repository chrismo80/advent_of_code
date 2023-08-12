pub use crate::extensions::parallel_foreach::*;
pub use crate::path_finding::grid::*;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day14;
pub mod day25;

pub fn run()
{
    let start = begin();

    for _ in 0..1 {
        day14::solve();
    }

    end(start);
}

pub fn run_all()
{
    let start = begin();

    day01::solve();
    day02::solve();
    day03::solve();
    day04::solve();
    day05::solve();
    day06::solve();
    day07::solve();
    day08::solve();
    day09::solve();
    day10::solve();
    day11::solve();
    day12::solve();
    day14::solve();
    day25::solve();

    end(start);
}

fn begin() -> std::time::Instant
{
    println!("Advent of code 2022\n");
    println!("Day\tPart 1\t\tPart 2\n---------------------------------------");

    std::time::Instant::now()
}

fn end(start: std::time::Instant)
{
    println!(
        "---------------------------------------\nDuration: {:.1} ms",
        start.elapsed().as_micros() as f32 / 1000.0
    );
}
