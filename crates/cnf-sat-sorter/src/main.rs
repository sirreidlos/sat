use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn main() -> io::Result<()> {
    // Get the input and output file paths from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input.cnf> <output.cnf>", args[0]);
        return Ok(());
    }
    let input_path = &args[1];
    let output_path = &args[2];

    // Open the input file
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);

    // Read the header line and the clauses
    let mut header = String::new();
    let mut clauses = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.starts_with('p') {
            header = line;
        } else if !line.trim().is_empty() {
            clauses.push(line);
        }
    }

    // Sort the clauses by length (number of literals)
    clauses.sort_by_key(|clause| clause.split_whitespace().count());

    // Open the output file
    let mut output_file = File::create(output_path)?;

    // Write the header and sorted clauses to the output file
    writeln!(output_file, "{}", header)?;
    for clause in clauses {
        writeln!(output_file, "{}", clause)?;
    }

    Ok(())
}
