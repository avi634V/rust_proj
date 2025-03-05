use std::io;


fn main() {
    println!("Enter your name ");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("failed to read line");

    println!("Enter your age ");
    let mut age = String::new();
    io::stdin()
        .read_line(&mut age)
        .expect("failed to read line");

    println!("Enter your sex ");
    let mut sex: String = String::new();
    io::stdin()
        .read_line(&mut sex)
        .expect("failed to read line");

    println!("Name: {}", name.trim());
    println!("Age: {}", age.trim());
    println!("Sex: {}", sex.trim());
    println!("Hi, {} you are {} years old and you are {}", name.trim(), age.trim(), sex.trim());
}
