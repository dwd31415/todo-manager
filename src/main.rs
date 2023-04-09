use clap::Parser;
/**
Application to automatically format todo list written in code.
*/
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path(s) to the files to be scanned for TODO-items.
    #[clap(short, long, num_args = 0..)]
    input_files: Vec<std::path::PathBuf>,

    /// Name of the output file. 
    /// Note: This file should already exist and contain the macro "@TODO-List" somewhere.
    #[arg(short, long="output")]
    output_file: std::path::PathBuf
}

fn main() {
    let args = Args::parse();
}
