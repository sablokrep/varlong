use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/*
Gaurav Sablok
codeprog@icloud.com
*/

pub fn uniquehashes(kmerstring: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let filepath = Path::new("../kmer/all.fa");
    let fileopen = File::open(filepath).expect("file not present");
    let fileread = BufReader::new(fileopen);
    let mut stringvec: HashSet<String> = HashSet::new();
    for i in fileread.lines() {
        let line = i.expect("file not present");
        if !line.starts_with(">") {
            let valuestring = line
                .as_bytes()
                .windows(kmerstring.parse::<usize>().unwrap())
                .map(|x| str::from_utf8(x).unwrap())
                .collect::<Vec<_>>();
            for i in valuestring.iter() {
                stringvec.insert(i.to_string());
            }
        }
    }
    Ok(stringvec)
}
