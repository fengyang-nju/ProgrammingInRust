	fn main() {
	    let email = "username@example.com";
	
	    if email.contains('@') {
	        println!("Valid email");
	    } else {
	        println!("Invalid email");
	    }
	
	    match email.split('@').next() {
	        Some(user) => println!("Username is: {}", user),
	        None => println!("No username found"),
	    }
	}
    