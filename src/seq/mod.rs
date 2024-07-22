use std::default;

#[derive(Default)]
pub enum SeqType {
    #[default]
    DNA,
    Protein,
}
