fn main(){
    let  mut x=1;
    for i in 0..10{
        println!("x is :{} and i is :{} ",x,i);
         x=x+i;
         println!("After ( x=x+i )x is now :{} and i is now :{}",x,i);

        println!(" {} + {} = {}",x,i,x+i);
    }
}