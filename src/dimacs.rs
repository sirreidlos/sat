use std::io::{BufRead, Read};
use std::num::ParseIntError;
use std::{fs::File, io::BufReader};

#[derive(Debug)]
pub struct InputVariableState {
    pub idx: usize,
    pub nonnegated: bool,
}

#[derive(Debug)]
pub struct InputClause(Vec<InputVariableState>);

impl InputClause {
    pub fn iter(&self) -> ClauseIter {
        ClauseIter {
            clause: self,
            idx: 0,
        }
    }
}

pub struct ClauseIter<'a> {
    clause: &'a InputClause,
    idx: usize,
}

impl<'a> Iterator for ClauseIter<'a> {
    type Item = &'a InputVariableState;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.clause.0.len() {
            return None;
        }

        let output = &self.clause.0[self.idx];
        self.idx += 1;

        Some(output)
    }
}

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

        // println!("{header}");

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

        // println!("{} {}", input, input.trim().is_empty());
        while input.starts_with('c') || input.trim().is_empty() {
            // println!("{input}");
            input.truncate(0);
            self.reader.read_line(&mut input).ok()?;
        }

        let variables: Vec<Result<Option<InputVariableState>, ParseIntError>> = input
            .split_whitespace()
            .map(|n| {
                let mut variable = n.parse::<isize>()?;
                let mut negated = false;

                if variable == 0 {
                    return Ok(None);
                }

                if variable < 0 {
                    negated = true;
                    variable *= -1;
                }

                Ok(Some(InputVariableState {
                    idx: (variable - 1) as usize,
                    nonnegated: !negated,
                }))
            })
            .collect();

        let variables = variables
            .iter()
            .all(|e| e.is_ok())
            .then(|| variables.into_iter().map(|v| v.unwrap()))?
            .flatten()
            .collect();

        Some(InputClause(variables))
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
