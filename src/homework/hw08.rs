use std::io;

// Функція для перевірки, чи є число простим
fn is_prime(num: u64) -> bool {
    // Числа менші або рівні 1 не є простими
    if num <= 1 {
        return false;
    }
    // 2 та 3 є простими числами
    if num <= 3 {
        return true;
    }
    // Якщо число ділиться на 2 або 3, воно не є простим (крім самих 2 і 3)
    if num % 2 == 0 || num % 3 == 0 {
        return false;
    }
    // Перевіряємо дільники від 5
    // Можна перевіряти числа у формі 6k ± 1 до кореня з num
    let mut i = 5;
    while i * i <= num {
        if num % i == 0 || num % (i + 2) == 0 {
            return false;
        }
        i += 6; // Наступні можливі дільники
    }
    true
}

fn main() {
    println!("Введіть ціле невід'ємне число для перевірки на простоту:");

    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).expect("Не вдалося прочитати рядок");

    let number: u64 = match input_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Будь ласка, введіть дійсне ціле невід'ємне число.");
            return;
        }
    };

    if is_prime(number) {
        println!("Число {} є простим.", number);
    } else {
        println!("Число {} не є простим.", number);
    }
}
