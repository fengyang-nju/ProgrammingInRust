
// 结构体

//demo_6-1
struct Person{
    name: String,
    gender: String,
    age: i32,
    address:String,
}


fn fun1() {

    //demo_6-2
    let person1 = Person{
        name:"Mike".to_string(),
        gender:"man".to_string(),
        age:20,
        address:"Washington, USA".to_string()
    };

    //demo_6-3
    let gender = "woman".to_string();
    let address="Washington, USA".to_string();
    let person2 = Person{
        name:"Lisa".to_string(),
        age:19,
        gender,
        address
    };

    //demo_6-4
    let mut person3 = Person{
        name:"John".to_string(),
        gender:"man".to_string(),
        age:22,
        address:"Washington, USA".to_string()
    };
    person3.age=25;


    //demo_6-5
    let person4 = Person{
        name:"Jack".to_string(),
        age:19,
        gender:person1.gender,
        address:person1.address
    };

    //demo_6-6
    let person5 = Person{
        name:"Allison".to_string(),
        age:22,
        ..person2
    };

    //demo_6-7
    let Person{name:iname,gender:igender,age:iage,address:iaddress}=person3;
    println!("person3: name:{},gender:{},age:{},address:{}",iname,igender,iage,iaddress);
    let Person{name,gender,age,address}=person4;
    println!("person4: name:{},gender:{},age:{},address:{}",name,gender,age,address);
}



//demo_6-8
struct Point(i32, i32); 
struct Rectangle(i32, i32);
struct Cicle(Point, i32, String); 

fn fun2(){
    let point1 = Point(3, 4); 
    let rectangle = Rectangle(3, 4);
    let cicle1 = Cicle(point1, 5, "yellow".to_string());
    println!("the area of cicle1 is : {}", 3.14 * cicle1.1.pow(2) as f32);
}

//demo_6-9
#[derive(Debug)]
struct Year(i32);

fn fun3(){
    let year = Year(2023);
    let year_i32 = 2023;
}

//demo_6-10
struct Boy();
struct Girl{}


fn fun4(){
    let boy = Boy();
    let girl = Girl{};

    //demo_6-11
    let year = Year(2023);
    println!("year is : {:?}", year);
}


//demo_6-12
struct Book{
    name:String,
    price:i32,
}

impl Book{
    fn getPrice(&self) -> i32{
        self.price
    }

    fn setPrice(&mut self, newPrice:i32){
        self.price = newPrice
    }
}

fn fun5(){
    let mut book = Book{name:"The Red and the Black".to_string(), price:10};
    book.setPrice(12);
    println!("the price of 'The Red and the Black' is ${}", book.getPrice());
}


//demo_6-13
impl Book{
    fn new(name:String, price:i32) -> Book{
        Book{name, price}
    }
}

fn fun6(){
    let book = Book::new("Harry Potter".to_string(), 15);
}


fn main(){
    fun5()
}