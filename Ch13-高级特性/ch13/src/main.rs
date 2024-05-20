/*
pub fn get_password_of_url(param0: &str) -> Option<String> {
    let local0 = url::Url::parse(param0).unwrap();
    url::Url::password(&(local0)).map(|x| x.to_string())
}

pub fn print_password(pwd: Option<String>) {
    let password = match pwd {
        Some(pass) => pass,
        None => "No Password".to_owned(),
    };
    println!("{}", password);
}

fn main() {
    let url = "https://www.abc.com";
    let pwd = get_password_of_url(url);
    print_password(pwd);
}
*/
/*
struct MyStack {
    vec: [i32; 10],
    len: usize,
}

impl MyStack {
    fn new() -> Self {
        MyStack {
            vec: [0; 10],
            len: 0,
        }
    }

    fn get_len(&self) -> usize {
        self.len
    }

    fn push(&mut self, item: i32) -> Result<(), ()> {
        if self.len >= 10 {
            return Err(());
        }
        self.vec[self.len] = item;
        self.len += 1;
        Ok(())
    }
    fn pop_when_i_small(&mut self, i: i32) -> Result<i32, ()> {
        if i > -1000 {
            return Err(());
        }
        let res = self.vec[self.len];
        self.len -= 1;
        Ok(res)
    }
}

fn main() {
    let mut stack = MyStack::new();
    let _ = stack.push(1);
    let _ = stack.pop_when_i_small(-10000);
    let _ = stack.pop_when_i_small(-10000);

    let mut stack = MyStack::new();
    let _ = stack.get_len()
    let _ = stack.get_len();
}
*/
/*
fn main() {
    let mut x = 1;
    let mut p = &mut x as *mut i32;

    p = 0 as *mut i32;

    unsafe {
        println!("{}", *p);
    }
}
*/
/* *
extern crate just;

fn lex_and_parse<'a>(input: &'a str) -> Result<(), just::CompilationError<'a>> {
    let tokens = just::Lexer::lex(input)?;
    let parser = just::Parser::new(input, tokens);
    let _justfile = parser.justfile()?;
    Ok(())
}

fn main() {
    let input = "export	AA";
    lex_and_parse(&input);
}
*/

/*
fn main() {
    let mut a = 1;
    let x = &mut a;
    f(x);
    println!("{:?}", x);
}
*/
/*
use std::process::exit;

fn main() {
    let mut _local0 = url::Url::parse("1111");
    let mut local = if let Ok(x) = _local0 {
        x
    } else {
        use std::process;
        process::exit(0);
    };
    //let mut _local1 = _local0.unwrap();
    //let mut _local2 = _local0.unwrap();
    //let _ = url::Url::set_scheme(&mut (_local0), "111");
    //let _ = url::Url::set_scheme(&mut (_local0), "111");
}
*/

extern crate ratatui;
fn _to_u8(data: &[u8], index: usize) -> u8 {
    data[index]
}

fn _to_u16(data: &[u8], index: usize) -> u16 {
    let data0 = _to_u8(data, index) as u16;
    let data1 = _to_u8(data, index + 1) as u16;
    data0 << 8 | data1
}

use ratatui::backend::Backend;

