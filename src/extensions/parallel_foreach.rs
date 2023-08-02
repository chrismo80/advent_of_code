pub trait ParallelForEach
{
    type Item;
    type Output;

    fn parallel_foreach(&self, threads: usize, action: &(dyn Fn(&Self::Item) + Send + Sync)) -> Self::Output;
}

impl<T> ParallelForEach for Vec<T>
where
    T: Sync,
{
    type Item = T;
    type Output = usize;

    fn parallel_foreach(&self, threads: usize, action: &(dyn Fn(&Self::Item) + Send + Sync)) -> Self::Output
    {
        let chunks = self.chunks(self.len() / threads + (self.len() % threads != 0) as usize);
        let used_threads = chunks.len();

        std::thread::scope(|scope| {
            chunks.into_iter().for_each(|chunk| {
                scope.spawn(|| chunk.iter().for_each(|item| action(item)));
            });
        });

        used_threads
    }
}
