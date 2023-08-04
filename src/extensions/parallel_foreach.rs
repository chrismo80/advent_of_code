pub trait ParallelForEach
{
    type Type;
    type Output;

    fn parallel_foreach(&self, threads: u8, action: &(dyn Fn(&Self::Type) + Sync)) -> Self::Output;
}

impl<T> ParallelForEach for Vec<T>
where
    T: Sync,
{
    type Type = T;
    type Output = u8;

    fn parallel_foreach(&self, threads: u8, action: &(dyn Fn(&Self::Type) + Sync)) -> Self::Output
    {
        if threads == 0 {
            return 0;
        }

        let chunks = self.chunks(self.len() / threads as usize + (self.len() % threads as usize != 0) as usize);
        let used_threads = chunks.len() as u8;

        std::thread::scope(|scope| {
            chunks.into_iter().for_each(|chunk| {
                scope.spawn(|| chunk.iter().for_each(|item| action(item)));
            });
        });

        used_threads
    }
}
