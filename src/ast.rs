#[derive(Debug)]
pub enum Expr {
    Path(Vec<PathSegment>),
}

#[derive(Debug, Clone)]
pub enum PathSegment {
    Field(String),
    Index(usize),
    Iter,
}
