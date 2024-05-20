use std::rc::Rc;
fn main(){
    let a = Rc::new(String::from("hello, world")); // 创建一个Rc<String>
    let b = a.clone(); // b为Rc<String>
    let c = (*a).clone(); // c为String
}