//! Symbol: A simple visual programming language based on <><

mod state;

use std::fs::File;
use std::io::{Read,Write};

/// lines converts a Vector containing file data into vectors of lines
/// each element is a vector of chars and the line length for easy access
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

fn write_error(s : &str) {
    std::io::stderr()
       .write(s
       .as_bytes())
       .unwrap();
       std::process::exit(1);
}

fn print_char(val : i32) {
    let buf = [val as u8; 1];
    std::io::stdout().write(&buf).unwrap();
}

fn read_char() -> i32 {
    let mut buf = [0 as u8; 1];
    let len = std::io::stdin().read(&mut buf).unwrap();
    if len == 0 {
        return -1;
    }
    return buf[0] as i32;
}


fn main() {
    let fname_option = std::env::args().nth(1);
    if fname_option.is_none() {
        write_error("Please provide a file name!\nUsage: ./symbol <filename>\n");
    }
    let fname = fname_option.unwrap();
    let mut f = File::open(fname).unwrap();
    let mut read_in = Vec::new();
    f.read_to_end(&mut read_in).unwrap();
    let lines_in = lines(read_in);
    let numrows = lines_in.iter().count() as i32;
    let mut st = state::State::new();

    loop {
        let target_char = lines_in[st.ypos as usize].0[st.xpos as usize];
        //println!("Executing character: {}", target_char);
        match target_char {
            '>' => {
                st.dir = state::Direction::Right;
            },
            '<' => {
                st.dir = state::Direction::Left;
            },
            '^' => {
                st.dir = state::Direction::Up;
            },
            'v' => {
                st.dir = state::Direction::Down;
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
            'p' => { st.pop(); },
            '@' => st.dup(),
            '+' => st.add(),
            '-' => st.sub(),
            '*' => st.mult(),
            '/' => st.div(),
            ';' => std::process::exit(0),
            '|' => {
                if st.is_zero() {
                    st.dir = state::Direction::Up;
                }
                else {
                    st.dir = state::Direction::Down;
                }
            },
            '_' => {
                if st.is_zero() {
                    st.dir = state::Direction::Left;
                }
                else {
                    st.dir = state::Direction::Right;
                }
            },
            'e' => st.is_empty(),
            '!' => st.logical_neg(),
            '~' => st.negate(),
            '.' => print_char(st.pop()),
            '$' => st.push(read_char()),
            'g' => st.is_gt(),
            'l' => st.is_lt(),
            ' ' => (),
            ',' => (),
             _  => {
                 write_error(&format!("Unknown Character in program: '{}' \n", target_char));
                },
        }
        let (new_y, new_x) = st.next_step();
        if new_y < 0 || new_y >= numrows {
            std::process::exit(0);
        }
        if new_x < 0 || new_x >= lines_in[new_y as usize].1 {
            std::process::exit(0);
        }
    }
}
