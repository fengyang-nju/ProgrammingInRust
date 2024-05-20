	mod network {
    	    pub mod server {
    	        pub fn connect() {
    	            println!("Server connected");
    	        }
    	    }
    	}
    	
    	fn main() {
    	    network::server::connect();
    	}
    