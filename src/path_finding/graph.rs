use std::collections::*;

#[derive(Default)]
pub struct Graph<T>
{
    map: HashMap<T, HashMap<T, i32>>,
    previous: HashMap<T, T>,
}

impl<T> Graph<T> where T: Clone + std::fmt::Debug {}
