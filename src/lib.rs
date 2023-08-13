pub mod examples;
pub mod extensions;
pub mod path_finding;
pub mod year2016;
pub mod year2022;

pub fn run()
{
    let start = std::time::Instant::now();

    year2016::run_all();
    year2022::run_all();

    end(start, "Total");
}

fn begin() -> std::time::Instant
{
    println!("\n\nDay\tPart 1\t\tPart 2");
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
    println!("---------------------------------------");
}
