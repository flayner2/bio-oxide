use std::default;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum Alphabet {
    #[default]
    IUPACDNA,
    IUPACPROT,
}
