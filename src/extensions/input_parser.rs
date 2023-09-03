pub trait SimpleParser
{
    fn to_char_grid(&self) -> Vec<Vec<char>>;
    fn to_int_grid(&self) -> Vec<Vec<u32>>;
}

impl SimpleParser for &str
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
}

pub trait GenericParser
{
    fn to_vec_multiline<T: std::str::FromStr>(&self) -> Vec<T>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug;

    fn to_vec<T: std::str::FromStr>(&self) -> Vec<T>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug;
}

impl GenericParser for &str
{
    fn to_vec<T: std::str::FromStr>(&self) -> Vec<T>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        parse(self, "\n")
    }

    fn to_vec_multiline<T: std::str::FromStr>(&self) -> Vec<T>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        parse(self, "\n\n")
    }
}

fn parse<T: std::str::FromStr>(me: &str, delimiter: &str) -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    me.split(delimiter).map(|line| line.parse::<T>().unwrap()).collect()
}
