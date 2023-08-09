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

    println!("8\t{result1}\t{result2}");

    (result1, result2)
}

fn visible_from_outside(forest: &[Vec<char>], x: usize, y: usize) -> bool
{
    view_t(forest, x, y).iter().all(|tree| tree < &forest[x][y])
        || view_l(forest, x, y).iter().all(|tree| tree < &forest[x][y])
        || view_b(forest, x, y).iter().all(|tree| tree < &forest[x][y])
        || view_r(forest, x, y).iter().all(|tree| tree < &forest[x][y])
}

fn scenic_score(forest: &[Vec<char>], x: usize, y: usize) -> usize
{
    view_score(forest, view_t(forest, x, y), x, y)
        * view_score(forest, view_l(forest, x, y), x, y)
        * view_score(forest, view_b(forest, x, y), x, y)
        * view_score(forest, view_r(forest, x, y), x, y)
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

// Mutithreaded solution:

// let mut result1 = std::sync::Mutex::new(0);
// let mut result2 = std::sync::Mutex::new(0);

// std::thread::scope(|scope| {
//     let input = &input; // shadowing to enable move only for x
//     let result1 = &result1;
//     let result2 = &result2;

//     for x in 0..input.len() {
//         scope.spawn(move || {
//             for y in 0..input[0].len() {
//                 let mut result1 = result1.lock().unwrap();
//                 let mut result2 = result2.lock().unwrap();

//                 *result1 += visible_from_outside(input, x, y) as usize;
//                 *result2 = (*result2).max(scenic_score(input, x, y));
//             }
//         });
//     }
// });

// let result1 = result1.lock().unwrap().to_owned();
// let result2 = result2.lock().unwrap().to_owned();
