use std::io::{BufRead, Read};
use std::num::ParseIntError;
use std::{fs::File, io::BufReader};

type InputVariable = isize;
pub type InputClause = Vec<InputVariable>;

pub struct Parser<R: Read> {
    variables: usize,
    clauses: usize,
    reader: BufReader<R>,
}

impl Parser<File> {
    pub fn new(input_file: File) -> Self {
        let mut reader = BufReader::new(input_file);

        let mut header = String::new();
        reader.read_line(&mut header).unwrap();

        // This is to skip the comments and whitespaces before the header
        // which usually starts with 'p' for problem
        while !header.starts_with('p') {
            header.truncate(0);
            reader.read_line(&mut header).ok().unwrap();
        }

        let header_data: Vec<usize> = header
            .split_whitespace()
            .skip(2)
            .map(|c| c.parse::<usize>().unwrap())
            .collect();

        let variables = header_data[0];
        let clauses = header_data[1];

        Self {
            variables,
            clauses,
            reader,
        }
    }

    fn parse_next_line(&mut self) -> Option<InputClause> {
        let mut input = String::new();
        self.reader.read_line(&mut input).ok()?;

        while input.starts_with('c') || input.trim().is_empty() {
            input.truncate(0);
            let bytes_read = self.reader.read_line(&mut input).ok()?;
            if bytes_read == 0 {
                return None;
            }
        }

        let variables: Vec<Result<Option<InputVariable>, ParseIntError>> = input
            .split_whitespace()
            .map(|n| {
                let mut variable = n.parse::<isize>()?;
                if variable == 0 {
                    return Ok(None);
                }

                Ok(Some(variable))
            })
            .collect();

        let variables = variables
            .iter()
            .all(|e| e.is_ok())
            .then(|| variables.into_iter().map(|v| v.unwrap()))?
            .flatten()
            .collect();

        Some(variables)
    }

    pub fn get_variables(&self) -> usize {
        self.variables
    }
}

impl Iterator for Parser<File> {
    type Item = InputClause;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_next_line()
    }
}
