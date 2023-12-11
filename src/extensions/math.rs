pub trait Primes
{
    type Type;
    type Output;

    fn lcm(&self) -> Self::Output;
    fn gcf(&self) -> Self::Output;
}

impl<T> Primes for Vec<T>
where
    T: std::iter::Product
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>
        + Copy
        + PartialEq
        + PartialOrd<usize>
        + From<usize>,
{
    type Type = T;
    type Output = T;

    fn lcm(&self) -> Self::Output
    {
        let mut lcm = self[0];

        for value in self.iter().skip(1) {
            lcm = least_common_multiple(lcm, *value);
        }

        lcm
    }

    fn gcf(&self) -> Self::Output
    {
        let mut gcf = self[0];

        for value in self.iter().skip(1) {
            gcf = greatest_common_factor(gcf, *value);

            if gcf == 1 {
                return 1.into();
            }
        }

        gcf
    }
}

pub fn least_common_multiple<T>(a: T, b: T) -> T
where
    T: std::ops::Mul<Output = T> + std::ops::Div<Output = T> + std::ops::Rem<Output = T> + Copy + PartialEq + PartialOrd<usize>,
{
    let gcf = greatest_common_factor(a, b);

    a * b / gcf
}

pub fn greatest_common_factor<T>(a: T, b: T) -> T
where
    T: std::ops::Rem<Output = T> + Copy + PartialEq + PartialOrd<usize>,
{
    if a == 0 {
        return b;
    }

    greatest_common_factor(b % a, a)
}

#[test]
#[ignore]
fn test_lcm_gcf()
{
    assert_eq!(vec![8, 12, 20].gcf(), 4);

    assert_eq!(vec![6, 10].lcm(), 30);
    assert_eq!(vec![12, 15, 75].lcm(), 300);
}

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
fn test_permutations()
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
