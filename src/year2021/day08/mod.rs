use crate::extensions::converter::Parser;

pub fn solve() -> (usize, usize)
{
    let input = include_str!("input.txt").to_vec_of_vec_of_vec::<String>("\n", " | ", " ");

    let result1 = input
        .iter()
        .map(|l| l[1].iter().map(|w| w.len()))
        .map(|x| x.filter(|&d| d == 2 || d == 3 || d == 4 || d == 7).count())
        .sum();

    let result2 = input.iter().map(|l| Decoder::new(&l[0]).decode(&l[1])).sum();

    println!("8\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

struct Decoder
{
    one: Vec<char>,
    four: Vec<char>,
}

impl Decoder
{
    fn new(signal: &[String]) -> Decoder
    {
        Decoder {
            one: signal.iter().find(|s| s.len() == 2).unwrap().chars().collect(),
            four: signal.iter().find(|s| s.len() == 4).unwrap().chars().collect(),
        }
    }

    fn decode(&self, o: &[String]) -> usize
    {
        o.iter().map(|s| self.decode_segment(s)).collect::<String>().parse().unwrap()
    }

    fn decode_segment(&self, s: &str) -> char
    {
        match (s.len(), self.intersect(&self.one, s), self.intersect(&self.four, s)) {
            (2, _, _) => '1',
            (5, _, 2) => '2',
            (5, 2, _) => '3',
            (4, _, _) => '4',
            (5, 1, _) => '5',
            (6, 1, _) => '6',
            (3, _, _) => '7',
            (7, _, _) => '8',
            (6, _, 4) => '9',
            (6, _, 3) => '0',
            _ => panic!(),
        }
    }

    fn intersect(&self, number: &[char], s: &str) -> usize
    {
        number.iter().filter(|c| s.contains(**c)).count()
    }
}

#[test]
fn test()
{
    assert_eq!(solve(), (530, 1051087));
}
