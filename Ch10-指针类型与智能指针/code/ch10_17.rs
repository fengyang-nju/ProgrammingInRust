struct A{}
impl A{
    fn test(&self, num: &i32){
        println!("num = {}", *num);
    }
}

fn test(num: &&i32){
    println!("num = {}", **num);
}
fn main(){
    let a = A{};
    let num = 1;
    a.test(&num); // 正确写法，手动解引用
    // 以下是错误写法，不支持普通参数的自动解引用
    // a.test(num);
    test(&num);
}
