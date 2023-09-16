use crate::extensions::converter::{Converter, Matrix};

pub fn solve() -> (u32, u32)
{
    let input = include_str!("input.txt");

    let numbers = input.lines().next().unwrap().to_vec::<u32>(",");

    let mut tables = input.to_vec::<Board>("\n\n");
    tables.remove(0);

    for number in numbers {
        for table in tables.iter_mut() {
            table.draw(number);
        }

        if tables.iter().all(|table| table.finished) {
            break;
        }
    }

    tables.sort_by_key(|t| t.drawn.len());

    let first = tables.first().unwrap();
    let last = tables.last().unwrap();

    let result1 = first.points * first.drawn.last().unwrap();
    let result2 = last.points * last.drawn.last().unwrap();

    println!("4\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

impl Board
{
    fn draw(&mut self, number: u32)
    {
        if self.finished {
            return;
        }

        self.drawn.push(number);

        for row in self.rows.iter_mut() {
            if let Some(index) = row.iter().position(|&n| n == number) {
                row.remove(index);

                self.points -= number;

                if row.is_empty() {
                    self.finished = true;
                }
            }
        }

        for col in self.cols.iter_mut() {
            if let Some(index) = col.iter().position(|&n| n == number) {
                col.remove(index);

                if col.is_empty() {
                    self.finished = true;
                }
            }
        }
    }
}

#[derive(Debug)]
struct Board
{
    rows: Vec<Vec<u32>>,
    cols: Vec<Vec<u32>>,
    drawn: Vec<u32>,
    finished: bool,
    points: u32,
}

impl std::str::FromStr for Board
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let grid = s.to_matrix(" ");

        Ok(Board {
            rows: grid.clone(),
            cols: grid.transpose(),
            drawn: Vec::new(),
            finished: false,
            points: grid.iter().map(|r| r.iter().sum::<u32>()).sum(),
        })
    }
}

#[test]
fn test()
{
    assert_eq!(solve(), (16716, 4880));
}
