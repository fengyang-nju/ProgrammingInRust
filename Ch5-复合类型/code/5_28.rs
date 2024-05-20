		use std::collections::HashSet;
	
	let mut books = HashSet::new();
	books.insert("The Odyssey".to_string());
	books.insert("The Iliad".to_string());
	
	for book in &books {
	    println!("{}", book);
	}
