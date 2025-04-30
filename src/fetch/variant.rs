#[derive(Debug, Clone, PartialEq)]
pub enum Variant<T, U> {
    Single(T),
    Multiple(U),
    None,
}

impl<T, U> Variant<T, U> {
    pub(crate) fn single_or_default(self) -> T
    where
        T: Default,
    {
        match self {
            Variant::Single(var) => var,
            _ => T::default(),
        }
    }

    pub fn multiple_or_default(self) -> U
    where
        U: Default,
    {
        match self {
            Variant::Multiple(map) => map,
            _ => U::default(),
        }
    }
}
