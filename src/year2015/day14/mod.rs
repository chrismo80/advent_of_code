struct Reindeer
{
    name: String,
    speed: usize,
    fly_time: usize,
    rest_time: usize,
    distance: usize,
    points: usize,
}

impl std::str::FromStr for Reindeer
{
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let mut words = s.split_whitespace();

        Ok(Reindeer {
            name: words.next().unwrap().to_string(),
            speed: words.nth(2).unwrap().parse::<usize>().unwrap(),
            fly_time: words.nth(2).unwrap().parse::<usize>().unwrap(),
            rest_time: words.nth(6).unwrap().parse::<usize>().unwrap(),
            distance: 0,
            points: 0,
        })
    }
}

impl Reindeer
{
    fn update_distance(&mut self, duration: usize)
    {
        let cycle_time = self.fly_time + self.rest_time;
        let cycles = duration / cycle_time;
        let remainder = self.fly_time.min(duration % cycle_time);

        self.distance = (cycles * self.fly_time + remainder) * self.speed;
    }
}

struct Race
{
    reindeers: Vec<Reindeer>,
}

impl Race
{
    fn new(reindeers: Vec<Reindeer>) -> Self
    {
        Self { reindeers }
    }

    fn update(&mut self, duration: usize)
    {
        for reindeer in self.reindeers.iter_mut() {
            reindeer.update_distance(duration);
        }

        let max_distance = self.reindeers.iter().map(|r| r.distance).max().unwrap();

        for reindeer in self.reindeers.iter_mut() {
            if reindeer.distance == max_distance {
                reindeer.points += 1;
            }
        }
    }
}

pub fn solve() -> (usize, usize)
{
    let reindeers: Vec<Reindeer> = include_str!("input.txt").lines().map(|l| l.parse().unwrap()).collect();

    let mut race = Race::new(reindeers);

    for s in 1..=2503 {
        race.update(s);
    }

    let result1 = race.reindeers.iter().map(|r| r.distance).max().unwrap();
    let result2 = race.reindeers.iter().map(|r| r.points).max().unwrap();

    println!("14\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (2640, 1102));
    }
}
