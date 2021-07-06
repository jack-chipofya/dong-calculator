use std::io;
//Simple Calculator in rust

//Structure of a mathematical Expression
#[derive(Debug)]
struct Expression {
    left_val: i32,
    sign: char,
    right_val: i32,
}

impl Expression {
//This function evaluates the mathematical Expression
    fn eval(&self)->(i8,i32){
        if self.sign=='+' {
            (0,self.left_val+self.right_val)
        } else if self.sign=='-' {
            (0,self.left_val-self.right_val)
        } else if self.sign=='*' {
            (0,self.left_val*self.right_val)
        } else if self.sign=='/' {
            (0,self.left_val/self.right_val)
        } else {
            (-1,0)
        }
    }
}
fn main() {
    println!("CALCULATOR");
    println!("Enter num 1");
    let mut left_val=String::new();
    io::stdin()
        .read_line(&mut left_val)
        .expect("Failed to read input");
    println!("Enter sign");
    let mut sign=String::new();
    io::stdin()
        .read_line(&mut sign)
        .expect("Failed to read input");
    println!("Enter num 2");
    let mut right_val=String::new();
    io::stdin()
        .read_line(&mut right_val)
        .expect("Failed to read input");
    let sign=sign.trim().parse().expect("Failed to convert");
    let left_val: i32=left_val.trim().parse().expect("Failed to convert");
    let right_val: i32=right_val.trim().parse().expect("Failed to convert");
    let expr: Expression=Expression{
        left_val,
        right_val,
        sign
    };
    if expr.eval().0==-1 {
        println!("Error invalid sign {}",sign);
    } else{
        println!("{}",expr.eval().1);
    }
}