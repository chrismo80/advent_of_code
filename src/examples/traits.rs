pub fn main()
{
    let pola = PhotoCamera {
        name: "Polaroid".to_string(),
        id: 1,
    };

    init_camera(&pola);
    pola.capture();

    let hollywood = VideoCamera {
        name: "Sony2000".to_string(),
        id: 2,
    };

    init_camera(&hollywood);
    hollywood.capture();

    make_movie(&hollywood, 2);
}

fn init_camera(camera: &impl Capturing)
{
    camera.init();
}

fn make_movie(camera: &impl Recording, duration: u64)
{
    camera.start();
    std::thread::sleep(std::time::Duration::from_secs(duration));
    camera.stop();
}

pub struct VideoCamera
{
    name: String,
    id: i32,
}

pub struct PhotoCamera
{
    name: String,
    id: i32,
}

trait Capturing
{
    fn init(&self)
    {
        println!("Initializing camera");
    }

    fn capture(&self);
}

trait Recording
{
    fn start(&self);
    fn stop(&self);
}

impl Capturing for PhotoCamera
{
    fn capture(&self)
    {
        println!("Capturing photo");
    }
}

impl Capturing for VideoCamera
{
    fn capture(&self)
    {
        println!("Capturing video frame");
    }
}

impl Recording for VideoCamera
{
    fn start(&self)
    {
        println!("Starting video recording");
    }

    fn stop(&self)
    {
        println!("Stopping video recording");
    }
}
