pub fn solve() -> (usize, usize)
{
    let input = include_str!("input.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut result1 = 0;
    let mut result2 = 0;

    for x in 0..input.len() {
        for y in 0..input[0].len() {
            result1 += visible_from_outside(&input, x, y) as usize;
            result2 = result2.max(scenic_score(&input, x, y));
        }
    }

    println!("8\t{result1}\t\t{result2}");

    (result1, result2)
}

fn visible_from_outside(forest: &[Vec<char>], x: usize, y: usize) -> bool
{
    view_t(forest, x, y).all(|tree| tree < forest[x][y])
        || view_l(forest, x, y).all(|tree| tree < forest[x][y])
        || view_b(forest, x, y).all(|tree| tree < forest[x][y])
        || view_r(forest, x, y).all(|tree| tree < forest[x][y])
}

fn scenic_score(forest: &[Vec<char>], x: usize, y: usize) -> usize
{
    view_score(forest[x][y], view_t(forest, x, y))
        * view_score(forest[x][y], view_l(forest, x, y))
        * view_score(forest[x][y], view_b(forest, x, y))
        * view_score(forest[x][y], view_r(forest, x, y))
}

fn view_score(height: char, mut view: impl std::iter::ExactSizeIterator<Item = char>) -> usize
{
    let length = view.len();

    match view.position(|tree| tree >= height) {
        Some(pos) => pos + 1,
        None => length,
    }
}

fn view_t(forest: &[Vec<char>], x: usize, y: usize) -> impl ExactSizeIterator<Item = char> + '_
{
    (0..x).map(move |i| forest[i][y]).rev()
}

fn view_l(forest: &[Vec<char>], x: usize, y: usize) -> impl ExactSizeIterator<Item = char> + '_
{
    (0..y).map(move |i| forest[x][i]).rev()
}

fn view_b(forest: &[Vec<char>], x: usize, y: usize) -> impl ExactSizeIterator<Item = char> + '_
{
    (x..forest.len()).map(move |i| forest[i][y]).skip(1)
}

fn view_r(forest: &[Vec<char>], x: usize, y: usize) -> impl ExactSizeIterator<Item = char> + '_
{
    (y..forest[0].len()).map(move |i| forest[x][i]).skip(1)
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
