pub trait ParallelForEach
{
    type Type;
    type Output;

    fn parallel_foreach(&self, threads: usize, action: &(dyn Fn(&Self::Type) + Send + Sync)) -> Self::Output;
}

impl<T> ParallelForEach for Vec<T>
where
    T: Sync,
{
    type Type = T;
    type Output = usize;

    fn parallel_foreach(&self, threads: usize, action: &(dyn Fn(&Self::Type) + Send + Sync)) -> Self::Output
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
