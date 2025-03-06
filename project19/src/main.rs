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

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
//  fn main(){
//      let user1 = build_user(String::from("      "),String::from("       "));
//         println!("User1 email is :{}",user1.email); 
//  }
//  fn build_user(email:String,username: String) -> User{
        
//     User{
//         active:true,
//         username:String:: from ("Avijit"),
//         email:String:: from ("avijit@gmail.com"),
//         sign_in_count:1,
//     }
//  } 
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    //println!("User1 username is :{}", user1.username); // this wont run as the value has been moveD OUT OF user1
    println!("User2 active is :{}", user1.active);} // this will run as this is implementing copy trait as its a bool type
//     println!("User1 email is :{}", user1.email);
//     println!("User2 email is :{}", user2.email); 

//     println!("User2 username is :{}", user2.username);
//     println!("User2 sign_in_count is :{}", user2.sign_in_count);
//     println!("User2 active is :{}", user2.active);
// }  

// stuct update syntax :

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     }
    
//     fn main() {
//     let user1 = User {
//     active: true,
//     username: String::from("someusername123"),
//     email: String::from("someone@example.com"),
//     sign_in_count: 1,
//     };
//     // below we are creating an struct instance user 2 from the previous struct user 1 using update syntax:
//     let user2 = User { 
//     email: String::from("another@example.com"),
//     ..user1
//      }; 
     
//     println!("User1 email is :{}", user1.email);
//     println!("User2 email is :{}", user2.email);
//     println!("User2 username is :{}", user2.username);
//     println!("User2 sign_in_count is :{}", user2.sign_in_count);
//     println!("User2 active is :{}", user2.active);
//     }