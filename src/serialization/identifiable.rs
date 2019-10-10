pub trait Identifiable {
    type Identifier;
    fn identifier(&self) -> Identifier;
}