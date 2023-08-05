use crate::extensions::count_items::*;
use std::collections::*;
use std::fmt::*;

pub fn main()
{
    let text = String::from("Hello world wonderful world");
    let words: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();
    let map = words.count_items();

    let v = vec![1, 2, 3, 4, 3, 2, 7, 2, 1, 3, 2, 1, 2, 3, 4, 3, 4, 5, 4, 3, 9];
    let c = v.count_items();

    print_data(map);
    print_data(c);
}

fn print_data<K, V>(data: HashMap<K, V>)
where
    K: Display,
    V: Display,
{
    data.iter().for_each(|(key, value)| println!("{key}: {value}"));
}
