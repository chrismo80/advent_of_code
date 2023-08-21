pub trait Permutations
where
    Self: Sized + Clone,
{
    fn permutations(&self) -> Vec<Self>;
}

impl<T> Permutations for Vec<T>
where
    T: Sized + Clone,
{
    fn permutations(&self) -> Vec<Vec<T>>
    {
        permutate(self.clone())
    }
}

pub fn permutate<T: Clone>(source: Vec<T>) -> Vec<Vec<T>>
{
    let mut values = source;

    permutations(values.len(), &mut values)
}

fn permutations<T: Clone>(count: usize, values: &mut [T]) -> Vec<Vec<T>>
{
    let mut variations = Vec::new();

    if count <= 1 {
        variations.push(values.to_vec());
    }
    else {
        variations.append(&mut permutations(count - 1, values));

        for i in 0..(count - 1) {
            if count % 2 == 0 {
                values.swap(i, count - 1);
            }
            else {
                values.swap(0, count - 1);
            }
            variations.append(&mut permutations(count - 1, values));
        }
    }

    variations
}
