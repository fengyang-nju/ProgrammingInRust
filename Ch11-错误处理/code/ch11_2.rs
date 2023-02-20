pub enum Option<T> { 
    // 值缺失
    None,
    // 值不确实，关联类型为T的值
    Some(T)
}