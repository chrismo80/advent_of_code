pub fn solve() -> (usize, usize)
{
    let input: Vec<&str> = include_str!("input.txt").split(' ').collect();

    let players = input[0].parse::<usize>().unwrap();
    let last_marble = input[6].parse::<usize>().unwrap();

    let result1 = get_max_score(players, last_marble);
    let result2 = get_max_score(players, last_marble * 100);

    println!("9\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn get_max_score(players: usize, last_marble: usize) -> usize
{
    let mut circle = std::collections::VecDeque::new();
    let mut scores = vec![0; players];

    circle.push_front(0);

    for marble in 1..last_marble {
        if marble % 23 == 0 {
            for _ in 0..7 {
                let e = circle.pop_front().unwrap();
                circle.push_back(e);
            }

            scores[marble % players] += marble + circle.pop_back().unwrap();
        }
        else {
            for _ in 0..2 {
                let e = circle.pop_back().unwrap();
                circle.push_front(e);
            }
            circle.push_back(marble);
        }
    }

    *scores.iter().max().unwrap()
}

#[test]
fn test()
{
    assert_eq!(solve(), (404502, 3243916887));
}
