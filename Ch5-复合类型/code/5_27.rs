	use std::collections::HashSet;
	
	let mut books = HashSet::new();
	books.insert("The Odyssey".to_string());
	books.insert("The Iliad".to_string());
	
	if books.contains("The Odyssey") {
	    println!("We have The Odyssey");
	}
