use std::io::{self, Write, Error, ErrorKind};

extern crate clap;
use clap::{App, Arg};

/// Converts an integer to a string of one's and zero's
fn to_binary_string(n: u64) -> String {
    let mut out = String::new();
    let mut remainder = n;

    while remainder > 0 {
        out.push_str(&(remainder % 2).to_string());
        remainder /= 2;
    }
    
    return format!("{:0>64}", out.chars().rev().collect::<String>());
}

/// Converts a string of one's and zero's to an integer
fn from_binary_string(s: String) -> u64 {
    let mut out = 0;
    let mut power = 0;
    let mut remainder = s;

    while !remainder.is_empty() {
        let to_add: u64 = match remainder.pop() {
            Some('0') => 0,
            Some('1') => 2u64.pow(power),
            _ => panic!("Oddly, found something besides 1 or 0")
        };
        out += to_add;
        power += 1;
    }

    return out;
}

/// Shifts a string of one's and zero's left by the specified `magnitude`
fn shift_left(s: &str, magnitude: usize) -> String {
    let mut retained = s.to_string().split_off(magnitude);
    let padding = (0..magnitude).map(|_| '0').collect::<String>();
    retained.push_str(&padding);
    return retained;
}

/// Shifts a string of one's and zero's right by the specified `magnitude`
fn shift_right(s: &str, magnitude: usize) -> String {
    let mut retained = s.to_string();
    let _ = retained.split_off(64 - magnitude);
    let mut padding = (0..magnitude).map(|_| '0').collect::<String>();
    padding.push_str(&retained);
    return padding;
}

fn main() -> Result<(), Error> {
    let matches = App::new("Shift those bits!")
        .version("1.0")
        .author("Eric Burden <eric@ericburden.dev>")
        .arg(Arg::with_name("BASE")
                .help("The number whose bits get shifted.")
                .required(true)
                .index(1))
        .arg(Arg::with_name("left")
                .short("l")
                .long("left")
                .value_name("LEFT")
                .help("Number of bits to shift to the left")
                .takes_value(true))
        .arg(Arg::with_name("right")
                .short("r")
                .long("right")
                .value_name("RIGHT")
                .help("Number of bits to shift to the right")
                .takes_value(true))
        .get_matches();

    let base_input = matches.value_of("BASE")
        .ok_or(Error::new(ErrorKind::Other, "Must provide an number to bit shift"))?;
    let base = base_input.parse::<u64>()
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
    let binary_repr = to_binary_string(base);
    writeln!(io::stdout(), "\nBinary representation of {}:", base)?;
    writeln!(io::stdout(), "{}", binary_repr)?;

    let left_shift = matches.value_of("left")
        .map(
            |o| o.parse::<usize>()
                 .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
        )
        .transpose()?;
    if let Some(m) = left_shift {
        let left_repr = shift_left(&binary_repr, m);
        writeln!(io::stdout(), "\nLeft shifting by {} yields:", m)?;
        writeln!(io::stdout(), "{}", left_repr)?;
        writeln!(
            io::stdout(), 
            "which is {} in decimal form.", 
            from_binary_string(left_repr)
        )?;
    }

    let right_shift = matches.value_of("right")
        .map(
            |o| o.parse::<usize>()
                 .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
        )
        .transpose()?;
    if let Some(m) = right_shift {
        let right_repr = shift_right(&binary_repr, m);
        writeln!(io::stdout(), "\nRight shifting by {} yields:", m)?;
        writeln!(io::stdout(), "{}", right_repr)?;
        writeln!(
            io::stdout(), 
            "which is {} in decimal form.", 
            from_binary_string(right_repr)
        )?;
    }
        
    Ok(())
}
