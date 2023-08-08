pub mod examples;
pub mod extensions;
pub mod year2022;

pub fn run()
{
    let start = begin();

    year2022::day5::solve();

    end(start);
}

pub fn run_all()
{
    let start = begin();

    year2022::day1::solve();
    year2022::day2::solve();
    year2022::day3::solve();
    year2022::day4::solve();
    year2022::day5::solve();
    year2022::day6::solve();
    year2022::day7::solve();
    year2022::day8::solve();

    end(start);
}

fn begin() -> std::time::Instant
{
    println!("Day\tPart 1\tPart 2");

    std::time::Instant::now()
}

fn end(start: std::time::Instant)
{
    println!("Duration: {:.1} ms", start.elapsed().as_micros() as f32 / 1000.0);
}
