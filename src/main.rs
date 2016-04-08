use std::fs::File;
use std::io::{Read,Write};

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct State {
    xpos: i32,
    ypos: i32,
    dir: Direction,
    stack: Vec<i32>,
}

impl State {
    fn push(&mut self, val: i32) {
        self.push(val);
    }

    fn pop(&mut self) -> i32 {
        self.pop()
    }

    fn add(&mut self) {
        let a1 = self.pop();
        let a2 = self.pop();
        self.push(a1+a2);
    }

    fn sub(&mut self) {
        let a1 = self.pop();
        let a2 = self.pop();
        self.push(a1-a2);
    }

    fn mult(&mut self) {
        let a1 = self.pop();
        let a2 = self.pop();
        self.push(a1*a2);
    }

    fn div(&mut self) {
        let a1 = self.pop();
        let a2 = self.pop();
        self.push(a1/a2);
    }

    fn dup(&mut self) {
        let top = self.pop();
        self.push(top);
        self.push(top);
    }

}

/* lines converts a Vector containing file data into vectors of lines
 * each element is a vector of chars and the line length for easy access */
fn lines(v : Vec<u8>) -> Vec<(Vec<char>, i32)> {
    let mut ret = Vec::new();
    let mut new_line = Vec::new();
    for ch in v {
        if ch as char == '\n' {
            let length = new_line.iter().count();
            ret.push((new_line, length as i32));
            new_line = Vec::new();
        }
        else {
            new_line.push(ch as char);
        }
    }
    ret
}


fn main() {
    let mut f = File::open("tempy.txt").unwrap();
    let mut read_in = Vec::new();
    f.read_to_end(&mut read_in).unwrap();
    let lines_in = lines(read_in);
    let mut st = State { xpos: 0, ypos: 0, dir: Direction::Left, stack: Vec::new()};

    while true {
        let target_char = lines_in[st.ypos as usize].0[st.xpos as usize];
        match target_char {
            '>' => println!("Hit >"),
            '<' => println!("Hit <"),
             _  => {
                 std::io::stderr()
                    .write(format!("Unknown Character in program: '{}' ", target_char)
                    .as_bytes())
                    .unwrap();
                    std::process::exit(1);
                },
        }
    }
}
