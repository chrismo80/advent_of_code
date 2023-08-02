pub trait ParallelForEach
{
    type Type;
    type Output;

    fn parallel_foreach(&self, threads: u8, action: &(dyn Fn(&Self::Type) + Send + Sync)) -> Self::Output;
}

impl<T> ParallelForEach for Vec<T>
where
    T: Sync,
{
    type Type = T;
    type Output = Result<usize, String>;

    fn parallel_foreach(&self, threads: u8, action: &(dyn Fn(&Self::Type) + Send + Sync)) -> Self::Output
    {
        if threads == 0
        {
            return Err(String::from("Cannot use 0 threads"));
        }

        let chunks = self.chunks(self.len() / threads as usize + (self.len() % threads as usize != 0) as usize);
        let used_threads = chunks.len();

        std::thread::scope(|scope| {
            chunks.into_iter().for_each(|chunk| {
                scope.spawn(|| chunk.iter().for_each(|item| action(item)));
            });
        });

        Ok(used_threads)
    }
}
