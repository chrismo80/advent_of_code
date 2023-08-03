use chrono::*;

pub fn main()
{
    let mut moldi = Person::new(String::from("Moldi"), 1980);
    println!("{:?}", moldi);

    moldi.go_to_work();
    println!("{:?}", moldi);

    moldi.go_home();
    println!("{:?}", moldi);

    moldi.rename(String::from("Verena"));
    println!("{:?}", moldi);

    let myself = Person::moldi();
    println!("{:?}", myself);
}

#[derive(Debug)]
struct Person
{
    name: String,
    birth_year: i32,
    age: i32,
    working: bool,
}

impl Person
{
    fn new(name: String, birth_year: i32) -> Self
    {
        Person {
            name,
            birth_year,
            age: chrono::Utc::now().year() - birth_year,
            working: false,
        }
    }

    fn moldi() -> Person
    {
        Person::new(String::from("Moldi"), 1980)
    }
}

impl Person
{
    fn rename(&mut self, name: String)
    {
        self.name = name;
    }

    fn go_to_work(&mut self)
    {
        self.working = true;
    }

    fn go_home(&mut self)
    {
        self.working = false;
    }
}
