fn b_called_by_a(){
    panic!("panic at fn b");
}

fn a_called_by_main(){
    let _a_obejct_in_heap = String::from("this str is in heap");
    let _a_object_in_stack1 = 0;
    let _a_object_in_stack2 = [1, 2, 3];
    b_called_by_a();
}
fn main(){
    a_called_by_main();
}