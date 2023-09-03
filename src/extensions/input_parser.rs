pub trait SimpleParser
{
    fn to_char_grid(&self) -> Vec<Vec<char>>;
    fn to_int_grid(&self) -> Vec<Vec<u32>>;
    fn to_vec_of_vec(&self) -> Vec<Vec<i64>>;
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

    fn to_vec_of_vec(&self) -> Vec<Vec<i64>>
    {
        self.split("\n\n")
            .map(|lines| lines.lines().map(|line| line.parse::<i64>().unwrap()).collect())
            .collect()
    }
}

pub trait GenericParser
{
    fn to_vec_multiline<T>(&self) -> Vec<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug;

    fn to_vec<T: std::str::FromStr>(&self) -> Vec<T>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug;
}

impl GenericParser for &str
{
    fn to_vec<T>(&self) -> Vec<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        parse(self, "\n")
    }

    fn to_vec_multiline<T>(&self) -> Vec<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        parse(self, "\n\n")
    }
}

fn parse<T>(me: &str, delimiter: &str) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    me.split(delimiter).map(|line| line.parse::<T>().unwrap()).collect()
}
