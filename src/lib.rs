pub mod examples;
pub mod extensions;
pub mod path_finding;
pub mod year2016;
pub mod year2022;

pub fn run()
{
    year2016::run_all();
    year2022::run_all();
}
