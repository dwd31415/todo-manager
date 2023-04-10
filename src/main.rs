use clap::Parser;
use pathdiff::diff_paths;
use std::fs::metadata;
use std::fs;
use glob::glob;
use regex::{self, Regex};


/**
Application to automatically format a todo list written in code.
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
    output_file: std::path::PathBuf,

    /// Option to display the Markdown code in the terminal. 
    /// This can be used in case the output file does not contain the "@TODO-List" macro. 
    /// The paths are still given relative to the output file.
    #[arg(short, long="display", default_value_t=false)]
    display_markdown: bool,
}

fn main() -> std::io::Result<()>{
    let args = Args::parse();
    let mut files = Vec::<std::path::PathBuf>::new();
    let mut paths = Vec::<std::path::PathBuf>::new();

    // Compute the base path for the links to the TODO items.
    let output_meta = metadata(&args.output_file)?;
    if !(output_meta.is_file()){
        println!("Error: The output file does not exist or is a directory.");
        return Ok(());
    }
    let output_file_fullpath = args.output_file.canonicalize().unwrap();
    let base_path = output_file_fullpath.parent().expect(
            format!("Error: The output file {} is an invalid path.", args.output_file.display()).as_str());

    // Extract all the input pattern and convert them to paths
    for file_name in args.input_files {
        for entry in glob(file_name.as_str()).expect("Failed to read pattern ") {
            match entry {
                Ok(path) => paths.push(path),
                Err(e) => println!("Error ocurred: {:?}", e),
            }
        }
    }

    // Generate a list of all files that should be scanned.
    for path in &paths{
        let meta = metadata(&path)?;
        if meta.is_file(){
            files.push(path.clone());
        }
        if meta.is_dir(){
            let paths = fs::read_dir(&path)
                .expect(format!("Error: Could not read file {}.", path.display()).as_str());
            for path in paths {
                println!("Name: {}", path.unwrap().path().display())
            }
        }
    }

    // Otherwise the ordering might differ from run to run 
    files.sort();

    // Generate Markdown code 
    let mut markdown_code = "<!---@TODO-List-Start--->\n".to_owned();
    for file_name in files{
        let content = std::fs::read_to_string(&file_name)
            .expect(format!("Error: Could not read file {}.", &file_name.display()).as_str());

        let relative_path = diff_paths(&file_name.canonicalize().unwrap(), &base_path.canonicalize().unwrap())
            .expect(format!("Error: Unexpected error while computing the relative path of the output file.").as_str());
        let mut line_counter = 1;
        for line in content.lines() {
            if line.contains("@TODO:") {
                let mut parts: Vec<&str> = line.split("@TODO:").collect();
                parts.drain(..1);
                for part in parts {
                    markdown_code.push_str("- [ ] •");
                    markdown_code.push_str(part);
                    markdown_code.push_str(format!(" [See in file]({}#L{})", relative_path.display(),line_counter).as_str());
                    markdown_code.push_str("\n");
                }
            }
            line_counter += 1;
        }
    }
    markdown_code.push_str("<!---@TODO-List-End--->");

    // Display markdown code to user (optional)
    if args.display_markdown{
        println!("Markdown code for TODO list:");
        println!("{}", markdown_code);
    }

    // Write Markdown code into output file.
    let regex_macro = Regex::new(r"<!\-\-\-@TODO\-List\-Start\-\-\->(?s:.+)<!\-\-\-@TODO\-List\-End\-\-\->").unwrap();
    let content_output = std::fs::read_to_string(&args.output_file)
        .expect(format!("Error: Could not read file {}.",args.output_file.display()).as_str());
    if content_output.contains("<!---@TODO-List-Start--->") && content_output.contains("<!---@TODO-List-End--->"){
        let processed = regex_macro.replace_all(content_output.as_str(), &markdown_code);
        let error_msg = format!("Error: Could not write to file {}.", args.output_file.display());
        fs::write(args.output_file, processed.as_ref()).expect(error_msg.as_str()); 
    }
    else{
        println!("Error: The output file {} doesn't contain the macros <!---@TODO-List-Start---> and <!---@TODO-List-End--->.\nPlease add this somewhere.", args.output_file.display());
    }
    Ok(())
}
