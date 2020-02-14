fn chapter8_1() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    // 所有分支必须返回相同的类型？
    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            10 * n
        } else {
            println!(", and is a big number, half the number");
            n / 2
        };

    println!("{} -> {}", n, big_n);
}

fn chapter8_2() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    loop {
        count += 1;

        if count == 3 {
            println!("three");
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");
            break;
        }
    }
}

fn chapter8_2_1() {

    #![allow(unreachable_code)]
    'outer: loop {
        println!("Entered the outer loop");

        #[allow(unused_labels)]
        'inner: loop {
            println!("Entered the inner loop");

            // 这只是中断内部的循环
            //break;

            // 这会中断外层循环
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop"); 
}

fn chapter8_2_2() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

fn chapter8_3() {
    let mut n = 1;

    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }
}

fn chapter8_4() {
    for n in 1..101 {   // 不包括101，如果用..=就包括101了
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.into_iter() {
        // into_iter会把集合中的元素消耗（move）掉！注意前后对比下match的分支类型。
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        match name {
            &mut "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}

fn chapter8_5() {
    let number = 13;
    println!("Tell me about {}", number);
    match number {
        // 匹配单个值
        1 => println!("One!"),
        // 匹配多个值
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // 匹配一个闭区间范围
        13..=19 => println!("A teen"),
        // 处理其他情况（必须得有这个才能穷尽所有可能性）
        _ => println!("Ain't special"),
    }

    let boolean = true;
    // match 也是一个表达式
    let binary = match boolean {
        // match 分支必须覆盖所有可能的值
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}

fn chapter8_5_1_1() {
    let pair = (0, -2);

    println!("Tell me about {:?}", pair);
    // match 可以解构一个元组
    match pair {
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        _      => println!("It doesn't matter what they are"),
    }
}

fn chapter8_5_1_2() {
    // 需要 `allow` 来消除警告，因为只使用了枚举类型的一种取值。
    #[allow(dead_code)]
    enum Color {
        Red,
        Blue,
        Green,
        // 这些则把 `u32` 元组赋予不同的名字，以色彩模型命名。
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    let color = Color::RGB(122, 17, 40);

    println!("What color is it?");
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!", c, m, y, k),
        // 不需要其它分支，因为所有的情形都已覆盖
    }
}

fn chapter8_5_1_3() {
    let reference = &4; // 应该和在 reference 前面用 ref 修饰是一样的吧

    match reference {
        // 加 &，则 val 就是 i32 类型
        // 如果不加 &，则 val 就是引用类型（&i32），后面的 println! 也是可以工作的
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // 解引用（dereference）
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_a_reference = 3;
    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;

    match value {
        // 这里的 r 就是 &i32 类型了
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            // 先要解引用才能使用
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
}

fn chapter8_5_1_4() {
    struct Foo { x: (u32, u32), y: u32 }

    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;     // 解构结构体的成员

    println!("a = {}, b = {},  y = {} ", a, b, y);

    // 成员顺序并不重要
    let Foo { y: i, x: j } = foo;
    println!("i = {:?}, j = {:?}", i, j);

    // 还可以忽略某些变量
    let Foo { y, .. } = foo;
    println!("y = {}", y);

    // 但不能直接就这么少了 x ！
    // let Foo { y } = foo;
}

fn chapter8_5_2() {
    let pair = (2, -2);

    println!("Tell me about {:?}", pair);
    match pair {
        // match 配 guard
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}

fn chapter8_5_3() {
    fn age() -> u32 {
        15
    }

    println!("Tell me type of person you are");
    match age() {
        0             => println!("I'm not born yet I guess"),
        // 将变量绑定到名称上
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // 其它情况
        n             => println!("I'm an old person of age {:?}", n),
    }
}

fn chapter8_6() {
    // Option<i32> 类型
    let optional = Some(7);

    match optional {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
        },
        _ => {},    // 因为还有None的情况没有覆盖。。。
    };

    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // 换个简洁的写法，等效于前面的 match 代码
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // 如果要指明失败情形，就使用 else：
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // 这个和前面的 _ => 就差不多了吧
        println!("Didn't match a number. Let's go with a letter!");
    };

    let i_like_letters = false;

    // 可以和普通的 if/else 结合起来使用
    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    };

    enum Foo {
        Bar,
        Baz,
        Qux(u32)
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);
    
    // if let 还可以匹配枚举值
    if let Foo::Bar = a {
        println!("a is foobar");
    }
    
    // 没有匹配上
    if let Foo::Bar = b {
        println!("b is foobar");
    }
    
    // 和前面的 Some() 类似，可以提取值
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }
}

fn chapter8_7() {
   let mut optional = Some(0);

    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);     // 这里的 optional 是不是覆盖了原来那个？
                }
            },
            _ => { break; }
        }
    }

    let mut optional = Some(0);

    // 更简洁的写法来了！
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }   // 这个就不能搭配 else 了哈
}

fn main() {
    chapter8_1();
    chapter8_2();
    chapter8_2_1();
    chapter8_2_2();
    chapter8_3();
    chapter8_4();
    chapter8_5();
    chapter8_5_1_1();
    chapter8_5_1_2();
    chapter8_5_1_3();
    chapter8_5_1_4();
    chapter8_5_2();
    chapter8_5_3();
    chapter8_6();
    chapter8_7();
}
