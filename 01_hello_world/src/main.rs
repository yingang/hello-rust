use std::fmt;

fn chapter1() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");
}

fn chapter1_1() {
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);

    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
    println!("{} of {:b} people know binary, the other half don't", 1, 2);  // 二进制输出
    println!("{number:>width$}", number = 1, width = 6);    // 指定宽度
    println!("{number:>0width$}", number = 1, width = 6);   // 前补零
    println!("My name is {0}, {1} {0}", "Bond", "James");
}

fn chapter1_2() {
    #[allow(dead_code)]
    #[derive(Debug)]    // 有这个就可以用 {:?} 了
    struct Structure(i32);

    println!("This struct `{:?}` won't print...fixed", Structure(3));

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {  // 这个方法的签名怎么理解？
            // f 看上去是一个缓冲区
            write!(f, "Structure({})", self.0) // 结尾没有 `;` 表示直接返回 write! 的返回值吧
        }
    }

    println!("This struct `{}` won't print...fixed", Structure(3));

    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);
}

fn chapter1_2_1() {
    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    println!("Now {:?} will print!", Structure(3));

    println!("Now {:?} will print!", Deep(Structure(7)));

    #[derive(Debug)]
    struct Person<'a> {     // 这个 'a 是表示生命周期吗？
        name: &'a str,      // 这个 & 呢？
        age: u8,
    }

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // 美化打印
    println!("{:#?}", peter);
}

fn chapter1_2_2() {
    #[derive(Debug)]
    struct MinMax(i64, i64);

    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64,
    }

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }

    let complex = Complex { real: 3.3, imag: 7.2 };

    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}

fn chapter1_2_2_1() {
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;    // 有 ? 表示活没干完，后续继续
            for (count, v) in vec.iter().enumerate() {
                if count != 0 { write!(f, ", ")?; }
                write!(f, "{}: {}", count, v)?;
            }
            write!(f, "]")      // 无 ? 表示活干完了，同前，不需要用 `;` 结尾
        }
    }

    let v = List(vec![1, 2, 3]);    // 这个 vec! 又是什么？
    println!("{}", v);
}

fn chapter1_3() {
    use std::fmt::{self, Formatter, Display};

    struct City {
        name: &'static str,
        lat: f32,
        lon: f32,
    }

    impl Display for City {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
            write!(f, "{}: {:.3}°{} {:.3}°{}",
                self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
        }
    }

    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    impl Display for Color {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "RGB ({}, {}, {}) 0x{:02X}{:02X}{:02X}",
                self.red, self.green, self.blue,
                self.red, self.green, self.blue)
        }
    }

    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{}", *color)
    }
}

fn main() {
    chapter1();
    chapter1_1();
    chapter1_2();
    chapter1_2_1();
    chapter1_2_2();
    chapter1_2_2_1();
    chapter1_3();
}
