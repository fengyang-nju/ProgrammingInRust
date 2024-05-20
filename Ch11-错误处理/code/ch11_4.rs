enum Result<T, E> {
    //操作成功，关联结果类型为T的结果
    Ok(T),
    //操作失败，记录错误类型为E的错误
    Err(E), 
}