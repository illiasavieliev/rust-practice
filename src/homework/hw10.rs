use std::io;

// Функція для перевірки, чи є число паліндромом
fn is_palindromic_number(num: u64) -> bool {
    // Від'ємні числа зазвичай не вважаються паліндромами
    // В цьому випадку, ми використовуємо u64, тому від'ємні числа неможливі.
    // Однозначні числа завжди є паліндромами.
    if num < 10 {
        return true;
    }

    let original_num = num;
    let mut reversed_num = 0;
    let mut temp_num = num;

    while temp_num > 0 {
        let digit = temp_num % 10; // Отримуємо останню цифру
        reversed_num = reversed_num * 10 + digit; // Додаємо цифру до оберненого числа
        temp_num /= 10; // Видаляємо останню цифру з тимчасового числа
    }

    original_num == reversed_num
}

fn main() {
    println!("Введіть ціле невід'ємне число для перевірки на паліндромність:");

    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).expect("Не вдалося прочитати рядок");

    let number: u64 = match input_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Будь ласка, введіть дійсне ціле невід'ємне число.");
            return;
        }
    };

    if is_palindromic_number(number) {
        println!("Число {} є паліндромом.", number);
    } else {
        println!("Число {} не є паліндромом.", number);
    }
}
