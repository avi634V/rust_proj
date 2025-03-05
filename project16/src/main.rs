fn main() {

    let temp_cel=37;
    let temp_far=convert_cel_to_far(temp_cel);
    println!("Temperature in Fahrenheit: {}",temp_far);

}
fn convert_cel_to_far(temp_cel:i32)->i32 {
    let temp_far=(temp_cel*9/5)+32;
   temp_far
    
}
