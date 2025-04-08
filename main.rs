//
// Задание. Счастливый билетик
// Раньше в автобусах выдавали билеты с шестизначными номерами.
// Если сумма его первых трех цифр равна сумме его последних трех цифр, то такой билетик считался счастливым.
//
// Пусть у вас в программе задается шестизначное число. Определить, является ли оно счастливым.
//

use rand::Rng;
fn main() {
    let mut rng = rand::rng();
    let tiker = [
        [
            rng.random_range(0..10),
            rng.random_range(0..10),
            rng.random_range(0..10),
        ],
        [
            rng.random_range(0..10),
            rng.random_range(0..10),
            rng.random_range(0..10),
        ],
    ];
    let sum_left: i32 = tiker[0].iter().sum();
    let sum_rigth: i32 = tiker[1].iter().sum();
    println!("{} : {}", sum_left, sum_rigth);
}
