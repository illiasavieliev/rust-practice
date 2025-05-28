use rand::Rng; // Імпортуємо трейт Rng для генерації випадкових чисел

// 1. Функція для генерації випадкового вектора
//    Довжина: n
//    Значення: [10..99]
fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng(); // Створюємо генератор випадкових чисел
    let mut vector = Vec::with_capacity(n); // Задаємо початкову ємність для ефективності

    for _ in 0..n {
        vector.push(rng.gen_range(10..100)); // Генеруємо число в діапазоні [10, 99]
    }
    vector
}

// 2. Функція для знаходження мінімальної суми сусідніх елементів
//    Повертає Option<i32>, тому що вектор може не мати сусідніх пар.
fn min_adjacent_sum(data: &[i32]) -> Option<i32> {
    // Якщо вектор має менше 2 елементів, сусідніх пар немає
    if data.len() < 2 {
        return None;
    }

    // Ініціалізуємо мінімальну суму сумою першої пари
    let mut min_sum = data[0] + data[1];

    // Проходимо по решті пар
    for i in 1..(data.len() - 1) { // Ітеруємо від другого елемента до передостаннього
        let current_sum = data[i] + data[i + 1];
        if current_sum < min_sum {
            min_sum = current_sum;
        }
    }
    Some(min_sum)
}

// 3. Функція для виведення результатів у зрозумілому вигляді
fn print_results(vector: &[i32], min_sum_option: Option<i32>) {
    println!("Згенерований вектор: {:?}", vector);

    match min_sum_option {
        Some(min_sum) => {
            // Знаходимо першу пару, яка дає мінімальну суму, для виводу
            let mut found_pair = false;
            for i in 0..(vector.len() - 1) {
                if vector[i] + vector[i+1] == min_sum {
                    println!("Мінімальна сума сусідніх елементів: {} (пара: ({}, {}))", min_sum, vector[i], vector[i+1]);
                    found_pair = true;
                    break; // Виводимо лише першу знайдену пару
                }
            }
            // Цей блок не повинен бути досяжний, якщо min_sum_option є Some,
            // але для повноти або якщо логіка зміниться.
            if !found_pair {
                println!("Помилка: мінімальна сума {} знайдена, але відповідна пара не виявлена.", min_sum);
            }
        },
        None => {
            println!("Немає сусідніх пар у векторі (довжина вектора менше 2).");
        }
    }
}

fn main() {
    let vector_length = 20; // Задаємо довжину вектора згідно з завданням

    // Генеруємо вектор
    let random_vec = gen_random_vector(vector_length);

    // Знаходимо мінімальну суму сусідніх елементів
    let min_sum = min_adjacent_sum(&random_vec);

    // Виводимо результати
    print_results(&random_vec, min_sum);

    // Приклад з вектором, де немає сусідніх пар
    println!("\n--- Перевірка для коротких векторів ---");
    let short_vec1 = gen_random_vector(0);
    let min_sum1 = min_adjacent_sum(&short_vec1);
    print_results(&short_vec1, min_sum1);

    let short_vec2 = gen_random_vector(1);
    let min_sum2 = min_adjacent_sum(&short_vec2);
    print_results(&short_vec2, min_sum2);
}
