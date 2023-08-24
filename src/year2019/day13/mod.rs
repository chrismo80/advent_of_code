use super::intcode_computer::*;
use std::collections::*;

pub fn solve() -> (usize, usize)
{
    let input = include_str!("input.txt").split(',');

    let mut memory: HashMap<i64, i64> = input.enumerate().map(|(i, x)| (i as i64, x.parse().unwrap())).collect();

    let mut icc = IntCodeComputer::new(memory.clone());

    icc.run();

    let outputs: Vec<i64> = icc.outputs.into();

    let screen = outputs.chunks(3).map(|x| (x[0], x[1], x[2]));

    let result1 = screen.clone().filter(|tile| tile.2 == 2).count();

    let x_max = screen.clone().map(|tile| tile.0).max().unwrap();
    let y_max = screen.clone().map(|tile| tile.1).max().unwrap();

    let mut board: Vec<Vec<i64>> = (0..=y_max).map(|_| vec![0; (x_max + 1) as usize]).collect();

    memory.insert(0, 2);

    let mut game = IntCodeComputer::new(memory);

    let (mut paddle, mut score, mut ball) = (0, 0, 0);

    while game.run() == State::Waiting || !game.outputs.is_empty() {
        let x = game.get_output().unwrap();
        let y = game.get_output().unwrap();
        let tile = game.get_output().unwrap();

        match x < 0 {
            true => score = tile,
            false => board[y as usize][x as usize] = tile,
        }

        match tile {
            3 => ball = x,
            4 => paddle = x,
            _ => {}
        }

        if game.outputs.is_empty() {
            game.add_input((paddle - ball).signum());
            //print_board(&board);
        }
    }

    let result2 = score as usize;

    println!("13\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

fn print_board(board: &Vec<Vec<i64>>)
{
    for row in board {
        for tile in row {
            print!(
                "{}",
                match tile {
                    1 => '|',
                    2 => '#',
                    3 => '-',
                    4 => 'o',
                    _ => '.',
                }
            );
        }
        println!();
    }
    std::thread::sleep(std::time::Duration::from_millis(50));
}

#[test]
fn test()
{
    assert_eq!(solve(), (320, 15156));
}
