	mod geometry {
    	    pub fn init() {
    	        println!("Initializing geometry module...");
    	    }
    	}
    	
    	mod tools {
    	    pub fn init() {
    	        println!("Loading tools module...");
    	    }
    	}
    	
    	use geometry::*;
    	use tools::*; // 这会导致不明确的来源错误
    	
    	fn main() {
    	    init(); // 错误：不明确的`init`——它来自`geometry`还是`tools`？
    	}
    