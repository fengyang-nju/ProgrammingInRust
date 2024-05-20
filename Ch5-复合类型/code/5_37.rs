		fn main() {
	    let name = "Alice";
	    let age = 30;
	    let score = 92.5;
	
	    let formatted_string = format!(
	        "{name} is {age} years old, and her score is {score:.2}",
	        name = name,
	        age = age,
	        score = score
	    );
	
	    println!("{}", formatted_string);
	}
    