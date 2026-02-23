use crate::ontstruct::FASTA;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Author Gaurav Sablok,
Email: codeprog@icloud.com
*/

pub fn readfasta(pathfile: &str) -> Result<Vec<FASTA>, Box<dyn Error>> {
    let fileopen = File::open(pathfile).expect("file not present");
    let fileread = BufReader::new(fileopen);
    let mut fastavec: Vec<FASTA> = Vec::new();
    let mut id: Vec<String> = Vec::new();
    let mut sequence: Vec<String> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("line not present");
        if line.starts_with(">") {
            id.push(line.replace(">", ""));
        } else if !line.starts_with("#") {
            sequence.push(line);
        }
    }
    for i in 0..id.len() {
        fastavec.push(FASTA {
            id: id[i].clone().to_string(),
            sequence: sequence[i].clone().to_string(),
        })
    }

    Ok(fastavec)
}
