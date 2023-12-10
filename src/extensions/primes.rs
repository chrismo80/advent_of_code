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
fn test()
{
    assert_eq!(vec![8, 12, 20].gcf(), 4);

    assert_eq!(vec![6, 10].lcm(), 30);
    assert_eq!(vec![12, 15, 75].lcm(), 300);
}
