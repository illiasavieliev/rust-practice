use std::io;

// Функція для зміни регістру символів:
// великі -> малі, малі -> великі, інші символи залишаються без змін
fn swap_case(s: &str) -> String {
    s.chars().map(|c| {
        if c.is_uppercase() {
            c.to_lowercase().next().unwrap_or(c)
        } else if c.is_lowercase() {
            c.to_uppercase().next().unwrap_or(c)
        } else {
            c
        }
    }).collect()
}

fn main() {
    println!("Введіть рядок для зміни регістру:");

    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Не вдалося прочитати рядок");

    // Видаляємо символ нового рядка, який read_line() додає в кінці
    let input_string = input_string.trim();

    let swapped_string = swap_case(input_string);

    println!("Оригінальний рядок: \"{}\"", input_string);
    println!("Рядок зі зміненим регістром: \"{}\"", swapped_string);
}