fn test_function28(
    _param0: u16,
    _param1: u16,
    _param2: u16,
    _param3: u16,
    _param4: u16,
    _param5: u16,
    _param6: u16,
    _param7: u16,
    _param8: u16,
    _param9: u16,
    _param10: u16,
    _param11: u16,
    _param12: u16,
    _param13: u16,
    _param14: u16,
    _param15: u16,
    _param16: u16,
    _param17: u16,
    _param18: u16,
    _param19: u16,
    _param20: u16,
    _param21: u16,
) {
    println!(
            "_param1 = {}, _param2 = {}, _param3 = {}, _param4 = {}, _param10 = {}, _param11 = {}, _param12 = {}, _param13 = {}",
            _param1, _param2, _param3, _param4, _param10, _param11, _param12, _param13
        );
    /*
    let _ = ratatui::widgets::Padding::uniform(_param0);
    let _ = ratatui::widgets::Padding::zero();*/
    let _local2 = ratatui::layout::Rect::new(_param1, _param2, _param3, _param4);
    /*let _ = ratatui::widgets::Padding::uniform(_param5);
    let _ = ratatui::widgets::Padding::horizontal(_param6);
    let _ = ratatui::widgets::Padding::horizontal(_param7);
    let _ = ratatui::style::Style::new();
    let _ = ratatui::widgets::Padding::horizontal(_param8);
    let _ = ratatui::widgets::Padding::vertical(_param9);*/
    let _local9 = ratatui::layout::Rect::new(_param10, _param11, _param12, _param13);
    let _local10 = ratatui::layout::Rect::union(_local2, _local9);
    /*let mut _local11 = ratatui::backend::TestBackend::new(_param14, _param15);
    let _ = ratatui::layout::Rect::left(_local10);
    let _ = ratatui::style::Style::reset();
    let _ = ratatui::backend::Backend::flush(&mut (_local11));
    let _ = ratatui::backend::Backend::hide_cursor(&mut (_local11));
    let _ = ratatui::backend::Backend::get_cursor(&mut (_local11));
    let _ = ratatui::backend::Backend::size(&(_local11));
    let _ = ratatui::backend::Backend::show_cursor(&mut (_local11));
    let _ = ratatui::backend::Backend::hide_cursor(&mut (_local11));
    let _local20 = ratatui::backend::TestBackend::buffer(&(_local11));
    let _ = ratatui::buffer::Buffer::index_of(_local20, _param16, _param17);
    let _ = ratatui::backend::Backend::size(&(_local11));
    let _ = ratatui::backend::Backend::hide_cursor(&mut (_local11));
    let _ = ratatui::widgets::Padding::new(_param18, _param19, _param20, _param21);*/
}

fn _read_data() -> Vec<u8> {
    use std::env;
    use std::process::exit;
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No crash filename provided");
        exit(-1);
    }
    use std::path::PathBuf;
    let crash_file_name = &args[1];
    let crash_path = PathBuf::from(crash_file_name);
    if !crash_path.is_file() {
        println!("Not a valid crash file");
        exit(-1);
    }
    use std::fs;
    let data = fs::read(crash_path).unwrap();
    data
}
/*
fn main() {
    let _content = _read_data();
    let data = &_content;
    println!("data = {:?}", data);
    println!("data len = {:?}", data.len());
    //actual body emit
    if data.len() != 44 {
        return;
    }
    let _param0 = _to_u16(data, 0);
    let _param1 = _to_u16(data, 2);
    let _param2 = _to_u16(data, 4);
    let _param3 = _to_u16(data, 6);
    let _param4 = _to_u16(data, 8);
    let _param5 = _to_u16(data, 10);
    let _param6 = _to_u16(data, 12);
    let _param7 = _to_u16(data, 14);
    let _param8 = _to_u16(data, 16);
    let _param9 = _to_u16(data, 18);
    let _param10 = _to_u16(data, 20);
    let _param11 = _to_u16(data, 22);
    let _param12 = _to_u16(data, 24);
    let _param13 = _to_u16(data, 26);
    let _param14 = _to_u16(data, 28);
    let _param15 = _to_u16(data, 30);
    let _param16 = _to_u16(data, 32);
    let _param17 = _to_u16(data, 34);
    let _param18 = _to_u16(data, 36);
    let _param19 = _to_u16(data, 38);
    let _param20 = _to_u16(data, 40);
    let _param21 = _to_u16(data, 42);
    test_function28(
        _param0, _param1, _param2, _param3, _param4, _param5, _param6, _param7, _param8, _param9,
        _param10, _param11, _param12, _param13, _param14, _param15, _param16, _param17, _param18,
        _param19, _param20, _param21,
    );
}*/

fn main() {
    let _local2 = ratatui::layout::Rect::new(13876, 13622, 14080, 128);

    let _local9 = ratatui::layout::Rect::new(65535, 11858, 4096, 0);
    let _local10 = ratatui::layout::Rect::union(_local2, _local9);
}
