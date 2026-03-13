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
    println!("Name:     {}", std::str::from_utf8(record.name()).unwrap());                  // chr1
    println!("Desc:     {:?}", record.description());         // Some("Homo sapiens chromosome 1")
    println!("Sequence: {}", std::str::from_utf8(record.sequence().as_ref()).unwrap());

    // --- 2. Write a FASTA file to stdout ---

    let stdout = std::io::stdout();
    let mut writer = fasta::io::Writer::new(stdout.lock());
    writer.write_record(&record).expect("failed to write record");
    // Output:
    // >chr1 Homo sapiens chromosome 1
    // ATCGATCGNNNATCG

    // --- 3. Read FASTA records from a byte slice ---

    let data = b">seq1 first sequence\nACGT\n>seq2\nTTTT\n";
    let mut reader = fasta::io::Reader::new(&data[..]);

    for result in reader.records() {
        let rec = result.expect("failed to read record");
        println!(
            "Read -> name={}, len={}",
            std::str::from_utf8(rec.name()).unwrap(),
            rec.sequence().len()
        );
    }
    // Read -> name=seq1, len=4
    // Read -> name=seq2, len=4

    // --- 4. Slice a sub-sequence (e.g. positions 2..6, 0-based) ---

    let seq = Sequence::from(b"AAACCCGGG".to_vec());
    let sub: &[u8] = &seq.as_ref()[2..6]; // "ACCC"
    println!("Sub-sequence: {}", std::str::from_utf8(sub).unwrap());
}