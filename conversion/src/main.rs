fn chapter6_1() {
    let my_str = "hello";
    let _my_string = String::from(my_str);

    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }

    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    // 试试删除类型说明
    let num: Number = int.into();
    println!("My number is {:?}", num);
}

fn chapter6_2() {
    struct Circle {
        radius: i32
    }

    impl ToString for Circle {
        fn to_string(&self) -> String {
            format!("Circle of radius {:?}", self.radius)
        }
    }

    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    // 需要提供转换的目标类型，两种方式都可以
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!{"Sum: {:?}", sum};
}

fn main() {
    chapter6_1();
    chapter6_2();
}
