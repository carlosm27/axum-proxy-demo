use std::fs;
use std::io::BufReader; 
use std::io::BufRead;
use std::io;


pub fn read_file_lines_to_vec(filename: &str) -> io::Result<Vec<String>> { 
    let file_in = fs::File::open(filename)?; 
    let file_reader = BufReader::new(file_in); 
    Ok(file_reader.lines().filter_map(io::Result::ok).collect()) 
} 

pub fn check_address_block(address_to_check: &str) -> bool {
    let addresses_blocked = read_file_lines_to_vec(&"./blacklist.txt".to_string());
    let addresses_blocked_iter: Vec<String> = match addresses_blocked {
        Ok(vector) => vector,
        Err(_) => vec!["Error".to_string()]
    };
    
    let address_in = addresses_blocked_iter.contains(&address_to_check.to_string());
    return address_in
    

}