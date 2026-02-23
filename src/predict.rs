use crate::kmerionise::uniquehashes;
use smartcore::linalg::basic::matrix::DenseMatrix;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
codeprog@icloud.com
*/

pub fn predict_hashes(
    pathfile: &str,
    kmerstring: &str,
) -> Result<DenseMatrix<f64>, Box<dyn Error>> {
    let filepath = File::open(pathfile).expect("file not present");
    let fileopen = BufReader::new(filepath);
    let sequencehash = uniquehashes(kmerstring).unwrap();
    let mut valueseq: Vec<(String, usize)> = Vec::new();
    for i in fileopen.lines() {
        let lineseq = i.expect("file not present");
        let seqhash = lineseq
            .as_bytes()
            .windows(kmerstring.parse::<usize>().unwrap())
            .map(|x| str::from_utf8(x).unwrap())
            .collect::<Vec<_>>();
        let mut counthash = 0usize;
        for i in seqhash.iter() {
            for val in sequencehash.iter() {
                if *val == i.to_string() {
                    counthash += 1usize;
                }
            }
        }
        let valueinsert: (String, usize) = (lineseq, counthash);
        valueseq.push(valueinsert);
    }

    let mut cloneable: Vec<Vec<f64>> = Vec::new();
    for i in valueseq.iter() {
        let valueunwrap = i.0.clone();
        let valueseq = valueunwrap.chars().collect::<Vec<_>>();
        for ichar in valueseq.iter() {
            let mut count_a = 0usize;
            let mut count_t = 0usize;
            let mut count_g = 0usize;
            let mut count_c = 0usize;
            match ichar {
                'A' => count_a += 1usize,
                'T' => count_t += 1usize,
                'G' => count_g += 1usize,
                'C' => count_c += 1usize,
                _ => continue,
            }
            let mut vecname: Vec<f64> = Vec::new();
            vecname.push(count_a as f64);
            vecname.push(count_c as f64);
            vecname.push(count_t as f64);
            vecname.push(count_g as f64);
            vecname.push(
                count_a as f64
                    + count_t as f64
                        / (count_a as f64 + count_t as f64 + count_g as f64 + count_c as f64)
                        * 100 as f64,
            );
            vecname.push(
                count_a as f64 * 313.2
                    + count_t as f64 * 304.2
                        / (count_c as f64 * 289.2
                            + count_a as f64 * 313.2 as f64
                            + count_t as f64 * 304.2
                            + count_g as f64 * 329.2),
            );
            vecname.push(count_a as f64 + count_t as f64 * 2 as f64);
            vecname.push(count_c as f64 + count_g as f64 * 3 as f64);
            vecname.push(i.1 as f64);
            cloneable.push(vecname);
        }
    }

    let densematrix = DenseMatrix::from_2d_vec(&cloneable).unwrap();
    Ok(densematrix)
}
