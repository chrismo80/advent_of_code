pub mod examples;
pub mod extensions;
pub mod year2022;

fn main()
{
    run_aoc();

    examples::process_items::main();
}

fn run_aoc()
{
    let times = 1;
    let start = std::time::Instant::now();

    for _ in 0..times {
        year2022::day1::solve();
        year2022::day2::solve();
    }

    println!("Duration: {:.1} ms", start.elapsed().as_micros() as f32 / 1000.0);
}
