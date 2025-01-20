use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub fn parse_request(stream: &mut TcpStream) -> Result<Vec<String>, String> {
    let buf_reader = BufReader::new(stream);
    let lines = buf_reader.lines();


    let mut vec_lines = vec![];

    for line in lines {
        match line {
            Ok(line_content) => {
                if line_content.is_empty() {
                    break;
                }
                vec_lines.push(line_content);
            },
            Err(e) => {
                return Err(format!("Failed to read line: {}", e));
            }
        }
    }

     Ok(vec_lines)
           
} 
   
