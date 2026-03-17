use noodles_fasta as fasta;
use noodles_fasta::record::{Definition, Sequence};

fn main() {
    // --- 1. Build a FASTA record manually ---

    // Definition holds the sequence name and optional description
    let definition = Definition::new("chr1", Some("Homo sapiens chromosome 1".into()));

    // Sequence holds the raw nucleotide/amino acid bytes
    let sequence = Sequence::from(b"ATCGATCGNNNATCG".to_vec());

    // Combine into a Record
    let record = fasta::Record::new(definition, sequence);

    // Access fields
    //println!("Name:     {}", std::str::from_utf8(record.name()).unwrap());
    println!("Name:     {}", std::str::from_utf8(record.name()).unwrap());
    println!("Desc:     {:?}", record.description());         // Some("Homo sapiens chromosome 1")
    println!("Sequence: {}", std::str::from_utf8(record.sequence().as_ref()).unwrap());

}