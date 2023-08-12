pub mod examples;
pub mod extensions;
pub mod path_finding;
pub mod year2022;

pub fn run()
{
    let start = begin();

    for _ in 0..1 {
        year2022::day12::solve();
    }

    end(start);
}

pub fn run_all()
{
    let start = begin();

    year2022::day01::solve();
    year2022::day02::solve();
    year2022::day03::solve();
    year2022::day04::solve();
    year2022::day05::solve();
    year2022::day06::solve();
    year2022::day07::solve();
    year2022::day08::solve();
    year2022::day09::solve();
    year2022::day10::solve();
    year2022::day11::solve();
    year2022::day12::solve();

    end(start);
}

fn begin() -> std::time::Instant
{
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
