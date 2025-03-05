struct Book{
    title: String,
    No_of_pages: i32,
}

fn main(){
    let  mut book=Book{
        title: String::from ("Rust Prgorammming"),
        No_of_pages:386

    };
    println!("Book Title: {}",book.title);

    fn up_date_book_title(book:  &mut Book){
        book.title=String::from("Rust Programming1");
    }
     up_date_book_title(&mut book);
    println!("{}", book.No_of_pages);
    println!("Book Title: {}",book.title);



}