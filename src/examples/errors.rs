type Result<T> = std::result::Result<T, MyError>;

#[derive(Debug)]
struct MyError
{
    message: String,
}

pub fn main()
{
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
    println!("7: {:?}", do_the_thing_short(7));
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

    return Ok(i);
}

fn do_the_thing_short(i: i32) -> Result<i32>
{
    let i = halves_if_even(i)?.pow(2);

    Ok(i)
}
