use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "varlog",
    version = "1.0",
    about = "     varlog: Bees olfactory clasification
               based on sequence to chemical bond approach
       ************************************************
       Author Gaurav Sablok,
       Email: codeprog@icloud.com
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// remove Classified
    Threadedremoval {
        /// provide ONT file
        pathfile: String,
        /// path olfactor file
        denosfile: String,
        /// number of thread for minimap
        threadnt: String,
    },
    /// machine learning
    MachineLearn {
        /// provide ONT or the PacbioHifi Fasta file.
        path: String,
        /// kmer hash to use
        kmerhash: String,
        /// threshold to use for the calculation
        threshold: String,
    },
}
