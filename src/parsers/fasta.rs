use std::io::Error;

use crate::{
    alphabets::Alphabet,
    seq::{SeqType, AMINOACID_EXCLUSIVE_SYMBOLS},
};

pub struct FastaRecord {
    sequences: Vec<FastaSeq>,
}

/**
A structure representing a single sequence in a FASTA file.
*/
pub struct FastaSeq {
    sequence: String,
    alphabet: Alphabet,
    seq_type: SeqType,
    id: String,
    desc: Option<String>,
}

impl FastaSeq {
    /**
    Creates a new [`FastaSeq`] with a specified `sequence`, [`Alphabet`],
    [`SeqType`], `id` and `desc`.

    This method is suitable for manual construction of FASTA sequences. For
    parsing FASTA files, see [`FastaRecord::from_file`]. For loading
    sequences from a FASTA-formatted string, take a look at
    [`FastaSeq::from_string`] and [`FastaSeq::from_string_inferred`].
    */
    pub fn new(
        sequence: String,
        alphabet: Alphabet,
        seq_type: SeqType,
        id: String,
        desc: Option<String>,
    ) -> Self {
        Self {
            sequence,
            alphabet,
            seq_type,
            id,
            desc,
        }
    }

    /**
    Creates a new [`FastaSeq`] from an input string, an [`Alphabet`] and a
    [`SeqType`]. This method expects the user to provide the [`Alphabet`] and
    [`SeqType`] explicitly. The input FASTA string must have a valid FASTA
    format and may be any of [`String`], `&String`, [`str`] or `&str`.

    For automatic inference of the [`Alphabet`] and [`SeqType`], take a look at
    [`FastaSeq::from_string_inferred`].
    */
    pub fn from_string(
        input_str: impl AsRef<str>,
        seq_type: SeqType,
        alphabet: Alphabet,
    ) -> Result<Self, Error> {
        let input_str_value = input_str.as_ref();

        let (header, sequence) = input_str_value
            .split_at(
                input_str_value
                    .find('>')
                    .expect("a valid FASTA sequence should have a header starting with '>'")
                    + 1,
            )
            .1
            .split_once('\n')
            .expect("a valid FASTA sequence should have the header and sequence separated by at least one line break");

        let (id, desc) = header.split_once(' ').unwrap_or_else(|| (header, ""));

        let id = (!id.trim().is_empty())
            .then(|| id.trim().to_owned())
            .expect("a valid FASTA sequence should contain an ID");
        let desc = (!desc.is_empty()).then(|| desc.trim().to_owned());
        let seq = sequence.replace("\n", "").trim().to_owned();

        Ok(Self::new(seq, alphabet, seq_type, id, desc))
    }

    fn infer_type_and_alphabet(input_str: impl AsRef<str>) -> Result<(SeqType, Alphabet), Error> {
        let input_str_value = input_str.as_ref().to_ascii_uppercase();

        if input_str_value.contains('U') {
            Ok((SeqType::RNA, Alphabet::IUPACNucleicAcid))
        } else if AMINOACID_EXCLUSIVE_SYMBOLS
            .clone()
            .any(|symbol| input_str_value.contains(*symbol))
        {
            Ok((SeqType::Protein, Alphabet::IUPACProtein))
        } else {
            Ok((SeqType::default(), Alphabet::default()))
        }
    }

    pub fn from_string_inferred(input_str: impl AsRef<str>) -> Result<Self, Error> {
        let input_str_value = input_str.as_ref();
        // TODO: Improve this
        let fasta_seq =
            Self::from_string(input_str_value, SeqType::DNA, Alphabet::IUPACNucleicAcid)?;
        let (seq_type, alphabet) = Self::infer_type_and_alphabet(fasta_seq.sequence())?;

        Ok(Self::new(
            fasta_seq.sequence().to_owned(),
            alphabet,
            seq_type,
            fasta_seq.id().to_owned(),
            fasta_seq.desc().map(|val| val.to_owned()),
        ))
    }

