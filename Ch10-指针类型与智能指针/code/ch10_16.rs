struct E(i32);
trait A{
    fn print(&self);
}
trait B{
    fn print(&self);
}
impl A for E{
    fn print(&self){
        println!("A's print: {}", self.0);
    }
}
impl B for E{
    fn print(&self){
        println!("B's print: {}", self.0);
    }
}
impl E{
    fn print(&self){
        println!("E's print: {}", self.0);
    }
}
fn main(){
    let e = E(1);
    e.print(); // 默认调用E自己实现的print方法
    A::print(&e); // 显式调用A的print
    B::print(&e); // 显式调用B的print
    <E as A>::print(&e); 
    <E as B>::print(&e);
}