fn chapter5_1() {
    #![allow(overflowing_literals)]

    let decimal = 65.4321_f32;

    // 不提供隐式转换！
    // let integer: u8 = decimal;

    // 可以显式转换
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // 1000 已经在 u16 的范围内
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 当把任何类型转换为无符号类型 T 时，
    // 实际上的处理方式是按位宽从低位进行截取，
    // 逻辑上相当于不断加上或减去 (std::T::MAX + 1)，
    // 直到结果值位于新类型 T 的范围内。

    // 1000 = 0x3E8 => 0xE8 = 232
    println!("1000 as a u8 is : {}", 1000 as u8);

    // -1 = 0xFF => 0xFF = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // 对正数，这就和取模一样。
    println!("1000 mod 256 is : {}", 1000 % 256);

    // 当转换到有符号类型时，实际上也是从截取，
    // 但逻辑上相当于先转换到对应的无符号类型，
    // 然后再看最高位，如果是 1 则结果值为负（用补码的方式来解读）

    // 已经在目标类型的范围内
    println!(" 128 as a i16 is: {}", 128 as i16);

    // 128 = 0x80 = 0b1000_0000 => -128（1000_0000取反加一，即10000000）
    println!(" 128 as a i8 is : {}", 128 as i8);

    // 232 = 0xE8 = 0b1110_1000 =>  -24（1110_1000取反加一，为11000）
    println!(" 232 as a i8 is : {}", 232 as i8);
}

fn chapter5_2() {
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    let i = 1;      // 默认i32
    let f = 1.0;    // 默认f64

    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}

fn chapter5_3() {
    let elem = 5u8;

    // rust 的类型推导不只是看定义时的右值类型~
    let mut vec = Vec::new();
    // 现在编译器还不知道 `vec` 的具体类型，只知道它是某种东西构成的向量（`Vec<_>`）
    
    // 如果注释下面这一行，build 的时候会报 type annotations needed / cannot infer type for `T`
    vec.push(elem);
    // 啊哈！现在编译器知道 `vec` 是 u8 的向量了（`Vec<u8>`）。

    println!("{:?}", vec);
}

fn chapter5_4() {
    // `NanoSecond` 是 `u64` 的新名字。
    type NanoSecond = u64;
    type Inch = u64;

    // 别名必须是 Camel Case~
    #[allow(non_camel_case_types)]
    type u64_t = u64;

    // `NanoSecond` = `Inch` = `u64_t` = `u64`.
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    // 注意类型别名*并不能*提供额外的类型安全，因为别名*并不是*新的类型。
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}

fn main() {
    chapter5_1();
    chapter5_2();
    chapter5_3();
    chapter5_4();
}
