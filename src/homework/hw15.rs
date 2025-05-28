// src/homeworks/hw15.rs

use std::collections::HashMap;

// Структура для зберігання знайденого рішення
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Solution {
    pub m: i32, pub u: i32, pub x: i32, pub a: i32,
    pub s: i32, pub l: i32, pub o: i32, pub n: i32,
}

// Реалізація відображення для Solution, щоб виводити у заданому форматі
impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "  {}{}{}{}\n", self.m, self.u, self.x, self.a
        )?;
        write!(f,
            "x       {}\n", self.a
        )?;
        write!(f,
            "  ------\n"
        )?;
        write!(f,
            "  {}{}{}{}\n", self.s, self.l, self.o, self.n
        )
    }
}

/// Знаходить всі можливі рішення криптарифметичної задачі "МУХА * А = СЛОН".
/// Кожна літера відповідає унікальній цифрі від 1 до 8.
///
/// # Повертає
/// Вектор структур `Solution`, кожна з яких представляє знайдене рішення.
pub fn find_cryptarithmetic_solution() -> Vec<Solution> {
    let mut solutions: Vec<Solution> = Vec::new();
    let mut assigned_values: HashMap<char, i32> = HashMap::new();
    // Використовуємо масив used_digits для відстеження використання цифр від 1 до 8.
    // Розмір 9, щоб індексувати від 1. used_digits[0] не використовується.
    let mut used_digits: [bool; 9] = [false; 9];

    // Порядок літер для систематичного присвоєння цифр
    let letters = ['М', 'У', 'Х', 'А', 'С', 'Л', 'О', 'Н'];

    // Рекурсивна допоміжна функція для генерації перестановок та перевірки
    // `k` - поточний індекс літери, якій ми намагаємося присвоїти цифру
    fn solve(
        k: usize,
        letters: &[char],
        assigned_values: &mut HashMap<char, i32>,
        used_digits: &mut [bool; 9],
        solutions: &mut Vec<Solution>,
    ) {
        // Базовий випадок: всі літери призначені
        if k == letters.len() {
            // Отримуємо значення, присвоєні кожній літері
            let m_val = *assigned_values.get(&'М').unwrap();
            let u_val = *assigned_values.get(&'У').unwrap();
            let x_val = *assigned_values.get(&'Х').unwrap();
            let a_val = *assigned_values.get(&'А').unwrap();
            let s_val = *assigned_values.get(&'С').unwrap();
            let l_val = *assigned_values.get(&'Л').unwrap();
            let o_val = *assigned_values.get(&'О').unwrap();
            let n_val = *assigned_values.get(&'Н').unwrap();

            // Формуємо числа
            let muha = m_val * 1000 + u_val * 100 + x_val * 10 + a_val;
            let slon = s_val * 1000 + l_val * 100 + o_val * 10 + n_val;

            // Перевіряємо рівняння
            if muha * a_val == slon {
                solutions.push(Solution {
                    m: m_val, u: u_val, x: x_val, a: a_val,
                    s: s_val, l: l_val, o: o_val, n: n_val,
                });
            }
            return;
        }

        let current_letter = letters[k];

        // Перебираємо кожну цифру від 1 до 8 для поточної літери
        for digit in 1..=8 {
            if !used_digits[digit] { // Якщо цифра ще не була використана
                used_digits[digit] = true; // Позначаємо як використану
                assigned_values.insert(current_letter, digit); // Присвоюємо

                // Рекурсивний виклик для наступної літери
                solve(k + 1, letters, assigned_values, used_digits, solutions);

                // Відкат (Backtrack): знімаємо позначку використання цифри та видаляємо присвоєння
                assigned_values.remove(&current_letter);
                used_digits[digit] = false;
            }
        }
    }

    // Запускаємо рекурсивний пошук з першої літери (індекс 0)
    solve(0, &letters, &mut assigned_values, &mut used_digits, &mut solutions);
    solutions
}

// Основна функція для запуску рішення та виведення результатів
fn main() {
    println!("--- Завдання 15: Криптарифметична задача ---");
    println!("У цьому записі треба замість літер поставити числа від 1 до 8.");
    println!("Однаковим літерам відповідають однакові числа.");
    println!("\n  МУХА");
    println!("x    А");
    println!("------");
    println!("  СЛОН\n");

    let solutions = find_cryptarithmetic_solution();

    if solutions.is_empty() {
        println!("Рішень не знайдено.");
    } else {
        println!("Знайдено {} рішення(нь):", solutions.len());
        for (i, sol) in solutions.iter().enumerate() {
            println!("\nРішення {}:", i + 1);
            println!("{}", sol);

            // Виведення присвоєних значень для кожної змінної
            println!("Де:");
            println!("  М = {}", sol.m);
            println!("  У = {}", sol.u);
            println!("  Х = {}", sol.x);
            println!("  А = {}", sol.a);
            println!("  С = {}", sol.s);
            println!("  Л = {}", sol.l);
            println!("  О = {}", sol.o);
            println!("  Н = {}", sol.n);
        }
    }
}

// Тест для перевірки, що функція знаходить рішення
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cryptarithmetic_solver() {
        let solutions = find_cryptarithmetic_solution();
        // Запускаємо перевірку. Очікується, що буде знайдено хоча б одне рішення.
        // Зазвичай, для таких задач є одне або декілька рішень.
        assert!(!solutions.is_empty(), "Не знайдено жодного рішення для задачі 'МУХА x А = СЛОН'");

        // Приклад перевірки одного з рішень, якщо воно буде знайдено.
        // Цей тест може бути розширений для перевірки конкретного відомого рішення,
        // якщо воно є детермінованим.
        // Наприклад, для рішення: М=1, У=7, Х=5, А=3, С=5, Л=2, О=6, Н=9
        // 1753 * 3 = 5259
        // Але в нашому випадку цифри від 1 до 8, тому 9 не може бути.
        // Тому необхідно покладатися на повний перебір.
        // Додаємо перевірку, що всі знайдені рішення є коректними.
        for sol in solutions {
            let muha = sol.m * 1000 + sol.u * 100 + sol.x * 10 + sol.a;
            let slon = sol.s * 1000 + sol.l * 100 + sol.o * 10 + sol.n;
            assert_eq!(muha * sol.a, slon, "Знайдене рішення невірне: {} * {} != {}", muha, sol.a, slon);

            // Перевірка унікальності цифр (хоча це гарантується алгоритмом `solve`)
            let mut digits = vec![sol.m, sol.u, sol.x, sol.a, sol.s, sol.l, sol.o, sol.n];
            digits.sort_unstable();
            // Цифри повинні бути від 1 до 8 і всі унікальні
            assert_eq!(digits, vec![1, 2, 3, 4, 5, 6, 7, 8], "Цифри в рішенні не унікальні або не в діапазоні 1-8");
        }
    }
}
