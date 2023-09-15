pub fn solve() -> (String, String)
{
    let input = include_str!("input.txt");

    let result1 = get_check_sum_faster(input, 272);
    let result2 = get_check_sum_faster(input, 35651584);

    println!("16\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn get_check_sum_faster(data: &str, disk_length: usize) -> String
{
    let mut bits = vec![false; disk_length * 2];

    for (i, c) in data.chars().enumerate() {
        bits[i] = c == '1';
    }

    let mut length = data.len();

    while length < disk_length {
        for i in 0..length {
            bits[length + 1 + i] = !bits[length - 1 - i];
        }
        length *= 2;
        length += 1;
    }

    length = disk_length;

    while length % 2 == 0 {
        for i in (0..length).step_by(2) {
            bits[i / 2] = bits[i] == bits[i + 1];
        }
        length /= 2;
    }

    let mut result = String::from("");

    for &bit in bits.iter().take(length) {
        result.push(char::from_digit(bit as u32, 2).unwrap());
    }

    result
}

fn get_check_sum(data: &str, disk_length: usize) -> String
{
    let mut bits: Vec<bool> = data.chars().map(|c| c == '1').collect();

    while bits.len() < disk_length {
        bits.push(false);
        bits.extend(bits.iter().rev().skip(1).map(|b| !b).collect::<Vec<bool>>());
    }

    bits = bits[..disk_length].to_vec();

    while bits.len() % 2 == 0 {
        bits = bits.chunks(2).map(|pair| pair[0] == pair[1]).collect();
    }

    bits.iter()
        .map(|b| char::from_digit(*b as u32, 2).unwrap())
        .collect::<String>()
}

#[test]
fn test()
{
    assert_eq!(solve(), ("10100011010101011".to_string(), "01010001101011001".to_string()));
}
