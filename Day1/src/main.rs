use std::fs;
use std::io;
use std::io::Read;
use std::io::prelude::*;
use std::io::BufReader;
fn main() -> io::Result<()> {
let mut file = fs::File::open("input.txt")?;
let mut buf = BufReader::new(file);
let mut vec: Vec<(i32, i32)> = Vec::new();
let mut total_value: i32 = 0;
let mut last_counter: i32 = 0;
for line in buf.lines() {
    let mut safe_line = String::new();
   match line {
    Ok(line) => safe_line = line,
    Err(e) => panic!("{e}")
   }
   if safe_line != "" {
    let converted_int: i32 = safe_line.parse().unwrap();
    total_value = total_value + converted_int;
   }
   else {
    last_counter += 1;
    vec.push((last_counter, total_value));
    total_value = 0;
   }
}
let mut largest_tuple: (i32, i32) = (0, 0);
for tup in vec.clone() {
    if tup.1 > largest_tuple.1 { 
        largest_tuple = tup;
    }

}
println!("Elf number {} has the most calories with {}!", largest_tuple.0, largest_tuple.1);
let mut second_largest: (i32, i32) = (0, 0);
for tup in vec.clone() {
    if tup.1 > second_largest.1 && tup.1 != largest_tuple.1 { 
        second_largest = tup;
    }

}
println!("Elf number {} has the second most calories with {}!", second_largest.0, second_largest.1);
let mut third_largest: (i32, i32) = (0, 0);
for tup in vec.clone() {
    if tup.1 > third_largest.1 && tup.1 != largest_tuple.1 && tup.1 != second_largest.1 {
        third_largest = tup
    }
}
println!("Elf number {} has the third most calories with {}!", third_largest.0, third_largest.1);
println!("The top three elves have {} calories total!", largest_tuple.1 + second_largest.1 + third_largest.1);
Ok(())
}