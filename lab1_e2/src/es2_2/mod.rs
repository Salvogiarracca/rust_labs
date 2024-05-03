use std::fmt::{Display, Formatter};
use std::time::SystemTime;

use crate::es2_2::Error::{Complex, Simple};

enum Error {
    Simple(SystemTime),
    Complex(SystemTime, String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Simple(time) => { write!(f, "Error occurred at time: {:?}", time) }
            Complex(time, message) => { write!(f, "Error occurred at time: {:?} Message: {}", time, message) }
        }
    }
}

fn print_error(e: Error) {
    println!("{}", e)
}

pub fn es2_2() {
    let simple_error = Simple(SystemTime::now());
    let complex_error = Complex(SystemTime::now(), String::from("This is a complex error"));

    print_error(simple_error);
    print_error(complex_error);
}
