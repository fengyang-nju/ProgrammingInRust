struct X { val_x: i32 }
struct Y { val_y: X }
struct Z { val_z: Y }
impl std::ops::Deref for Z {
    type Target = Y;
    fn deref(&self) -> &Y { &self.val_z }
}

impl std::ops::Deref for Y {
    type Target = X;
    fn deref(&self) -> &X { &self.val_y }
}
impl X{
    fn print(&self){
        println!("val_x = {}", self.val_x);
    }
}
fn main(){
    let z = Z{ val_z: Y{ val_y: X{ val_x: 10 } } };
    z.print(); // 输出 val_x = 10
}