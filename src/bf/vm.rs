use std::vec::Vec;
use std::io::Read;
use std::collections::linked_list::LinkedList;

struct bracket_marker {
    index: usize,
}


pub struct BFVirtualMachine {
    stack: LinkedList<bracket_marker>,
    data_index: isize,
    non_neg: Vec<u8>,
    neg: Vec<u8>,
}


impl BFVirtualMachine {
    pub fn new() -> BFVirtualMachine {
        let vm = BFVirtualMachine{
            stack: LinkedList::new(),
            data_index: 0,
            non_neg : Vec::new(),
            neg: Vec::new(),
        };
        vm.non_neg.resize(16, 0);
        vm.neg.resize(16,0);
        vm
    }

    pub fn run_from_string(&mut self, prog: String) -> Result<i32, String> {
        let bytes = prog.into_bytes(); // Only care about a few ascii characters
        let mut index:usize = 0;
        while index < bytes.len() {
            match bytes[index] as char {
                '<' => self.mov_left(),
                '>' => self.mov_right(),
                '+' => self.inc_val(),
                '-' => self.dec_val(),
                '.' => self.output(),
                ',' => self.input(),
                '[' => index = self.start_loop(),
                ']' => index = self.end_loop(),
            }
        }
        Ok(0)
    }

    fn mov_left(&mut self) {
        self.data_index -= 1;
        if self.data_index < 0 {
            if self.neg.len() < self.data_index.abs() as usize {
                self.neg.resize(self.neg.len() * 2, 0);
            }
        }
    }

    fn mov_right(&mut self) {
        self.data_index += 1;
        if self.data_index > 0 {
            if self.non_neg.len() < self.data_index.abs() as usize {
                self.non_neg.resize(self.non_neg.len() * 2, 0);
            }
        }
    }

    fn get_val(&mut self) -> u8 {
        if self.data_index >= 0 {
            return self.non_neg[self.data_index.abs() as usize];
        }
        else
        {
            return self.neg[(self.data_index.abs() - 1) as usize];
        }
    }

    fn set_val(&mut self, val: u8) {
        if self.data_index >= 0 {
            self.non_neg[self.data_index.abs() as usize] = val;
        }
        else
        {
            self.neg[(self.data_index.abs() - 1) as usize] = val;
        }
    }

    fn inc_val(&mut self) {
        self.set_val(self.get_val() + 1);
    }

    fn dec_val(&mut self) {
        self.set_val(self.get_val() - 1);
    }

    fn output(&self) {
        print!("{}", self.get_val() as char);
    }

    fn input(&mut self) {
        let input: Option<u8> = std::io::stdin()
            .bytes()
            .next()
            .and_then(|result| result.ok())
            .map(|byte| byte as u8);
        match input {
            Some(data) => self.set_val(data),
            None => panic!("Failed to read data"),
        }
    }
}