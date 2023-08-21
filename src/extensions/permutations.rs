pub trait Permutations
where
    Self: Sized,
{
    fn permutations(&self) -> Vec<Self>;
}

impl<T: Clone> Permutations for Vec<T>
{
    fn permutations(&self) -> Vec<Vec<T>>
    {
        let mut values = self.clone();

        permute(values.len(), &mut values)
    }
}

fn permute<T: Clone>(count: usize, values: &mut [T]) -> Vec<Vec<T>>
{
    let mut variations = Vec::new();

    if count <= 1 {
        variations.push(values.to_vec());

        return variations;
    }

    variations.append(&mut permute(count - 1, values));

    for i in 0..(count - 1) {
        match count % 2 {
            0 => values.swap(i, count - 1),
            _ => values.swap(0, count - 1),
        }

        variations.append(&mut permute(count - 1, values));
    }

    variations
}

#[test]
fn test()
{
    let input = "ABC";
    let result = input.chars().collect::<Vec<char>>().permutations();

    assert_eq!(result.len(), 6);
    assert_eq!(result[0], vec!['A', 'B', 'C']);
    assert_eq!(result[1], vec!['B', 'A', 'C']);
    assert_eq!(result[2], vec!['C', 'A', 'B']);
    assert_eq!(result[3], vec!['A', 'C', 'B']);
    assert_eq!(result[4], vec!['B', 'C', 'A']);
    assert_eq!(result[5], vec!['C', 'B', 'A']);
}
