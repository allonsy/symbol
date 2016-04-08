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
        self.stack.push(val);
    }

    fn pop(&mut self) -> i32 {
        let res = self.stack.pop();
        if std::option::Option::is_none(&res) {
            std::io::stderr()
               .write("Stack is empty!"
               .as_bytes())
               .unwrap();
               std::process::exit(1);
        }
        res.unwrap()
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

    fn is_zero(&mut self) -> bool {
        let top = self.pop();
        top == 0
    }

    fn is_gt(&mut self) -> bool {
        let top = self.pop();
        top > 0
    }

    fn is_lt(&mut self) -> bool {
        let top = self.pop();
        top < 0
    }

    fn negate(&mut self) {
        let top = self.pop();
        self.push(-1 * top);
    }

    fn next_step(&mut self) -> (i32, i32) {
        match self.dir {
            Direction::Up => {
                self.ypos -= 1;
            },
            Direction::Down => {
                self.ypos += 1;
            },
            Direction::Left => {
                self.xpos -= 1;
            },
            Direction::Right => {
                self.xpos += 1;
            }
        }
        (self.ypos, self.xpos)
    }

    fn logical_neg(&mut self) {
        let top = self.pop();
        if top != 0 {
            self.push(0);
        }
        else {
            self.push(1);
        }
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
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

fn print_char(val : i32) {
    let mut buf = [val as u8; 1];
    std::io::stdout.write(&buf).unwrap();
}

fn read_char() -> i32 {
    0
}


fn main() {
    let mut f = File::open("tempy.txt").unwrap();
    let mut read_in = Vec::new();
    f.read_to_end(&mut read_in).unwrap();
    let lines_in = lines(read_in);
    let mut st = State { xpos: 0, ypos: 0, dir: Direction::Right, stack: Vec::new()};

    while true {
        let target_char = lines_in[st.ypos as usize].0[st.xpos as usize];
        match target_char {
            '>' => {
                st.dir = Direction::Right;
            },
            '<' => {
                st.dir = Direction::Left;
            },
            '^' => {
                st.dir = Direction.Up;
            },
            'v' => {
                st.dir = Direction.Down;
            },
            '0' => st.push(0),
            '1' => st.push(1),
            '2' => st.push(2),
            '3' => st.push(3),
            '4' => st.push(4),
            '5' => st.push(5),
            '6' => st.push(6),
            '7' => st.push(7),
            '8' => st.push(8),
            '9' => st.push(9),
            '@' => st.dup(),
            '+' => st.add(),
            '-' => st.sub(),
            '*' => st.mult(),
            '/' => st.div(),
            ';' => std::process:exit(0),

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
