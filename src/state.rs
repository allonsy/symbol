/* allonsy state.rs
* Manages state of the symbol machine */

use std;

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub struct State {
    pub xpos: i32,
    pub ypos: i32,
    pub dir: Direction,
    stack: Vec<i32>,
}

impl State {

    pub fn new() -> State {
        State {xpos: 0, ypos: 0, dir: Direction::Right, stack:Vec::new()}
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
    }

    pub fn pop(&mut self) -> i32 {
     let res = self.stack.pop();
     if std::option::Option::is_none(&res) {
         panic!("Stack is empty!");
     }
     res.unwrap()
    }

    pub fn add(&mut self) {
     let a1 = self.pop();
     let a2 = self.pop();
     self.push(a1+a2);
    }

    pub fn sub(&mut self) {
     let a1 = self.pop();
     let a2 = self.pop();
     self.push(a2-a1);
    }

    pub fn mult(&mut self) {
     let a1 = self.pop();
     let a2 = self.pop();
     self.push(a1*a2);
    }

    pub fn div(&mut self) {
     let a1 = self.pop();
     let a2 = self.pop();
     self.push(a2/a1);
    }

    pub fn dup(&mut self) {
     let top = self.pop();
     self.push(top);
     self.push(top);
    }

    pub fn is_zero(&mut self) -> bool {
     let top = self.pop();
     top == 0
    }

    pub fn is_gt(&mut self) {
     let top = self.pop();
     if top > 0 {
         self.push(1);
     }
     else {
         self.push(0);
     }
    }

    pub fn is_lt(&mut self) {
     let top = self.pop();
     if top < 0 {
         self.push(1);
     }
     else {
         self.push(0);
     }
    }

    pub fn negate(&mut self) {
        let top = self.pop();
        self.push(-1 * top);
    }

    pub fn logical_neg(&mut self) {
        let top = self.pop();
        if top != 0 {
         self.push(0);
        }
        else {
         self.push(1);
        }
    }

    pub fn is_empty(&mut self) {
        if self.stack.is_empty() {
            self.push(1);
        }
        else {
            self.push(0);
        }
    }

    pub fn next_step(&mut self) -> (i32, i32) {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut st = State::new();
        st.push(0);
        st.push(1);
        st.push(2);
        st.push(3);
        let three = st.pop();
        let two = st.pop();
        let one = st.pop();
        let zero = st.pop();

        assert_eq!(three, 3);
        assert_eq!(two, 2);
        assert_eq!(one, 1);
        assert_eq!(zero, 0);
    }

    #[test]
    #[should_panic]
    fn test_empty_pop() {
        let mut st = State::new();
        st.pop();
    }

    #[test]
    fn test_add() {
        let mut st = State::new();
        st.push(1);
        st.push(2);
        st.add();
        st.push(-3);
        st.push(5);
        st.add();
        let res1 = st.pop();
        let res2 = st.pop();

        assert_eq!(res1, 2);
        assert_eq!(res2, 3);
    }

    #[test]
    fn test_sub() {
        let mut st = State::new();
        st.push(12);
        st.push(2);
        st.sub();
        st.push(-3);
        st.push(5);
        st.sub();
        let res1 = st.pop();
        let res2 = st.pop();

        assert_eq!(res1, -8);
        assert_eq!(res2, 10);
    }

    #[test]
    fn test_mult() {
        let mut st = State::new();
        st.push(3);
        st.push(2);
        st.mult();
        st.push(-3);
        st.push(5);
        st.mult();
        let res1 = st.pop();
        let res2 = st.pop();

        assert_eq!(res1, -15);
        assert_eq!(res2, 6);
    }

    #[test]
    fn test_div() {
        let mut st = State::new();
        st.push(7);
        st.push(2);
        st.div();
        let res1 = st.pop();

        assert_eq!(res1, 3);
    }

    #[test]
    fn test_dup() {
        let mut st = State::new();
        st.push(1);
        st.dup();
        let res1 = st.pop();
        let res2 = st.pop();

        assert_eq!(res1, res2);
    }

    #[test]
    fn test_is_zero() {
        let mut st = State::new();
        st.push(1);
        let res1 = st.is_zero();
        st.push(0);
        let res2 = st.is_zero();

        assert_eq!(res1, false);
        assert_eq!(res2, true);
    }

    #[test]
    fn test_is_gt() {
        let mut st = State::new();
        st.push(1);
        st.is_gt();
        let res1 = st.pop();
        st.push(0);
        st.is_gt();
        let res2 = st.pop();
        st.push(-1);
        st.is_gt();
        let res3 = st.pop();

        assert_eq!(res1, 1);
        assert_eq!(res2, 0);
        assert_eq!(res3, 0);
    }

    #[test]
    fn test_is_lt() {
        let mut st = State::new();
        st.push(1);
        st.is_lt();
        let res1 = st.pop();
        st.push(0);
        st.is_lt();
        let res2 = st.pop();
        st.push(-1);
        st.is_lt();
        let res3 = st.pop();

        assert_eq!(res1, 0);
        assert_eq!(res2, 0);
        assert_eq!(res3, 1);
    }

    #[test]
    fn test_negate() {
        let mut st = State::new();
        st.push(1);
        st.negate();
        let res1 = st.pop();
        st.push(0);
        st.negate();
        let res2 = st.pop();
        st.push(-1);
        st.negate();
        let res3 = st.pop();

        assert_eq!(res1, -1);
        assert_eq!(res2, 0);
        assert_eq!(res3, 1);
    }

    #[test]
    fn test_logical_neg() {
        let mut st = State::new();
        st.push(2);
        st.logical_neg();
        let res1 = st.pop();
        st.push(1);
        st.logical_neg();
        let res2 = st.pop();
        st.push(0);
        st.logical_neg();
        let res3 = st.pop();
        st.push(-7);
        st.logical_neg();
        let res4 = st.pop();

        assert_eq!(res1, 0);
        assert_eq!(res2, 0);
        assert_eq!(res3, 1);
        assert_eq!(res4, 0);
    }

    #[test]
    fn test_is_empty() {
        let mut st = State::new();
        st.is_empty();
        let res1 = st.pop();
        st.push(0);
        st.is_empty();
        let res2 = st.pop();

        assert_eq!(res1, 1);
        assert_eq!(res2, 0);
    }

    #[test]
    fn test_next_step() {
        let mut st = State::new();
        st.dir = Direction::Right;
        let p1 = st.next_step();
        st.dir = Direction::Down;
        let p2 = st.next_step();
        st.dir = Direction::Left;
        let p3 = st.next_step();
        st.dir = Direction::Up;
        let p4 = st.next_step();

        assert_eq!(p1, (0,1));
        assert_eq!(p2, (1,1));
        assert_eq!(p3, (1,0));
        assert_eq!(p4, (0,0));
    }

}
