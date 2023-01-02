
// 模块

{
    //demo_8-1
    //创建二进制可执行程序
    cargo new hello_world --bin
    //创建library
    cargo new hello_world --lib
}


{
    //demo_8-3   src/lib.rs
    mod communication{
        fn roar(){
        }
    }
}


{
    //demo_8-4   src/lib.rs
    mod communication{
        fn roar(){
        }
    }
    
    mod action{
        fn roar(){
        }
    }
}

{
    //demo_8-5   src/lib.rs
    mod communication{
        fn roar(){
        }
    }

    mod action{
        fn roar(){
        }
        
        mod sleep{
            fn snore(){

            }
        }
    }
}

{
    //demo_8-6   src/lib.rs
    mod communication;
    mod action{
        fn roar(){
        }
        
        mod sleep{
            fn snore(){
            }
        }
    }

    //demo_8-7   src/communication.rs
    fn roar(){
    }
}

{
    //demo_8-8   src/lib.rs
    mod communication;
    mod action;

    //demo_8-9   src/action.rs
    fn roar(){
    }
    
    mod sleep{
        fn snore(){
        }
    }
}


{
    //demo_8-10  src/action.rs
    fn roar(){
    }
    
    mod sleep{
        fn snore(){
        }
    }

    mod communication{
        fn roar(){
        }
    }
}


{
    //demo_8-11  src/action/mod.rs
    fn roar(){
    }
    
    mod sleep;
    mod communication;

    //--------------------------------------------
    //src/action/sleep.rs
    fn snore(){
    }

    //--------------------------------------------
    //src/action/communication.rs
    fn roar(){
    }
}

{
    //demo_8-12   src/main.rs
    use animal::communication;

    fn main(){
        communication::roar();
    }
}


{
    //demo_8-13   src/lib.rs
    mod communication;
    pub mod action;

    
    //demo_8-14   src/action/mod.rs
    pub fn roar(){
    }

    mod sleep;
    mod communication;
}

{
    //demo_8-15   src/action/mod.rs
    pub fn roar(){
        communication::roar();
    }
    
    mod sleep;
    mod communication;

    //src/action/communication.rs
    pub fn roar(){
        super::roar();
    }
}

{
    //demo_8-16
    mod mod1 { 
        pub fn fun1(){} 
    } 
     
    mod mod2 { 
        fn fun2() { 
            crate::mod1::fun1();
        } 
    }
}

{
    //demo_8-17
    mod mod1 { 
        fn fun1(){}
    
        mod mod2{
            fn fun2() { 
                super::fun1();
                self::fun3();
            } 

            fn fun3(){}
        }

    }
}

{  
    //demo_8-18
    use std::io;
}

{
    //demo_8-19
    use std::io::{Read, Write}; 
}

{
    //demo_8-20
    use std::{io::{self, prelude}, fmt}; 
}

{
    // demo_8-21
    use std::io::*;  
}

{
    //demo_8-22
    use std::result::Result as StdResult; 
    use std::io::Result as IoResult;
}