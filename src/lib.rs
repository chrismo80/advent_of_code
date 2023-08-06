pub mod examples;
pub mod extensions;
pub mod year2022;

pub fn run()
{
    let start = std::time::Instant::now();

    println!("Day\tPart 1\tPart 2");

    year2022::day2::solve();

    println!("Duration: {:.1} ms", start.elapsed().as_micros() as f32 / 1000.0);
}

pub fn run_all()
{
    println!("Day\tPart 1\tPart 2");

    let start = std::time::Instant::now();

    year2022::day1::solve();
    year2022::day2::solve();

    println!("Duration: {:.1} ms", start.elapsed().as_micros() as f32 / 1000.0);
}
