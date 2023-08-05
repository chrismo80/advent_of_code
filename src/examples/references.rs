pub fn main()
{
    let mut test = vec![1, 2, 3, 4, 5];

    // immutable reference
    for i in &test {
        println!("{i}");
    }

    // mutable reference
    for i in &mut test {
        *i += 10;
    }

    for i in &test {
        println!("{i}");
    }
}
