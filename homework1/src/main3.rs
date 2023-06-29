// 3.1, 3.2 (3 глава) 
fn main() {

    // ПЕРЕМЕННЫЕ И ИЗМЕНЯЕМОСТЬ

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Константы
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Const: {THREE_HOURS_IN_SECONDS}");

    // Затенение 1
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Затенение 2
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces = {spaces}");

    // ТИПЫ ДАННЫХ
    
    // Числа с плавающей точкой
    // let x = 2.0; // f64

    // let y: f32 = 3.0; // f32

    // Числовые операции
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
    println!("sum = {sum}, diff = {difference}, pr = {product}, q = {quotient}, tr = {truncated}, r = {remainder}");

    // Логический тип данных
    let t = true;
    let f: bool = false; // with explicit type annotation

    // Символьный тип данных
    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // Кортеж
    let tup: (i32, f64, u8) = (500, 6.4, 1);
  
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y, z, x is: {y}, {z}, {x}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("five_hundred = {five_hundred}, six_point_four = {six_point_four}, one = {one}");

    // Массивы
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    println!("first = {first}, second = {second}");
    
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let b: [i32; 5] = [1, 2, 3, 4, 5];
    
    let m = [3; 5];
    for i in m{
        print!("{}", i);}

    
}


    