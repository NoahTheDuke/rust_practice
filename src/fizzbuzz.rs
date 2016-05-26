fn main() {
    for x in 1..101 {
        match x % 15 {
            0               => println!("fizzbuzz"),
            i if i % 3 == 0 => println!("fizz"),
            i if i % 5 == 0 => println!("buzz"),
            _               => println!("{}", x),
        }
    }
}
