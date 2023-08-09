pub fn solve() -> (i32, i32)
{
    let input = include_str!("input.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut result1 = 0;
    let mut result2 = 0;

    for x in 0..input.len() {
        for y in 0..input[0].len() {
            result1 += visible_from_outside(&input, x, y) as i32;
            result2 = result2.max(scenic_score(&input, x, y) as i32);
        }
    }

    println!("8\t{result1}\t{result2}");

    (result1, result2)
}

fn visible_from_outside(forest: &[Vec<char>], x: usize, y: usize) -> bool
{
    let visible_t = view_t(forest, x, y).iter().all(|tree| tree < &forest[x][y]);
    let visible_l = view_l(forest, x, y).iter().all(|tree| tree < &forest[x][y]);
    let visible_b = view_b(forest, x, y).iter().all(|tree| tree < &forest[x][y]);
    let visible_r = view_r(forest, x, y).iter().all(|tree| tree < &forest[x][y]);

    visible_t || visible_l || visible_b || visible_r
}

fn scenic_score(forest: &[Vec<char>], x: usize, y: usize) -> usize
{
    let score_t = view_score(forest, view_t(forest, x, y), x, y);
    let score_l = view_score(forest, view_l(forest, x, y), x, y);
    let score_b = view_score(forest, view_b(forest, x, y), x, y);
    let score_r = view_score(forest, view_r(forest, x, y), x, y);

    score_t * score_l * score_b * score_r
}

fn view_score(forest: &[Vec<char>], view: Vec<char>, x: usize, y: usize) -> usize
{
    match view.iter().position(|tree| tree >= &forest[x][y]) {
        Some(i) => i + 1,
        None => view.len(),
    }
}

fn view_t(forest: &[Vec<char>], x: usize, y: usize) -> Vec<char>
{
    (0..x).map(|i| forest[i][y]).rev().collect()
}

fn view_l(forest: &[Vec<char>], x: usize, y: usize) -> Vec<char>
{
    (0..y).map(|i| forest[x][i]).rev().collect()
}

fn view_b(forest: &[Vec<char>], x: usize, y: usize) -> Vec<char>
{
    (x..forest.len()).map(|i| forest[i][y]).skip(1).collect()
}

fn view_r(forest: &[Vec<char>], x: usize, y: usize) -> Vec<char>
{
    (y..forest[0].len()).map(|i| forest[x][i]).skip(1).collect()
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve(), (1845, 230112));
    }
}
