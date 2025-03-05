fn main() {
    print_labelled_measurement(5,'h');
}
fn print_labelled_measurement(x :i32, unit_label :char) {
    println!("{}{}", x, unit_label);
}
