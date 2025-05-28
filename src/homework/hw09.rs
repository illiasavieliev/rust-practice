use std::io;

/// Виконує циклічний зсув рядка.
///
/// # Аргументи
/// * `s` - Вхідний рядок.
/// * `shift_amount` - Величина зсуву.
///   - Позитивне значення: зсув вправо.
///   - Негативне значення: зсув вліво.
///
/// # Повертає
/// Новий рядок з виконаним циклічним зсувом.
fn cyclic_shift(s: &str, shift_amount: i32) -> String {
    let n = s.len();

    // Якщо рядок порожній або складається з одного символу, зсув не має ефекту.
    if n == 0 {
        return String::new();
    }
    if n == 1 {
        return s.to_string();
    }

    // Нормалізуємо величину зсуву, щоб вона була в межах [0, n)
    // і представляла собою ефективний правий зсув.
    let mut effective_shift = shift_amount % (n as i32);
    if effective_shift < 0 {
        effective_shift += n as i32; // Перетворюємо негативний зсув на еквівалентний позитивний
    }

    let k = effective_shift as usize;

    // Розділяємо рядок на дві частини та об'єднуємо їх у новому порядку
    // Приклад для правого зсуву на k: останні k символів переміщуються на початок.
    // s = "abcdef", k = 2 (правий зсув) -> "ef" + "abcd" -> "efabcd"
    // Це відповідає s[n-k..n] + s[0..n-k]
    let (left_part, right_part) = s.split_at(n - k);
    format!("{}{}", right_part, left_part)
}

fn main() {
    println!("Введіть рядок:");
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).expect("Не вдалося прочитати рядок");
    let input_str = input_str.trim();

    println!("Введіть величину зсуву (позитивне для правого, негативне для лівого):");
    let mut shift_amount_str = String::new();
    io::stdin().read_line(&mut shift_amount_str).expect("Не вдалося прочитати рядок");
    let shift_amount: i32 = match shift_amount_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Будь ласка, введіть дійсне ціле число.");
            return;
        }
    };

    let shifted_string = cyclic_shift(input_str, shift_amount);

    println!("Оригінальний рядок: \"{}\"", input_str);
    println!("Величина зсуву: {}", shift_amount);
    println!("Зсунутий рядок: \"{}\"", shifted_string);

    // Приклад тесткейсів (якби вони були у файлі):
    // Припустимо, що у файлі були такі тесткейси:
    // "abc", 1 -> "cab" (right shift by 1)
    // "abc", -1 -> "bca" (left shift by 1)
    // "hello", 2 -> "llohe" (right shift by 2)
    // "rust", -2 -> "str" (left shift by 2)
    // "test", 4 -> "test" (shift by length)
    // "test", 0 -> "test" (shift by 0)

    println!("\nПриклади тесткейсів (для перевірки):");
    println!("cyclic_shift(\"abc\", 1) -> \"{}\"", cyclic_shift("abc", 1));   // Очікується: "cab"
    println!("cyclic_shift(\"abc\", -1) -> \"{}\"", cyclic_shift("abc", -1)); // Очікується: "bca"
    println!("cyclic_shift(\"hello\", 2) -> \"{}\"", cyclic_shift("hello", 2)); // Очікується: "llohe"
    println!("cyclic_shift(\"rust\", -2) -> \"{}\"", cyclic_shift("rust", -2)); // Очікується: "str"
    println!("cyclic_shift(\"test\", 4) -> \"{}\"", cyclic_shift("test", 4)); // Очікується: "test"
    println!("cyclic_shift(\"test\", 0) -> \"{}\"", cyclic_shift("test", 0)); // Очікується: "test"
    println!("cyclic_shift(\"Rust\", 5) -> \"{}\"", cyclic_shift("Rust", 5)); // Очікується: "tRus" (shift by 5 is like shift by 1 for length 4)
    println!("cyclic_shift(\"Hello World\", -13) -> \"{}\"", cyclic_shift("Hello World", -13)); // Очікується: "lo WorldHel"
}
