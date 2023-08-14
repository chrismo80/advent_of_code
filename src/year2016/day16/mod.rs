//use iter_tools::Itertools;

pub fn solve() -> (String, String)
{
    let input = include_str!("input.txt");

    let result1 = get_check_sum(input.to_string(), 272);
    let result2 = get_check_sum(input.to_string(), 35651584);

    println!("16\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn get_check_sum(data: String, disk_length: usize) -> String
{
    let mut digits: Vec<bool> = data.chars().map(|c| c == '1').collect();

    while digits.len() < disk_length {
        digits.push(false);
        digits.extend(digits.iter().rev().skip(1).map(|b| !b).collect::<Vec<bool>>());
    }

    digits = digits[..disk_length].to_vec();

    while digits.len() % 2 == 0 {
        digits = digits.chunks(2).map(|pair| pair[0] == pair[1]).collect();
    }

    digits
        .iter()
        .map(|b| char::from_digit(*b as u32, 10).unwrap())
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
