fn main() {
    let s1 = "ProgrammingInRust".to_string();
    let s2 = "RustProgrammingLanguage".to_string();

    let bigger_string_move = move || {
        s1.max(s2)
    };
    assert_eq!(
        bigger_string_move(), 
        "RustProgrammingLanguage".to_string()
    );
}