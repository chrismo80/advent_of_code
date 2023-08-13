use iter_tools::Itertools;

pub fn solve() -> (usize, usize)
{
    let mut rooms: Vec<(Vec<String>, usize, String)> = Vec::new();

    for line in include_str!("input.txt").lines() {
        let mut parts = line.split(['-', '[', ']']).collect::<Vec<&str>>();

        parts.pop(); // skip last empty one

        let checksum = parts.pop().unwrap().parse().unwrap();
        let sector_id: usize = parts.pop().unwrap().parse().unwrap();
        let name = parts.iter().map(|s| s.to_string()).collect::<Vec<String>>();

        rooms.push((name, sector_id, checksum));
    }

    let location = "northpole object storage";

    let result1 = rooms.iter().filter(|(n, _, c)| check(n) == *c).map(|(_, s, _)| s).sum();
    let result2 = rooms.iter().find(|(n, s, _)| shift(n, *s) == location).unwrap().1;

    println!("4\t{result1:<15}\t{result2:<15}");

    (result1, result2)
}

fn check(name: &[String]) -> String
{
    name.iter()
        .flat_map(|part| part.chars().collect::<Vec<char>>())
        .fold(vec![], |mut acc, c| {
            if !acc.contains(&c) {
                acc.push(c);
            }
            acc
        })
        .iter()
        .map(|c| {
            (
                c,
                name.iter()
                    .flat_map(|part| part.chars().collect::<Vec<char>>())
                    .filter(|&c2| c2 == *c)
                    .count(),
            )
        })
        .sorted()
        .sorted_by(|(_, count1), (_, count2)| count2.cmp(count1))
        .map(|(c, _)| c)
        .take(5)
        .copied()
        .collect::<String>()
}

fn shift(name: &[String], sector_id: usize) -> String
{
    name.iter()
        .map(|word| {
            word.chars()
                .map(|c| (((c as usize - 'a' as usize + sector_id) % 26) + 'a' as usize) as u8 as char)
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (245102, 324));
    }
}