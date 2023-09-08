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
        self.split(delim)
            .filter(|e| !e.is_empty())
            .map(|e| e.parse::<T>().unwrap())
            .collect()
    }

    fn to_vec_of_vec<T>(&self, delim1: &str, delim2: &str) -> Vec<Vec<T>>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.split(delim1)
            .filter(|e| !e.is_empty())
            .map(|e| e.to_vec::<T>(delim2))
            .collect()
    }

    fn to_vec_of_vec_of_vec<T>(&self, delim1: &str, delim2: &str, delim3: &str) -> Vec<Vec<Vec<T>>>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.split(delim1)
            .filter(|e| !e.is_empty())
            .map(|e| e.to_vec_of_vec::<T>(delim2, delim3))
            .collect()
    }
}
