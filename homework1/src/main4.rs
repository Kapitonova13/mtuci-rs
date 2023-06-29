// 3.3, 3.4, 3.5 (3 глава)
// ФУНКЦИИ

// fn main() {
//     // println!("Hello, world!");

//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }


// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// Операторы и выражения
// Оператор

// fn main() {
//     let y = 6;
// }
// Выражение
// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {y}");
// }

// Функции с возвращаемыми значениями

// fn five() -> i32 {
//     5
// }
// fn main() {
//     let x = five();
//     println!("The value of x is: {x}");
// }


// fn main() {
//     let x = plus_one(5);
//     println!("The value of x is: {x}");
// }
// fn plus_one(x: i32) -> i32 {
//     x + 1
// }


// УПРАВЛЯЮЩИЕ КОНСТРИКЦИИ
// Выражение if
// fn main() {
//     let number = 3;

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }


// fn main() {
//     let number = 3;

//     if number {
//         println!("number was three");
//     }
// }


// fn main() {
//     let number = 3;

//     if number != 0 {
//         println!("number was something other than zero");
//     }
// }

// Обработка нескольких условий с помощью else if

// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// Использование if в let-операторах

// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };

//     println!("The value of number is: {number}");
// }

// fn main() {
//     let condition = true;

//     let number = if condition { 5 } else { "six" };

//     println!("The value of number is: {number}");
// }

// Повторение выполнения кода с помощью loop

// fn main() {
//     loop {
//         println!("again!");
//     }
// }


// Возвращение значений из циклов
// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");
// }

// Метки циклов для устранения неоднозначности между несколькими циклами
// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }

// Циклы с условием while

// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }


// Цикл по элементам коллекции с помощью for

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("the value is: {}", a[index]);

//         index += 1;
//     }
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a {
//         println!("the value is: {element}");
//     }
// }


fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}









