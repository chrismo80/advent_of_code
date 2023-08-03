use chrono::*;

pub fn main()
{
    let mut p = Person::new(String::from("Max Mustermann"), 1986);
    println!("{:?}, age: {}", p, p.get_age());

    p.go_to_work();
    println!("{:?}, age: {}", p, p.get_age());

    p.go_home();
    println!("{:?}, age: {}", p, p.get_age());

    p.rename(String::from("Michael Mustermann"));
    println!("{:?}, age: {}", p, p.get_age());

    let myself = Person::moldi();
    println!("{:?}, age: {}", myself, myself.get_age());
}

#[derive(Debug)]
pub struct Person
{
    name: String,
    birth_year: i32,
    working: bool,
}

impl Person
{
    pub fn new(name: String, birth_year: i32) -> Self
    {
        Person {
            name,
            birth_year,
            working: false,
        }
    }

    pub fn moldi() -> Person
    {
        Person::new(String::from("Moldi"), 1980)
    }
}

impl Person
{
    pub fn get_age(&self) -> i32
    {
        chrono::Local::now().year() - self.birth_year
    }

    pub fn rename(&mut self, name: String)
    {
        self.name = name;
    }

    pub fn go_to_work(&mut self)
    {
        self.working = true;
    }

    pub fn go_home(&mut self)
    {
        self.working = false;
    }
}
