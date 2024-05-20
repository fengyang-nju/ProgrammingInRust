	mod arithmetic {
	    pub mod basic {
	        pub fn add(a: i32, b: i32) -> i32 {
	            a + b
	        }
	    }
	}
	
	mod geometry {
	    pub mod shapes {
	        pub fn area_of_circle(radius: f64) -> f64 {
	            3.14159 * radius * radius
	        }
	    }
	}
	
	mod statistics {
	    pub mod analysis {
	        pub fn mean(data: &[f64]) -> f64 {
	            let sum: f64 = data.iter().sum();
	            sum / data.len() as f64
	        }
	    }
	}
    