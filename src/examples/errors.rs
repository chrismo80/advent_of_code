type Result<T> = std::result::Result<T, MyError>;

#[derive(Debug)]
struct MyError
{
    message: String,
}

pub fn main()
{
    let numbers = vec![1, 2, 3, 4, 5];

    let mut colors: Vec<&str> = Vec::new();
    colors.push("Red");
    colors.push("Green");
    colors.push("Blue");

    let mut cars: Vec<String> = Vec::new();
    cars.push(String::from("Ford"));
    cars.push(String::from("Audi"));
    cars.push(String::from("Tesla"));

    println!("item {} is {}", 3, get_item_int(&numbers, 3));
    println!("item {} is {}", 30, get_item_int(&numbers, 30));
    println!("item {} is {}", 3, numbers.get(3).unwrap_or(&i32::default()));
    println!("item {} is {}", 30, numbers.get(30).unwrap_or(&0));

    println!("item {} is {}", 2, get_item_str(&colors, 2));
    println!("item {} is {}", 20, get_item_str(&colors, 20));
    println!("item {} is {}", 2, colors.get(2).unwrap_or(&"White"));
    println!("item {} is {}", 20, colors.get(20).unwrap_or(&"White"));

    println!("item {} is {}", 2, get_item_string(&cars, 2));
    println!("item {} is {}", 20, get_item_string(&cars, 20));
    println!("item {} is {}", 2, cars.get(2).unwrap_or(&String::from("Fiat")));
    println!("item {} is {}", 20, cars.get(20).unwrap_or(&String::from("Fiat")));

    println!("2: {:?}", do_the_thing(2));
    println!("3: {:?}", do_the_thing(3));
    println!("4: {:?}", do_the_thing(4));
    println!("5: {:?}", do_the_thing(5));
    println!("6: {:?}", do_the_thing(6));
    println!("7: {:?}", do_the_thing(7));

    println!("2: {:?}", do_the_thing_short(2));
    println!("3: {:?}", do_the_thing_short(3));
    println!("4: {:?}", do_the_thing_short(4));
    println!("5: {:?}", do_the_thing_short(5));
    println!("6: {:?}", do_the_thing_short(6));

    let value = match do_the_thing_short(7) {
        Ok(v) => v,
        Err(e) => {
            println!("Error: {}", e.message);
            0
        }
    };

    println!("7: {:?}", value);
}

fn get_item_int<'a>(v: &'a Vec<i32>, pos: usize) -> &'a i32
{
    match v.get(pos) {
        Some(r) => r,
        None => &0,
    }
}

fn get_item_str<'a>(v: &'a Vec<&str>, pos: usize) -> &'a str
{
    match v.get(pos) {
        Some(r) => r,
        None => "",
    }
}

fn get_item_string<'a>(v: &'a Vec<String>, pos: usize) -> &'a String
{
    match v.get(pos) {
        Some(r) => r,
        None => v.get(0).unwrap(),
    }
}

fn halves_if_even(i: i32) -> Result<i32>
{
    if i % 2 == 0 {
        Ok(i / 2)
    }
    else {
        Err(MyError {
            message: format!("{i} is not even"),
        })
    }
}

fn do_the_thing(i: i32) -> Result<i32>
{
    let i = match halves_if_even(i) {
        Ok(i) => i.pow(2),
        Err(e) => return Err(e),
    };

    Ok(i)
}

fn do_the_thing_short(i: i32) -> Result<i32>
{
    let i = halves_if_even(i)?.pow(2);

    Ok(i)
}
