fn main(){

    let  mut s = String::from("wELCOME TO THE nibm fINTECH EVENT");
    let final_string= upper_case( s);
    println!("{}",final_string);
}

    fn upper_case(mut s:String) -> String{

    s= s.to_uppercase();
    println!("{}",s);
    s.split_off(4);

    return String::from(s);

}

