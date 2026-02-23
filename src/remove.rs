use crate::minimap::readfasta;
use crate::ontstruct::FASTA;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::Command;

/*
Author Gaurav Sablok,
Email: codeprog@icloud.com
*/

pub fn mapper(pathfile: &str, pathnos: &str, threadnum: &str) -> Result<String, Box<dyn Error>> {
    let mapfasta = readfasta(pathfile).unwrap();
    let mut filewrite = File::create("mapfasta.fasta").expect("file not present");
    for i in mapfasta.iter() {
        writeln!(filewrite, ">{}\t{}", i.id, i.sequence).expect("file not present");
    }

    let _ = Command::new("minimap")
        .arg(pathfile)
        .arg(pathnos)
        .arg(">")
        .arg(threadnum)
        .arg("mapped.sam")
        .output()
        .expect("commandfailed");

    let samopen = File::open("mapped.sam").expect("file not present");
    let samread = BufReader::new(samopen);
    // making a iterative vector for the search
    let mut ontsearch: Vec<(String, String, String)> = Vec::new();
    for i in samread.lines() {
        let line = i.expect("line not present");
        if !line.starts_with("@") {
            let linevec = line.split("\t").map(|x| x.to_string()).collect::<Vec<_>>();
            let vecinsert: (String, String, String) = (
                linevec[0].to_string(),
                linevec[3].to_string(),
                linevec[9].len().to_string(),
            );
            ontsearch.push(vecinsert);
        }
    }

    let mut hashvecid = HashSet::new();
    for i in ontsearch.iter() {
        hashvecid.insert(i.0.clone());
    }

    let mut mappingstruct: Vec<FASTA> = Vec::new();
    for i in hashvecid.iter() {
        for j in mapfasta.iter() {
            if j.id.clone() == *i {
                mappingstruct.push(FASTA {
                    id: i.to_string(),
                    sequence: j.sequence.clone(),
                })
            }
        }
    }

    let mut finalseq: Vec<FASTA> = Vec::new();
    for i in mappingstruct.iter() {
        let mut finalvec = String::from("");
        for j in ontsearch.iter() {
            if j.0.clone().to_string() == *i.id {
                let sequence = i.sequence.clone();
                let mut startvec: Vec<usize> = Vec::new();
                let mut endvec: Vec<usize> = Vec::new();
                startvec.push(j.1.parse::<usize>().unwrap() - 1);
                endvec.push(j.2.parse::<usize>().unwrap());
                let lengthvec = startvec.len();
                while lengthvec <= startvec.len() {
                    let new_seq = sequence[startvec[lengthvec]..endvec[lengthvec]].to_string();
                    finalvec.push_str(&new_seq);
                }
            }
        }
        finalseq.push(FASTA {
            id: i.id.clone(),
            sequence: finalvec,
        })
    }

    // stats for the merged count
    let mut count: usize = 0usize;
    let mut countgap: usize = 0usize;
    for i in ontsearch.iter() {
        let basecount = i.1.split("\t").collect::<Vec<_>>()[9]
            .parse::<usize>()
            .unwrap();
        count += basecount;
        let countg = i.1.split("\t").collect::<Vec<_>>()[10]
            .parse::<usize>()
            .unwrap();
        countgap += countg;
    }

    let mut veccount: Vec<_> = Vec::new();
    let mut vecseq: Vec<usize> = Vec::new();
    for i in ontsearch.iter() {
        veccount.push(i.0.clone());
        vecseq.push(i.1.len());
    }
    let writesec = vecseq.iter().fold(0, |acc, x| acc + x);
    let mut statfile = File::create("statfile.txt").expect("file not present");
    writeln!(statfile, "{}", "The stats for the given file are:").expect("file not present");
    writeln!(
        statfile,
        "Number of sequences:{}\tTotal bases:{}\tNumber of matching bases:{}\tNumber of bases including gaps:{}",
        veccount.len(),
        writesec.to_string(),
        count,
        countgap
    ).expect("file not written");

    Ok("Mapping has been finished".to_string())
}
