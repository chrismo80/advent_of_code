pub fn solve() -> (usize, usize)
{
    let mut sequence = include_str!("input.txt").to_string();

    (0..40).for_each(|_| process(&mut sequence));
    let result1 = sequence.len();

    (0..10).for_each(|_| process(&mut sequence));
    let result2 = sequence.len();

    println!("10\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn process(sequence: &mut String)
{
    let mut builder = String::new();
    let mut number = sequence.chars().next().unwrap();
    let mut count = 0;

    for letter in sequence.chars() {
        if letter != number {
            builder.push(char::from_digit(count, 10).unwrap());
            builder.push(number);
            number = letter;
            count = 0;
        }

        count += 1;
    }

    builder.push(char::from_digit(count, 10).unwrap());
    builder.push(number);

    *sequence = builder;
}

#[test]
fn test()
{
    assert_eq!(solve(), (252594, 3579328));
}
