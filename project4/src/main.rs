

fn main(){

 
    let mut x =String::from("Hello Avijit");
    update_string(&mut x);

}
fn update_string(s: &mut String){
    s.push_str(" How are you?");
    println!("{}",s);
}

