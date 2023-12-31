#![allow(dead_code)]
pub mod extensions;
pub mod path_finding;
pub mod year2015;
pub mod year2016;
pub mod year2017;
pub mod year2018;
pub mod year2019;
pub mod year2020;
pub mod year2021;
pub mod year2022;
pub mod year2023;

pub fn run()
{
    let start = std::time::Instant::now();

    run_all();

    end(start, "Total");
}

fn run_all()
{
    year2015::run_all();
    year2016::run_all();
    year2017::run_all();
    year2018::run_all();
    year2019::run_all();
    year2020::run_all();
    year2021::run_all();
    year2022::run_all();
    year2023::run_all();
}

fn begin() -> std::time::Instant
{
    println!("\n\nDay\tPart 1\t\t\tPart 2");
    print_line();
    std::time::Instant::now()
}

fn end(start: std::time::Instant, scope: &str)
{
    print_line();

    println!("{scope}: {:.1} ms\n\n", start.elapsed().as_micros() as f32 / 1000.0);
}

fn print_line()
{
    println!("--------------------------------------------------------------");
}

fn start_watch() -> std::time::Instant
{
    std::time::Instant::now()
}

fn stop_watch(start: std::time::Instant) -> std::time::Instant
{
    println!("{:.1} ms\n\n", start.elapsed().as_micros() as f32 / 1000.0);
    std::time::Instant::now()
}
