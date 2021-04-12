pub trait CloneFn: Fn(f64) -> f64 {
    fn clone_box(&self) -> Box<dyn CloneFn>;
}

impl<F: Clone + 'static> CloneFn for F
where
    F: Fn(f64) -> f64,
{
    fn clone_box(&self) -> Box<dyn CloneFn> {
        Box::new(self.clone())
    }
}
