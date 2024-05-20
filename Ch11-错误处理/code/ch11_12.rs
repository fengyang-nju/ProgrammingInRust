use std::thread;

fn trigger_error(thread_name:&str){
    panic!("panic in thread {}", thread_name);
}

fn spawn_child() -> thread::JoinHandle<()>{
    thread::spawn(move ||{
        println!("child thread does something");
        trigger_error("child thread");
        println!("this code should not be executed!")
    })
}

fn main(){
    println!("spawn child thread");
    let child = spawn_child();
    let _ = child.join();
    println!("main thread does something");
    trigger_error("main");
}

