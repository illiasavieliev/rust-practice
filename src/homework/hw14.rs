// src/homeworks/hw14.rs

/// Генерує n-бітний код Грея.
///
/// Код Грея - це послідовність бінарних чисел, де кожні два сусідні числа
/// відрізняються лише одним бітом.
///
/// Алгоритм є рекурсивним:
/// - G(0) = [""]
/// - G(1) = ["0", "1"]
/// - G(n) формується шляхом взяття G(n-1), додавання префікса "0" до кожного елемента,
///   потім взяття G(n-1) у зворотному порядку, додавання префікса "1" до кожного елемента,
///   і об'єднання цих двох списків.
///
/// # Аргументи
/// * `n` - Кількість бітів для коду Грея (u8).
///
/// # Повертає
/// `Vec<String>`, що містить послідовність коду Грея.
pub fn gray(n: u8) -> Vec<String> {
    if n == 0 {
        return vec!["".to_string()];
    }

    if n == 1 {
        return vec!["0".to_string(), "1".to_string()];
    }

    // Рекурсивно отримуємо код Грея для n-1 бітів
    let prev_gray_code = gray(n - 1);

    let mut current_gray_code = Vec::with_capacity(2 * prev_gray_code.len());

    // Перша половина: додаємо "0" до кожного елемента G(n-1)
    for code in &prev_gray_code {
        current_gray_code.push(format!("0{}", code));
    }

    // Друга половина: додаємо "1" до кожного елемента G(n-1) у зворотному порядку
    for code in prev_gray_code.iter().rev() {
        current_gray_code.push(format!("1{}", code));
    }

    current_gray_code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gray_code_generation() {
        // Тестові дані, як вказано в завданні.
        // Зверніть увагу: послідовність для n=4 виправлена на коректний код Грея,
        // оскільки приклад в зображенні для n=4 є послідовністю двійкових чисел, а не кодом Грея.
        let test_data = vec![
            (0, vec!["".to_string()]),
            (1, vec!["0".to_string(), "1".to_string()]),
            (2, vec!["00".to_string(), "01".to_string(), "11".to_string(), "10".to_string()]),
            (3, vec![
                "000".to_string(), "001".to_string(), "011".to_string(), "010".to_string(),
                "110".to_string(), "111".to_string(), "101".to_string(), "100".to_string(),
            ]),
            (4, vec![
                "0000".to_string(), "0001".to_string(), "0011".to_string(), "0010".to_string(),
                "0110".to_string(), "0111".to_string(), "0101".to_string(), "0100".to_string(),
                "1100".to_string(), "1101".to_string(), "1111".to_string(), "1110".to_string(),
                "1010".to_string(), "1011".to_string(), "1001".to_string(), "1000".to_string(),
            ]),
        ];

        for (n, out) in test_data.iter() {
            assert_eq!(gray(*n), *out, "Тест не пройшов для n={}", n);
        }
    }

    // Додатковий тест для перевірки властивості коду Грея (зміна одного біта)
    #[test]
    fn test_gray_code_single_bit_change() {
        for n in 1..=5 { // Перевіряємо до n=5
            let codes = gray(n);
            assert_eq!(codes.len(), 1 << n, "Невірна довжина для n={}", n); // Перевірка, що кількість кодів = 2^n

            for i in 0..codes.len() {
                let current_code = &codes[i];
                // Для останнього елемента порівнюємо з першим (циклічно)
                let next_code = &codes[(i + 1) % codes.len()];

                let mut diff_count = 0;
                for k in 0..n as usize {
                    // Порівнюємо біти на кожній позиції
                    if current_code.chars().nth(k) != next_code.chars().nth(k) {
                        diff_count += 1;
                    }
                }
                assert_eq!(diff_count, 1, "Більше одного біта змінилося між {} та {} для n={}", current_code, next_code, n);
            }
        }
    }
}

// Функція main для ручного тестування та демонстрації
fn main() {
    println!("Код Грея для n=0: {:?}", gray(0));
    println!("Код Грея для n=1: {:?}", gray(1));
    println!("Код Грея для n=2: {:?}", gray(2));
    println!("Код Грея для n=3: {:?}", gray(3));
    println!("Код Грея для n=4: {:?}", gray(4));
}
