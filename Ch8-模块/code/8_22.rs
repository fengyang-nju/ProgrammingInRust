		mod draw {
    	    pub fn line() {
    	        println!("Drawing a line.");
    	    }
    	    // 更多与绘制相关的函数...
    	}
    	
    	pub mod graphics {
    	    // 使用pub use在graphics模块内重导出draw模块中的line函数
    	    pub use crate::draw::line as draw_line;
    	}
    