    pub fn sequence(&self) -> &str {
        &self.sequence
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn desc(&self) -> Option<&str> {
        self.desc.as_deref()
    }

    pub fn alphabet(&self) -> Alphabet {
        self.alphabet
    }

    pub fn seq_type(&self) -> SeqType {
        self.seq_type
    }

    pub fn len(&self) -> usize {
        self.sequence().len()
    }
}

impl FastaRecord {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_seq_from_string() {
        let seq = String::from("  \n>Seq1 Homo Sapiens COX1\nACTGGGTGTGT\n\nAAATTTGG\nATG");
        let fasta = FastaSeq::from_string(seq, SeqType::DNA, Alphabet::IUPACNucleicAcid)
            .expect("Couldn't create FASTA sequence");

        assert_eq!(fasta.id(), "Seq1");
        assert_eq!(fasta.desc(), Some("Homo Sapiens COX1"));
        assert_eq!(fasta.sequence(), "ACTGGGTGTGTAAATTTGGATG");
        assert_eq!(fasta.alphabet(), Alphabet::IUPACNucleicAcid);
        assert_eq!(fasta.seq_type(), SeqType::DNA);
        assert_eq!(fasta.len(), fasta.sequence().len());
    }

    #[test]
    fn create_seq_from_string_no_desc() {
        let seq = String::from("  \n>Seq1\nACTGGGTGTGT\n\nAAATTTGG\nATG");
        let fasta = FastaSeq::from_string(seq, SeqType::DNA, Alphabet::IUPACNucleicAcid)
            .expect("Couldn't create FASTA sequence");

        assert_eq!(fasta.id(), "Seq1");
        assert_eq!(fasta.desc(), None);
        assert_eq!(fasta.sequence(), "ACTGGGTGTGTAAATTTGGATG");
        assert_eq!(fasta.alphabet(), Alphabet::IUPACNucleicAcid);
        assert_eq!(fasta.seq_type(), SeqType::DNA);
        assert_eq!(fasta.len(), fasta.sequence().len());
    }

    #[test]
    fn create_seq_from_string_inferred() {
        // Default, should be inferred as DNA
        let seq = String::from("\n>Seq1\n\nACTGCATT");
        let fasta = FastaSeq::from_string_inferred(seq).expect("Couldn't create FASTA sequence");

        assert_eq!(fasta.alphabet(), Alphabet::IUPACNucleicAcid);
        assert_eq!(fasta.seq_type(), SeqType::DNA);

        // Should be inferred as RNA
        let seq = String::from("\n>Seq1\n\nACUGCAuu\n");
        let fasta = FastaSeq::from_string_inferred(seq).expect("Couldn't create FASTA sequence");

        assert_eq!(fasta.alphabet(), Alphabet::IUPACNucleicAcid);
        assert_eq!(fasta.seq_type(), SeqType::RNA);

        // Should be inferred as Protein
        let seq = String::from("\n>Seq1\n\nYWATTVEIL\n");
        let fasta = FastaSeq::from_string_inferred(seq).expect("Couldn't create FASTA sequence");

        assert_eq!(fasta.alphabet(), Alphabet::IUPACProtein);
        assert_eq!(fasta.seq_type(), SeqType::Protein);

        // Should be inferred as DNA, the default since we can't differentiate between DNA/RNA
        let seq = String::from("\n>Seq1\n\nACAABBV\n");
        let fasta = FastaSeq::from_string_inferred(seq).expect("Couldn't create FASTA sequence");

        assert_eq!(fasta.alphabet(), Alphabet::IUPACNucleicAcid);
        assert_eq!(fasta.seq_type(), SeqType::DNA);

        // Should be inferred as DNA, although it looks like a protein, because we can't differentiate
        // and that's the default
        let seq = String::from("\n>Seq1\n\nATYYVHHR\n");
        let fasta = FastaSeq::from_string_inferred(seq).expect("Couldn't create FASTA sequence");

        assert_eq!(fasta.alphabet(), Alphabet::default());
        assert_eq!(fasta.seq_type(), SeqType::default());
    }
}
