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
        permute(&mut self.clone())
        //permute_recursive(self.len(), &mut self.clone())
    }
}

fn permute<T: Clone>(values: &mut [T]) -> Vec<Vec<T>>
{
    let mut variations = Vec::new();

    variations.push(values.to_vec());

    let mut c = vec![0; values.len()];
    let mut i = 0;

    while i < values.len() {
        if c[i] < i {
            values.swap((i % 2) * c[i], i);
            variations.push(values.to_vec());
            c[i] += 1;
            i = 1;
        }
        else {
            c[i] = 0;
            i += 1;
        }
    }

    variations
}

// https://en.wikipedia.org/wiki/Heap%27s_algorithm
fn permute_recursive<T: Clone>(count: usize, values: &mut [T]) -> Vec<Vec<T>>
{
    let mut variations = Vec::new();

    if count <= 1 {
        variations.push(values.to_vec());

        return variations;
    }

    variations.append(&mut permute_recursive(count - 1, values));

    for i in 0..(count - 1) {
        values.swap(i - (i * (count % 2)), count - 1);
        variations.append(&mut permute_recursive(count - 1, values));
    }

    variations
}

#[test]
#[ignore]
fn test()
{
    let input = vec!['A', 'B', 'C'];
    let result = input.permutations();

    assert_eq!(result.len(), 6);
    assert_eq!(result[0], vec!['A', 'B', 'C']);
    assert_eq!(result[1], vec!['B', 'A', 'C']);
    assert_eq!(result[2], vec!['C', 'A', 'B']);
    assert_eq!(result[3], vec!['A', 'C', 'B']);
    assert_eq!(result[4], vec!['B', 'C', 'A']);
    assert_eq!(result[5], vec!['C', 'B', 'A']);

    let input = (1..10).collect::<Vec<_>>();

    assert_eq!(
        permute(&mut input.clone()),
        permute_recursive(input.len(), &mut input.clone())
    );
}
