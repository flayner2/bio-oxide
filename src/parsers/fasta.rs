use crate::{alphabets::Alphabet, seq::SeqType};

pub struct FastaRecord {
    sequences: Vec<FastaSeq>,
}

pub struct FastaSeq {
    sequence: String,
    alphabet: Alphabet,
    seq_type: SeqType,
}
