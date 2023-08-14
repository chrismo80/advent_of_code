use std::collections::*;

pub fn solve() -> (isize, isize)
{
    let input = include_str!("input.txt").lines();

    let mut dir: Vec<String> = Vec::new();
    let mut files: Vec<(String, String, isize)> = Vec::new();

    for line in input.filter(|l| l != &"$ ls") {
        if line == "$ cd .." {
            dir.pop();
        }
        else if line.starts_with("$ cd") {
            dir.push(line.split_whitespace().collect::<Vec<&str>>()[2].to_string());
        }
        else if line.starts_with("dir") {
            files.push((
                dir.join("/") + "/" + line.split_whitespace().collect::<Vec<&str>>()[1],
                "-".to_string(),
                0,
            ));
        }
        else {
            files.push((
                dir.join("/"),
                line.split_whitespace().collect::<Vec<&str>>()[1].to_string(),
                line.split_whitespace().collect::<Vec<&str>>()[0]
                    .parse::<isize>()
                    .unwrap(),
            ));
        }
    }

    let mut folders = files
        .iter()
        .map(|f| f.0.clone())
        .collect::<HashSet<String>>()
        .iter()
        .map(|d| {
            (
                d.clone(),
                files.iter().filter(|f| f.0.starts_with(d)).map(|f| f.2).sum(),
            )
        })
        .collect::<Vec<(String, isize)>>();

    folders.sort();

    let result1 = folders.iter().filter(|f| f.1 < 100000).map(|f| f.1).sum();

    let result2 = *folders
        .iter()
        .filter(|f| f.1 > folders[0].1 - 40000000)
        .map(|f| f.1)
        .collect::<Vec<isize>>()
        .last()
        .unwrap();

    println!("7\t{result1:<20}\t{result2:<20}");

    (result1, result2)
}

#[cfg(test)]
mod tests
{
    #[test]
    fn solve()
    {
        assert_eq!(super::solve().0, 1513699);
    }
}
