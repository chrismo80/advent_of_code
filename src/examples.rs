use crate::extensions::*;
use std::collections::*;
use std::fmt::*;
use std::hash::*;
use std::sync::*;
use std::thread::*;
use std::time::*;

pub fn test()
{
    print_data(process_items("ABCDEF".chars().collect(), 5));
    print_data(process_items("ABCDEFGHIJKLMNOPQRSTU".chars().collect(), 14));

    print_data(process_items((8..=13).collect(), 3));
    print_data(process_items((1..=17).collect(), 7));
    print_data(process_items((1..=50).collect(), 13));

    print_data(process_items(vec!["First item", "Item N", "Last item"], 5));
    print_data(process_items(vec!["AB", "CD", "EF"], 4));
    print_data(process_items(vec!["Peter", "Paul", "Mary"], 2));
}

fn print_data<K, V>(data: HashMap<K, V>)
where
    K: Display,
    V: Display,
{
    data.iter().for_each(|(key, value)| println!("{key}: {value}"));
}

fn process_items<T>(items: Vec<T>, threads: i32) -> HashMap<T, T::Output>
where
    T: ProcessItem + Send + Sync + Copy + Eq + Hash,
    T::Output: Send,
{
    let start = Instant::now();
    let data: Mutex<HashMap<T, <T as ProcessItem>::Output>> = Mutex::new(HashMap::new());

    let function = |item: &T| {
        let result = item.process();
        data.lock().unwrap().insert(*item, result);
    };

    let threads_used = items.parallel_foreach(threads as usize, &function);
    let data = data.into_inner().unwrap();

    print!("processed {} items", data.len());
    print!(" with {}/{} threads", threads_used, threads);
    println!(" in {:.1} ms", start.elapsed().as_micros() as f32 / 1000.0);

    data
}

impl ProcessItem for i32
{
    type Output = i32;
    fn process(&self) -> Self::Output
    {
        wait();
        self * self
    }
}

impl ProcessItem for &str
{
    type Output = String;
    fn process(&self) -> Self::Output
    {
        wait();
        self.to_string() + " -> Done"
    }
}

impl ProcessItem for char
{
    type Output = String;
    fn process(&self) -> Self::Output
    {
        wait();
        self.to_string() + &self.to_string()
    }
}

fn wait()
{
    sleep(Duration::new(1, 0));
}

trait ProcessItem
{
    type Output;
    fn process(&self) -> Self::Output;
}
