use crate::extensions::converter::Parser;

pub fn solve() -> (String, usize)
{
    let map = include_str!("input.txt").to_char_grid();

    let mut x = map[0].iter().position(|&c| c == '|').unwrap();
    let mut y = 0;
    let mut steps = 0;
    let mut face = 2; // 0 north, 1 east, 2 south, 3 west
    let mut letters = String::new();

    while map[y][x] != ' ' {
        steps += 1;

        match map[y][x] {
            '+' => match (face % 2, x > 0 && map[y][x - 1] != ' ', y > 0 && map[y - 1][x] != ' ') {
                (0, true, _) => face = 3,
                (0, false, _) => face = 1,
                (1, _, true) => face = 0,
                (1, _, false) => face = 2,
                _ => unreachable!(),
            },
            '|' | '-' => {}
            c => letters.push(c),
        }

        match face {
            0 => y -= 1,
            1 => x += 1,
            2 => y += 1,
            3 => x -= 1,
            _ => panic!("Invalid face"),
        }
    }

    let result1 = letters;
    let result2 = steps;

    println!("19\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[test]
fn test()
{
    assert_eq!(solve(), (String::from("LXWCKGRAOY"), 17302));
}
