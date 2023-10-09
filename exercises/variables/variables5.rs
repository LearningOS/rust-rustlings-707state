// variables5.rs
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a hint.

fn main() {
    let number:&str = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    // number= 3; // don't rename this variable
    let number =match number{
        "T-H-R-E-E"=>3,
        _=>panic!("invalid number"),
    };
    println!("Number plus two is : {}", number + 2);
}
