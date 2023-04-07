use std::fs;
use std::io::BufReader; 
use std::io::BufRead;
use std::io;


pub fn read_file_lines_to_vec(filename: &str) -> io::Result<Vec<String>> { 
    let file_in = fs::File::open(filename)?; 
    let file_reader = BufReader::new(file_in); 
    Ok(file_reader.lines().filter_map(io::Result::ok).collect()) 
} 
