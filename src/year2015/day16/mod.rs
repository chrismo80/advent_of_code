#[derive(Debug, Default)]
struct Sue
{
    number: usize,
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}

impl std::str::FromStr for Sue
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let mut sue = Sue { ..Default::default() };

        let mut iter = s.split_whitespace();
        iter.next();

        sue.number = iter.next().unwrap().trim_end_matches(':').parse().unwrap();

        while let Some(prop) = iter.next() {
            let value = iter.next().unwrap().trim_end_matches(',');

            match prop {
                "children:" => sue.children = Some(value.parse().unwrap()),
                "cats:" => sue.cats = Some(value.parse().unwrap()),
                "samoyeds:" => sue.samoyeds = Some(value.parse().unwrap()),
                "pomeranians:" => sue.pomeranians = Some(value.parse().unwrap()),
                "akitas:" => sue.akitas = Some(value.parse().unwrap()),
                "vizslas:" => sue.vizslas = Some(value.parse().unwrap()),
                "goldfish:" => sue.goldfish = Some(value.parse().unwrap()),
                "trees:" => sue.trees = Some(value.parse().unwrap()),
                "cars:" => sue.cars = Some(value.parse().unwrap()),
                "perfumes:" => sue.perfumes = Some(value.parse().unwrap()),
                _ => panic!("Unknown property: {}", prop),
            }
        }

        Ok(sue)
    }
}

pub fn solve() -> (usize, usize)
{
    let input: Vec<Sue> = include_str!("input.txt").lines().map(|l| l.parse().unwrap()).collect();

    let result1 = input
        .iter()
        .find(|sue| {
            (sue.children.is_none() || sue.children == Some(3))
                && (sue.cats.is_none() || sue.cats == Some(7))
                && (sue.samoyeds.is_none() || sue.samoyeds == Some(2))
                && (sue.pomeranians.is_none() || sue.pomeranians == Some(3))
                && (sue.akitas.is_none() || sue.akitas == Some(0))
                && (sue.vizslas.is_none() || sue.vizslas == Some(0))
                && (sue.goldfish.is_none() || sue.goldfish == Some(5))
                && (sue.trees.is_none() || sue.trees == Some(3))
                && (sue.cars.is_none() || sue.cars == Some(2))
                && (sue.perfumes.is_none() || sue.perfumes == Some(1))
        })
        .unwrap()
        .number;

    let result2 = input
        .iter()
        .find(|sue| {
            (sue.children.is_none() || sue.children == Some(3))
                && (sue.cats.is_none() || sue.cats > Some(7))
                && (sue.samoyeds.is_none() || sue.samoyeds == Some(2))
                && (sue.pomeranians.is_none() || sue.pomeranians < Some(3))
                && (sue.akitas.is_none() || sue.akitas == Some(0))
                && (sue.vizslas.is_none() || sue.vizslas == Some(0))
                && (sue.goldfish.is_none() || sue.goldfish < Some(5))
                && (sue.trees.is_none() || sue.trees > Some(3))
                && (sue.cars.is_none() || sue.cars == Some(2))
                && (sue.perfumes.is_none() || sue.perfumes == Some(1))
        })
        .unwrap()
        .number;

    println!("16\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (373, 260));
    }
}
