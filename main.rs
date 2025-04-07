// Дается некий диапазон целых чисел, например от 17 до 24 включительно.
// Напишите программу, которая будет находить сумму целых чисел указанного диапазона.

use rand::Rng;

fn main() {
    let mut rng = rand::rng();

    let arr = [
        rng.random_range(17..25),
        rng.random_range(17..25),
        rng.random_range(17..25),
        rng.random_range(17..25),
        rng.random_range(17..25),
        rng.random_range(17..25),
        rng.random_range(17..25),
    ];

    let sum: i32 = arr.iter().sum();
    println!("Для массива {:?}, сумма равна {}", arr, sum);
}
