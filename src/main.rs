
#[derive(Debug)]
struct Book{
    title:String,
    author:String,
    year_published:u32,
    is_checked_out: bool
}


impl Book{
    fn new(title:String, author:String, year_published:u32) -> Book{
       Book{
        title,
        author,
        year_published,
        is_checked_out : false
       }
    }

    fn check_out(&mut self) -> bool{
        self.is_checked_out = true;
        return self.is_checked_out
    }

    fn check_in(&mut self) -> bool{
        self.is_checked_out = false;
        return self.is_checked_out
     }
}

fn main() {
   let mut book1  = Book::new("Animal Farm".to_string(),"George Orswell".to_string(),1945);
   let mut book2  = Book::new("Naruto".to_string(),"Masashi Kishimoto".to_string(),1999);
   let mut book3  = Book::new("Vagabond".to_string(),"Takehiko Inoue".to_string(),1998);
   let mut book4  = Book::new("Animal Farm".to_string(),"George Orswell".to_string(),1945);
   let mut book5  = Book::new("Animal Farm".to_string(),"George Orswell".to_string(),1945);
   

   println!();
   println!("Title: {:?}", book1.title);
   println!("Author: {:?}", book1.author);
   println!("Year Published: {:?}", book1.year_published);
   let is_out = book1.is_checked_out;
   if is_out{
    println!("Is it checked out: Yes");
    println!()
   }
   else{
    println!("Is it checked out: No");
    println!()
   }

   book1.check_out();

   println!("Do you guys have Animal Farm?");

   if is_out {
    println!("Is it checked out?\n\nThe book is not checked out")
   }
   else{
    println!("Is it checked out?\n\nThe book is checked out")
   }
  
   
}
