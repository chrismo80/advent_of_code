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
