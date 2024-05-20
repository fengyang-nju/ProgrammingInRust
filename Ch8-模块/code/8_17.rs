	mod a_very_long_module_name {
    	    pub mod another_nested_module {
    	        pub fn a_very_long_function_name() {
    	            println!("Function called");
    	        }
    	    }
    	}
    	
    	// 使用`as`关键字简化引用
    	use a_very_long_module_name::another_nested_module::a_very_long_function_name as short_fn;
    	
    	fn main() {
    	    short_fn(); // 简化后的函数调用
    	}
    