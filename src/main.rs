mod bf;

use std::io::Read;
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

    let mut file = File::open(&args[1]).expect("Filed to open file");

    let mut vm  = BFVirtualMachine::new();

    let mut prog = String::new();

    let _ = file.read_to_string(&mut prog).expect("Failed to read file");

    match vm.run_from_string(prog) {
        Ok(ret) => if ret != 0 {
            panic!("Ret code of {:?}", ret);
        },
        Err(why) => panic!("{:?}", why)
    }

    return;
}
