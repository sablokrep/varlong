mod args;
use crate::args::CommandParse;
use crate::args::Commands;
use clap::Parser;
mod kmerionise;
mod minimap;
mod ontstruct;
mod remove;
use crate::remove::mapper;
mod hash;
mod predict;
use self::predict::predict_hashes;
use crate::hash::cal_hashes;
use smartcore::ensemble::random_forest_classifier::RandomForestClassifier;
use smartcore::linear::logistic_regression::LogisticRegression;
use smartcore::neighbors::knn_classifier::KNNClassifier;
use std::fs::File;
use std::io::Write;

/*
Author Gaurav Sablok,
Email: codeprog@icloud.com
*/

fn main() {
    let argparse = CommandParse::parse();
    match &argparse.command {
        Commands::Threadedremoval {
            pathfile,
            denosfile,
            threadnt,
        } => {
            let command = mapper(pathfile, denosfile, threadnt).unwrap();
            println!("The command has finished:{}", command);
        }
        Commands::MachineLearn {
            path,
            kmerhash,
            threshold,
        } => {
            let command = cal_hashes(path, kmerhash, threshold).unwrap();
            let predictvalue = predict_hashes(path, kmerhash).unwrap();
            let logisticval =
                LogisticRegression::fit(&command.0, &command.1, Default::default()).unwrap();
            let logisticpredict = logisticval.predict(&predictvalue).unwrap();
            let randomclass =
                RandomForestClassifier::fit(&command.0, &command.1, Default::default()).unwrap();
            let randompredict = randomclass.predict(&predictvalue).unwrap();
            let knnclass = KNNClassifier::fit(&command.0, &command.1, Default::default()).unwrap();
            let knnpredict = knnclass.predict(&predictvalue).unwrap();

            let mut filewrite_1 = File::create("logclass.txt").expect("file not present");
            for i in logisticpredict.iter() {
                writeln!(filewrite_1, "{}", i).expect("line not present");
            }

            let mut filewrite_2 = File::create("randomclass.txt").expect("file not present");
            for i in randompredict.iter() {
                writeln!(filewrite_2, "{}", i).expect("file not present");
            }

            let mut filewrite_3 = File::create("knnclass.txt").expect("file not present");
            for i in knnpredict.iter() {
                writeln!(filewrite_3, "{}", i).expect("file not present");
            }

            println!("The command has finished");
        }
    }
}
