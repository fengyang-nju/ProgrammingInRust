		mod book {
    	    pub struct Book {
    	        pub title: String,
    	        pub author: String,
    	    }
    	
    	    impl Book {
    	        pub fn new(title: String, author: String) -> Book {
    	            Book { title, author }
    	        }
    	    }
    	
    	    pub fn display_book_info(book: &Book) {
    	        println!("Book: {} by {}", book.title, book.author);
    	    }
    	}
    