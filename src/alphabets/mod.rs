use std::default;

#[derive(Default)]
pub enum Alphabet {
    #[default]
    IUPACDNA,
    IUPACPROT,
}
