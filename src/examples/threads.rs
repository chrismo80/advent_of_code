pub fn main()
{
    let mut timer = Timer::new(10);

    timer.start();

    while timer.is_running() {
        println!("Running");
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    println!("Done");
}

pub struct Timer
{
    handle: Option<std::thread::JoinHandle<()>>,
    duration: usize,
}

impl Timer
{
    pub fn new(duration: usize) -> Timer
    {
        Timer { handle: None, duration }
    }

    pub fn start(&mut self)
    {
        let duration = self.duration;

        self.handle = Some(std::thread::spawn(move || {
            Self::run(duration);
        }))
    }

    pub fn is_running(&self) -> bool
    {
        match &self.handle {
            Some(handle) => !handle.is_finished(),
            None => false,
        }
    }

    fn run(mut duration: usize)
    {
        while duration > 0 {
            println!("Counting down: {}", duration);
            duration -= 1;
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }
}
