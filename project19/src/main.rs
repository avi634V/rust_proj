// struct Person {
//     name: String,
//     age: u8,
// }

// fn main(){

// let mut user = Person {
//     name: String::from("Alice"),
//     age: 30,
// };
// user.age = 31;
// println!("User new age is :{}",user.age);
// }

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
 fn main(){
     let user1 = build_user(String::from("      "),String::from("       "));
        println!("User1 email is :{}",user1.email); 
 }
 fn build_user(email:String,username: String) -> User{
        
    User{
        active:true,
        username:String:: from ("Avijit"),
        email:String:: from ("avijit@gmail.com"),
        sign_in_count:1,
    }
 }