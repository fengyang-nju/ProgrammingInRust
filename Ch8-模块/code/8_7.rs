	mod outer { // 默认私有
    	    pub mod inner { // 公有，因为前面有pub
    	        pub fn display_message() { // 公有，可从外部访问
    	            println!("Hello from the inner module!");
    	        }
    	
    	        fn secret_function() { // 私有，仅在inner模块内部可用
    	            println!("This is a secret function.");
    	        }
    	    }
    	
    	    pub fn call_secret_function() {
    	        inner::secret_function(); // 错误！secret_function在outer模块中不可访问
    	    }
    	}
    	
    	fn main() {
    	    outer::inner::display_message(); // 正确，因为inner模块和display_message函数都是公有的
    	    // outer::inner::secret_function(); // 错误，因为secret_function函数是私有的
    	    // outer::call_secret_function(); // 错误，因为call_secret_function尝试访问私有函数
    	}
    