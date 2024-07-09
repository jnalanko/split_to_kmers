use jseqio::writer::SeqRecordWriter;

fn main() {
    let infile = std::env::args().nth(1).unwrap();
    let outfile = std::env::args().nth(2).unwrap();
    let k: usize = std::env::args().nth(3).unwrap().parse().unwrap();

    let mut reader = jseqio::reader::DynamicFastXReader::from_file(&infile).unwrap();
    let mut writer = jseqio::writer::DynamicFastXWriter::new_to_file(&outfile).unwrap();

    while let Some(rec) = reader.read_next().unwrap() {
        //for kmer in rec.seq.windows(k) {
        for i in 0..(rec.seq.len() as isize -k as isize +1) as usize {
            writer.write_owned_record(
                &jseqio::record::OwnedRecord{head: rec.head.to_vec(), seq: rec.seq[i..i+k].to_vec(), qual: rec.qual.and_then(|s| Some(s[i..i+k].to_vec()))}
            ).unwrap();
        }
    }
}
