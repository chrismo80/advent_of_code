use crate::extensions::converter::Parser;

#[derive(Debug)]
struct Game
{
    id: usize,
    sets: Vec<(usize, usize, usize)>,
}

impl std::str::FromStr for Game
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let game = s.split_once(": ").unwrap();
        let id = game.0.split_once(' ').unwrap().1.parse::<usize>().unwrap();
        let rounds: Vec<&str> = game.1.split("; ").collect();
        let mut sets = Vec::new();

        for round in rounds {
            let colors: Vec<&str> = round.split(", ").collect();

            let mut r = 0;
            let mut g = 0;
            let mut b = 0;

            for color in colors {
                let parts = color.split_once(' ').unwrap();
                match parts.1 {
                    "red" => r = parts.0.parse::<usize>().unwrap(),
                    "green" => g = parts.0.parse::<usize>().unwrap(),
                    "blue" => b = parts.0.parse::<usize>().unwrap(),
                    _ => (),
                }
            }
            sets.push((r, g, b));
        }

        Ok(Game { id, sets })
    }
}

impl Game
{
    fn is_possible(&self, r: usize, g: usize, b: usize) -> bool
    {
        let r = self.sets.iter().map(|set| set.0).all(|x| x <= r);
        let g = self.sets.iter().map(|set| set.1).all(|x| x <= g);
        let b = self.sets.iter().map(|set| set.2).all(|x| x <= b);

        r && g && b
    }

    fn power(&self) -> usize
    {
        let r = self.sets.iter().map(|set| set.0).max().unwrap();
        let g = self.sets.iter().map(|set| set.1).max().unwrap();
        let b = self.sets.iter().map(|set| set.2).max().unwrap();

        r * g * b
    }
}

pub fn solve() -> (usize, usize)
{
    let input = include_str!("input.txt").to_vec::<Game>("\n");

    let result1 = input.iter().filter(|g| g.is_possible(12, 13, 14)).map(|g| g.id).sum();
    let result2 = input.iter().map(|g| g.power()).sum();

    println!("2\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (2268, 63542));
}
