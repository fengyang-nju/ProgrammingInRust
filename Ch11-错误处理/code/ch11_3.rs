fn main(){
    //表示类型为int的值1
    let some_int:Option<int> = Some(1);
    //表示类型char的值 a
    let _some_char:Option<char> = Some('a');
    //表示值缺失
    let _null_char:Option<char> = None;
    //提取some_int中的值 并 +1
    let _some_int_plus_one = some_int.unwrap() + 1;

    //error:cannot add `{integer}` to `Option<{integer}>`
    let _some_int_plus_one_err = some_int + 1;
}