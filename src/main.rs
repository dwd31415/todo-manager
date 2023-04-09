use clap::Parser;
use std::fs::metadata;
use std::fs;
use glob::glob;
/**
Application to automatically format todo list written in code.
*/
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path(s) to the files to be scanned for TODO-items.
    /// Should be provided as patterns, e.g. **/*.tex to get all .tex files.
    #[clap(short, long, num_args = 0..)]
    input_files: Vec<String>,

    /// Name of the output file. 
    /// Note: This file should already exist and contain the macro "@TODO-List" somewhere.
    #[arg(short, long="output")]
    output_file: std::path::PathBuf
}

fn main() -> std::io::Result<()>{
    let args = Args::parse();
    let mut files = Vec::<std::path::PathBuf>::new();
    let mut paths = Vec::<std::path::PathBuf>::new();
    for file_name in args.input_files {
        for entry in glob(file_name.as_str()).expect("Failed to read pattern ") {
            match entry {
                Ok(path) => paths.push(path),
                Err(e) => println!("Error ocurred: {:?}", e),
            }
        }
    }
    for path in &paths{
        let meta = metadata(&path)?;
        if meta.is_file(){
            files.push(path.clone());
        }
        if meta.is_dir(){
            let paths = fs::read_dir(&path).expect(format!("Could not read file {}.", path.display()).as_str());
            for path in paths {
                println!("Name: {}", path.unwrap().path().display())
            }
        }
    }

    let mut markdown_code = "".to_owned();
    for file_name in files{
        let content = std::fs::read_to_string(&file_name).expect("could not read file");
        for line in content.lines() {
            if line.contains("@TODO:") {
                let mut parts: Vec<&str> = line.split("@TODO:").collect();
                parts.drain(..1);
                for part in parts {
                    markdown_code.push_str("- [ ]");
                    markdown_code.push_str(part);
                    markdown_code.push_str("\n");
                }
            }
        }
    }
    println!("{}", markdown_code);
    Ok(())
}
