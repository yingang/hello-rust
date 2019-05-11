fn chapter3_1() {
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    // 单元结构体
    struct Nil;

    // 元组结构体
    struct Pair(i32, f32);  // 也就是具名元组

    // 带有两个字段（field）的结构体
    #[derive(Debug)]
    struct Point {
        x: f32,
        y: f32,
    }

    // 结构体可以作为另一个结构体的字段
    #[derive(Debug)]
    struct Rectangle {
        p1: Point,      // bottom-right?
        p2: Point,      // top-left?
    }

    fn rect_area(rect: Rectangle) -> f32 {
        let Rectangle {p1: Point {x: x1, y: y1}, p2: Point {x: x2, y: y2}} = rect;
        (x2 - x1) * (y1 - y2)
    }

    fn square(p: Point, len: f32) -> Rectangle {
        Rectangle {p1: Point {x: p.x + len, y: p.y}, p2: Point {x: p.x, y: p.y + len}}
    }

    // 使用简单的写法初始化字段，并创建结构体
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    
    // 以 Debug 方式打印结构体
    println!("{:?}", peter);
    
    // 实例化结构体 `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // 访问 point 的字段
    println!("point coordinates: ({}, {})", point.x, point.y);
    
    // 使用结构体更新语法创建新的 point，这样可以用到之前的 point 的字段
    let new_point = Point { x: 0.1, ..point };
    
    // `new_point.y` 与 `point.y` 一样，因为这个字段就是从 `point` 中来的
    println!("second point: ({}, {})", new_point.x, new_point.y);
    
    // 使用 `let` 绑定来解构 point
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // 结构体的实例化也是一个表达式
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    println!("rect_area {}", rect_area(_rectangle));

    let rect = square(Point {x: 3.0, y: 4.0}, 1.0);
    println!("new rect from square: {:?}", rect);

    // 实例化一个单元结构体
    let _nil = Nil;

    // 实例化一个元组结构体
    let pair = Pair(1, 0.1);

    // 访问元组结构体的字段
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 解构一个元组结构体
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}

fn chapter3_2() {

}

fn main() {
    chapter3_1();
    chapter3_2();
}
