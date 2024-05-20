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
    	
    	use geometry::init as geometry_init;
    	use tools::init as tools_init;
    	
    	fn main() {
    	    geometry_init(); // 调用geometry模块中的init函数
    	    tools_init();    // 调用tools模块中的init函数
    	}
    