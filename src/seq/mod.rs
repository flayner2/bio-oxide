use lazy_static::lazy_static;
use std::collections::{btree_set::Difference, BTreeSet};

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum SeqType {
    #[default]
    DNA,
    RNA,
    Protein,
}

lazy_static! {
    pub(crate) static ref NUCLEIC_ACID_SYMBOLS: BTreeSet<char> = BTreeSet::from([
        'A', 'C', 'G', 'T', 'U', 'W', 'S', 'M', 'K', 'R', 'Y', 'B', 'D', 'H', 'V', 'N',
    ]);
    pub(crate) static ref AMINOACID_SYMBOLS: BTreeSet<char> = BTreeSet::from([
        'A', 'R', 'N', 'D', 'C', 'Q', 'E', 'G', 'H', 'I', 'L', 'K', 'M', 'F', 'P', 'S', 'T', 'W',
        'Y', 'V', 'X',
    ]);
    pub(crate) static ref AMINOACID_EXCLUSIVE_SYMBOLS: Difference<'static, char> =
        AMINOACID_SYMBOLS.difference(&NUCLEIC_ACID_SYMBOLS);
}
