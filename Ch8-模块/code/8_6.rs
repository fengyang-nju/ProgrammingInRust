		mod my_module {
    	    pub struct MyStruct {
    	        pub public_field: i
    	        private_field: i
    	    }
    	    
    	    pub enum MyEnum {
    	        PublicVariant,
    	        PrivateVariant,
    	    }
    	}
    	
    	// 使用
    	fn main() {
    	    let my_struct = my_module::MyStruct { public_field:  private_field: }; // 错误！private_field是私有的。
    	    let my_enum = my_module::MyEnum::PrivateVariant; // 错误！PrivateVariant是私有的。
    	}
    