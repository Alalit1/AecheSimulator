pub trait Resources {
    fn id(&self) -> u64;
    fn name(&self) -> &str;
}