mod bf;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::env;
use std::path::Path;

use bf::BFVirtualMachine;

fn main() {
    let args:Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", Path::new(&args[0]).file_name().expect("Internal Error").to_str().expect(""));
        return;
    }

    let file = File::open(&args[1]).expect("Filed to open file");
    let file_data = BufReader::new(file);

    let mut vm  = BFVirtualMachine::new();

    for line in file_data.lines() {
        for c in line.expect("Unable to read line").chars() {
            match vm.parse_char(c) {
                Err(why) => panic!("{}", why),
                Ok(ret_val) => {
                    assert!(ret_val == 0);
                }
            }
        }
    }

    return;
}
