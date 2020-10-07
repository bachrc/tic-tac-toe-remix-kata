pub trait MonItertools : Iterator {
    fn partition_map_results<F, L, R>(self, predicate: F) -> (Vec<L>, Vec<R>)
        where Self: Sized,
              F: Fn(Self::Item) -> Result<L, R>,
    {
        let mut successes = Vec::new();
        let mut errors = Vec::new();

        self.for_each(|val| match predicate(val) {
            Ok(v) => successes.push(v),
            Err(v) => errors.push(v),
        });

        (successes, errors)
    }
}

impl<T: ?Sized> MonItertools for T where T: Iterator { }