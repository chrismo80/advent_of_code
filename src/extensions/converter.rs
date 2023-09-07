pub trait Converter
{
    fn to_char_grid(&self) -> Vec<Vec<char>>;

    fn to_int_grid(&self) -> Vec<Vec<u32>>;

    fn to_vec<T>(&self, delim: &str) -> Vec<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug;

    fn to_vec_of_vec<T>(&self, delim1: &str, delim2: &str) -> Vec<Vec<T>>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug;

    fn to_vec_of_vec_of_vec<T>(&self, delim1: &str, delim2: &str, delim3: &str) -> Vec<Vec<Vec<T>>>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug;
}

impl Converter for &str
{
    fn to_char_grid(&self) -> Vec<Vec<char>>
    {
        self.lines().map(|line| line.chars().collect()).collect()
    }

    fn to_int_grid(&self) -> Vec<Vec<u32>>
    {
        self.lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect()
    }

    fn to_vec<T>(&self, delim: &str) -> Vec<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.split(delim).map(|line| line.parse::<T>().unwrap()).collect()
    }

    fn to_vec_of_vec<T>(&self, delim1: &str, delim2: &str) -> Vec<Vec<T>>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.split(delim1)
            .map(|line| line.split(delim2).map(|e| e.parse::<T>().unwrap()).collect())
            .collect()
    }

    fn to_vec_of_vec_of_vec<T>(&self, delim1: &str, delim2: &str, delim3: &str) -> Vec<Vec<Vec<T>>>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.split(delim1)
            .map(|item1| {
                item1
                    .split(delim2)
                    .map(|item2| item2.split(delim3).map(|e| e.parse::<T>().unwrap()).collect())
                    .collect()
            })
            .collect()
    }
}
