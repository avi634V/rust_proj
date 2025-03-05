fn main(){
    for x in 0..10{
        if x<6{
            println!("The number is {} which is less than 6",x );
        }else if x==6{
            println!("The number is {} which is equal to 6",x);
            
        }{
            println!("The number is {} which is greatr than 6",x);
        }
    }
    println!("The loop is done");

}