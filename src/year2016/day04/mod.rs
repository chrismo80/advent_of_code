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

    println!("4\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn check(name: &[String]) -> String
{
    let all = name.join("");
    let chars = all.chars().unique();

    // closure to count the number of occurrences of a character in a name
    let count = |char: char| all.chars().filter(|&c| c == char).count();

    let counts = chars.map(|char| (char, count(char)));
    let sorted = counts.sorted().sorted_by(|(_, count1), (_, count2)| count2.cmp(count1));
    sorted.map(|(char, _)| char).take(5).collect::<String>()
}

fn shift(name: &[String], sector_id: usize) -> String
{
    // closure to calculate the new character after shifting
    let calc = |char| (((char as usize - 'a' as usize + sector_id) % 26) + 'a' as usize) as u8 as char;

    name.iter().map(|n| n.chars().map(calc).collect::<String>()).join(" ")
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
