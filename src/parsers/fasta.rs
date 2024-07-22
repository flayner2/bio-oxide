use std::io::Error;

use crate::{alphabets::Alphabet, seq::SeqType};

pub struct FastaRecord {
    sequences: Vec<FastaSeq>,
}

pub struct FastaSeq {
    sequence: String,
    alphabet: Alphabet,
    seq_type: SeqType,
    id: String,
    desc: String,
}

impl FastaSeq {
    /// Creates a new [`FastaSeq`] with a specified `sequence`, `alphabet` and `seq_type`.
    pub fn new(
        sequence: String,
        alphabet: Alphabet,
        seq_type: SeqType,
        id: String,
        desc: String,
    ) -> Self {
        Self {
            sequence,
            alphabet,
            seq_type,
            id,
            desc,
        }
    }

    pub fn from_string(
        input_str: impl AsRef<str>,
        seq_type: SeqType,
        alphabet: Alphabet,
    ) -> Result<Self, Error> {
        let input_str_value = input_str.as_ref();
        let id;
        let desc;
        let seq;

        let (header, sequence) = input_str_value
            .split_at(
                input_str_value
                    .find('>')
                    .expect("Couldn't find the header identification symbol")
                    + 1,
            )
            .1
            .split_once('\n')
            .expect("Expected header and sequence to be separated by a line break");

        (id, desc) = header
            .split_once(' ')
            .expect("Couldn't split header row correctly");
        seq = sequence.replace("\n", "");

        Ok(Self::new(
            seq,
            alphabet,
            seq_type,
            id.to_owned(),
            desc.to_owned(),
        ))
    }

    pub fn sequence(&self) -> &str {
        &self.sequence
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn desc(&self) -> &str {
        &self.desc
    }

    pub fn alphabet(&self) -> &Alphabet {
        &self.alphabet
    }

    pub fn seq_type(&self) -> &SeqType {
        &self.seq_type
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_seq_from_str() {
        let seq = String::from("  \n>Seq1 Homo Sapiens COX1\nACTGGGTGTGT\n\nAAATTTGG\nATG");
        let fasta = FastaSeq::from_string(seq, SeqType::DNA, Alphabet::IUPACDNA)
            .expect("Couldn't create FASTA sequence");

        assert_eq!(fasta.id(), "Seq1");
        assert_eq!(fasta.desc(), "Homo Sapiens COX1");
        assert_eq!(fasta.sequence(), "ACTGGGTGTGTAAATTTGGATG");
    }
}
