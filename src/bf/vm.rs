use std::vec::Vec;
use std::io::Read;
use std::collections::linked_list::LinkedList;

struct BracketMarker {
    index: usize,
    matching: Option<usize>,
}


pub struct BFVirtualMachine {
    stack: LinkedList<BracketMarker>,
    data_index: isize,
    non_neg: Vec<u8>,
    neg: Vec<u8>,
}


impl BFVirtualMachine {
    pub fn new() -> BFVirtualMachine {
        let mut vm = BFVirtualMachine{
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
                '[' => index = self.start_loop(index, &bytes),
                ']' => index = self.end_loop(index),
                  _ => {},
            }            
            index += 1;
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

    fn get_val(&self) -> u8 {
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
        self.set_val(self.get_val().wrapping_add(1));
    }

    fn dec_val(&mut self) {
        self.set_val(self.get_val().wrapping_sub(1));
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
            None => self.set_val(0),
        }
    }

    fn start_loop(&mut self, curr_index: usize, prog: &Vec<u8>) -> usize {
        let marker = if self.stack.len() > 0 && self.stack.front().unwrap().index == curr_index {
            self.stack.pop_front().unwrap()
        }
        else
        {
            BracketMarker{index: curr_index, matching: None}
        };

        let new_index = if self.get_val() == 0 {
            if marker.matching.is_some() {
                marker.matching.unwrap()
            }
            else {
                let mut i = curr_index + 1; // skip the current '['
                let mut nest_count = 0;
                while (i < prog.len()) && !(prog[i] as char == ']' && nest_count == 0){
                    match prog[i] as char {
                        '[' => nest_count += 1,
                        ']' => nest_count -= 1,
                        _ => {}
                    }
                    i += 1;
                }
                i
            }
        }
        else
        {
            self.stack.push_front(marker);
            curr_index
        };

        new_index
    }

    fn end_loop(&mut self, curr_index: usize) -> usize {
        let front = self.stack.front_mut();
        if front.is_some() {
            let mut m = front.unwrap();
            if m.matching.is_none() {
                m.matching = Some(curr_index);
            }
            return m.index - 1;
        }
        else
        {
            panic!("] found without starting [");
        }
    }
}