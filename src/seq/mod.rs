use std::default;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum SeqType {
    #[default]
    DNA,
    Protein,
}
