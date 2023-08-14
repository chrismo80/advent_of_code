pub fn solve() -> (String, String)
{
    let input = include_str!("input.txt");

    let result1 = get_check_sum(input, 272);
    let result2 = get_check_sum(input, 35651584);

    println!("16\t{result1:<20}\t{result2:<20}");

    (result1, result2)
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

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(
            super::solve(),
            ("10100011010101011".to_string(), "01010001101011001".to_string())
        );
    }
}
