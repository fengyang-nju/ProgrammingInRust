mod network {
    	    pub fn connect() {
    	        println!("Network connected");
    	    }
    	}
    	
    	fn main() {
    	    network::connect(); // 正确，因为connect函数是公开的。
    	}
    