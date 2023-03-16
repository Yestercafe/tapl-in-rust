#[derive(Debug, Clone)]
pub enum T {
    True,
    False,
    If(Box<T>, Box<T>, Box<T>),
}

#[derive(Debug, Clone)]
pub enum V {
    True,
    False,
}
