trait Authors {
    fn get_authors(&self) -> Vec<String>;
}

struct Paper {
    title: String,
    authors: Vec<String>,
    doc_id: String,
}

impl Authors for Paper {
    fn get_authors(&self) -> Vec<String> {
        return self.authors.clone();
    }
}

struct Book {
    name: String,
    authors: Vec<String>,
    publisher: String,
}

impl Authors for Book {
    fn get_authors(&self) -> Vec<String> {
        return self.authors.clone();
    }
}

// fn get_authors(x: i32) -> impl Authors {
//     let paper = Paper {
//         title: String::from("Paper Title"),
//         authors: vec![String::from("a")],
//         doc_id: String::from("1")
//     };

//     let book = Book {
//         name: String::from("ProgrammingInRust"),
//         authors: vec![String::from("b")],
//         publisher: String::from("c")
//     };

//     if x == 1 {
//         return paper;
//     }
//     else {
//         return book;
//     };
// }

fn get_boxed_authors(x: i32) -> Box<dyn Authors> {
    let paper = Paper {
        title: String::from("Paper Title"),
        authors: vec![String::from("a")],
        doc_id: String::from("1")
    };

    let book = Book {
        name: String::from("ProgrammingInRust"),
        authors: vec![String::from("b")],
        publisher: String::from("c")
    };

    if x == 1 {
        return Box::new(paper);
    }
    else {
        return Box::new(book);
    };
}
fn main() {
    println!("{:?}", get_boxed_authors(1).get_authors());
    println!("{:?}", get_boxed_authors(2).get_authors());
}