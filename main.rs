// В программе заведите 3 переменные для обозначения слова на трех языках. Создайте кортеж для слова, который будет включать в себя эти три значения.
//
// Далее по ходу программы измените значения исходных переменных. Создайте новый кортеж.
//
// Распечатайте оба кортежа.
//
// Подсказка:
// Тут все просто, главное не забыть сделать первые 3 переменные изменяемыми.
//

fn main() {
    let mut ru: String = "День".to_string();
    let mut de: String = "Tag".to_string();
    let mut en: String = "Day".to_string();
    let words_day: (String, String, String) = (en, ru, de);

    ru = "Луна".to_string();
    de = "Mond".to_string();
    en = "Moon".to_string();
    let words_moon: (String, String, String) = (en, ru, de);
    println!("{:?}", words_day);
    println!("{:?}", words_moon);
}
