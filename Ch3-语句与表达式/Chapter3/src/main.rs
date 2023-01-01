fn main() {
    println!("Hello, world!");
}

//语言与表达式

fn main() {
    {
        //demo_3-1
        let v1 = 2 + 3;
        let v2 = 4 - 2;
        let v3 = 3 * v1;
        let v4 = v2 / 2;
        let v5 = v4 % v3;
        let v6 = (v5 + 1) * 2;
    }

    {
        //demo_3-2
        let b1 = 3 > 2;
        let b2 = 1.5 < 3.0;
        let b3 = 5.0 >= 4.999;
        let b4 = 4.0 <= 4.00;
        let b5 = "123" != "1 2 3";
        let b6 = b5 == true;
        // let b7 = 3 > 2 > 1; error: comparison operators cannot be chained
    }

    {
        //demo_3-3
        let v1 = 10;
        let v2 = "hello".to_string();
        let b1 = v1 > 15 && v2.len() >= 3;              // false
        let b2 = v1 < 13 || v2.len() <= 4;              // true
        let b3 = !(v2.len()==3) && v1 > 2 && v1 < 20;   // true
        // print!("{} {} {}", b1, b2, b3);
    }

    {
        //demo_3-4
        let v1: u8 = 0b_0000_1111;
        let v2: u8 = 0b_1010_1010;
        let v3: u8 = ! v1;     // 11110000
        let v4: u8 = v1 & v2;  // 00001010
        let v5: u8 = v1 | v2;  // 10101111
        let v6: u8 = v1 ^ v2;  // 10100101
        let v7: u8 = v1 << 2;  // 00111100
        let v8: u8 = v2 >> 2;  // 00101010
        // println!("{:08b} {:08b} {:08b} {:08b} {:08b} {:08b}", v3,v4,v5,v6,v7,v8);
    }

    {
        //demo_3-5
        let b1 = true;
        let b2 = false;
        let b3 = b1 & b2;   // false
        let b4 = b1 | b2;   // true
        let b5 = b1 && b2;  // false
        let b6 = b1 || b2;  // true
        // println!("{} {} {} {}", b3, b4, b5, b6);
    }


    {
        //demo_3-6
        let mut v = 3;
        v = 4;
    }

    {
        //demo_3-7
        let mut x = 2;
        let y = x = 3;  // y: ()
        // let z:i32 = x = 4;  error: mismatched types     expected `i32`, found `()`
        // println!("{:?}", y);
    }

    {
        //demo_3-8
        let mut x = 10;
        x += 1;  // x = x + 1
        x -= 2;  // x = x - 2
        x *= 2;  // x = x * 2
        x /= 3;  // x = x / 3
    }

    {
        //demo_3-9
        let x: () = {println!("hello world!");};
        let y: i32 = {println!("hello world!"); 5};
    }

    {
        //demo_3-10
        let x = 4;
        if x%2==0 {
            println!("{} is an even number", x);
        }
    }

        {
        //demo_3-10
        let x = 4;
        if x%2==0 {
            println!("{} is an even number", x);
        }
    }

    {
        //demo_3-11
        let x = 4;
        if x%2==0 {
            println!("{} is an even number", x);
        }
        else {
            println!("{} is an odd number", x);
        }
    }


    {
        //demo_3-12
        let x = 4;
        if x<0 {
            println!("{} is a negative  number", x);
        }
        else if x>0 {
            println!("{} is a positive number", x);
        }
        else {
            println!("{} is zero", x);
        }
    }

    {
        //demo_3-13
        let x = 5;
        let y = if x>0 {
            "positive".to_string()
        }
        else if x<0{
            "negative".to_string()
        }
        else {
            "zero".to_string()
            //0   error: mismatched types     `if` and `else` have incompatible types
        };
    }

    {
        //demo_3-14
        let mut x = 10;
        loop{
            println!("hello");
            x -= 1;
            if x==0 {
                break;
            }
            else if x%2==0{
                continue;
            }
            println!("world");
        }
    }

    {
        //demo_3-15
        let mut x = 0;
        'a: loop{
           x += 10;
           'b: loop{
                x -= 1;
                if x==0{
                    break 'a;
                }
                else if x%2==0{
                    continue 'b;
                }
                println!("x is {}", x);
           }
        }
    }

    {
        //demo_3-16
        let x = loop{
            break 2;
        };
        // let y = loop{};
        // print!("{} {:?}", x, y);  //  unreachable statement
    }

    {
        //demo_3-17
        let mut x = 10;
        while x>=0 {
            x -= 1;
        }
    }

    {
        //demo_3-18
        let x;
        loop{
            x=1;
            break;
        }
        println!("x is {}", x);

        let y;
        while true{
            y=1;
            break;
        }
        // println!("y is {}", y); error: borrow of possibly-uninitialized variable: `y`
    }

    {
        //demo_3-19
        let arr = [10, 20, 30, 40, 50];
        for elem in arr{
            println!("{}", elem);
        }
    }

}
