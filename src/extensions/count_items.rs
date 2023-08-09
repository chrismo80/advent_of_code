pub trait CountItems
{
    type Type;
    type Output;

    fn count_items(&self) -> Self::Output;
}

impl<T> CountItems for Vec<T>
where
    T: PartialEq + Eq + Clone + std::hash::Hash,
{
    type Type = T;
    type Output = std::collections::HashMap<T, i32>;

    fn count_items(&self) -> Self::Output
    {
        let mut map: std::collections::HashMap<T, i32> = std::collections::HashMap::new();

        for item in self {
            let count = map.entry(item.clone()).or_insert(0); // 0 is the default value if the key doesn't exist
            *count += 1; // increment the value if the key exists
        }

        map
    }
}
