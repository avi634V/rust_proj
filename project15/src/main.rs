// fn main(){
//     loop{
//         println!("again")
//     }
// }

// fn main(){

//     let mut counter = 0;
//     let result = loop{
//         counter +=1;

//         if counter == 10{
//             break counter*2;

//         }
//     };
//     println!("The result is {}",result);
// }
//While loop
// fn main(){
//     let mut number = 10;
//     while number !=0{
//         println!("{}!",number);
//         number -=1;
//     }
//     println!("LIFTOFF!!!");
// }
//For loop

// fn main(){
//     let a =[10,20,30,40,50,60];
//     let mut index =0;

//     while index <6{
//         println!("The value of index is {}",a[index]);
//         index +=1;
//     }
// }

fn main(){
    let a =[10,20,30,40,50,60];
    for element in a{
        println!("The value of element is {}",element);
    }
}
