use std::io;
use std::io::buffered::BufferedReader;
use std::rand;
use std::rand::Rng;
use std::from_str::from_str;
 
fn main() {
    let mut reader = BufferedReader::new(io::stdin());
    // read line and parse to int NOW WITH 95% MORE SAFETY!
    let mut cases:int = match reader.read_line() {
        // parse to int or fail. trim_right to remove the extra '\n'
        Some(string) => match from_str(string.trim_right()) {
            // return the parsed number
            Some(i) => i,
            // if there was no number on the line
            None => 0
        },
        // if the line was empty
        None() => 0
    };
    println!("Cases {}", cases);
    while cases > 0 {
        let line = match reader.read_line() { Some(string) => string, None => ~""};
        let tokens = line.trim_right().split(' ').to_owned_vec();
        let token1 = match from_str(tokens[0]) {
            Some(i) => i,
            None => 0
        };
        let token2 = match from_str(tokens[1]) {
            Some(i) => i,
            None => 0
        };
        println!("{} {}",token1,token2);
        cases -= 1;
    }
    println!("{}","Done");
}
 
fn isPrime(n: int) -> bool {
    let k = 100;
    let mut i = 0;
 
    while i < k {
        let randomNumber = randLessThan(n);
 
        if modexp(randomNumber, n-1, n) != 1 {
            return false;
        }
        i += 1;
    }
    return true;
}
 
fn randLessThan(x: int) -> int {
    let mut rng = rand::rng();
    let mut r = x;
    while r >= x {
        r = rng.gen::<int>();
    }
    return r;
}
 
fn modexp(x: int, y: int, n: int) -> int {
    let mut z: int;
    if y==0 {
        return 1;
    }
 
    z = modexp(x, y/2, n);
 
    if y%2==0 {
        return (z*z) % n;
    } else {
        return (x * (z * z)) % n;
    }
}