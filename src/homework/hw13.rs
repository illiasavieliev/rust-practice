// src/homeworks/hw13.rs

// Точка на площині
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

// Прямокутник на площині, представлений двома точками:
// 'a' - ліва верхня, 'b' - права нижня.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Rectangle {
    pub a: Point, // Ліва верхня точка
    pub b: Point, // Права нижня точка
}

/// Рахує фактичну зайняту площу набору прямокутників.
/// Використовує алгоритм "горизонтальних смуг".
///
/// # Аргументи
/// * `rects` - Вектор прямокутників, що можуть перетинатися.
///
/// # Повертає
/// Зайняту площу як `i32`.
pub fn area_occupied(rects: &Vec<Rectangle>) -> i32 {
    if rects.is_empty() {
        return 0;
    }

    // Внутрішнє представлення прямокутника для зручності розрахунків:
    // (x_min, y_min, x_max, y_max) де y_min - нижній край, y_max - верхній край.
    // Це потрібно, тому що в завданні 'a' - верхня ліва, 'b' - нижня права.
    let canonical_rects: Vec<(i32, i32, i32, i32)> = rects.iter().map(|r| {
        (r.a.x, r.b.y, r.b.x, r.a.y)
    }).collect();

    // 1. Збираємо всі унікальні Y-координати з усіх прямокутників
    let mut y_coords: Vec<i32> = Vec::new();
    for rect in &canonical_rects {
        y_coords.push(rect.1); // y_min
        y_coords.push(rect.3); // y_max
    }
    y_coords.sort_unstable();
    y_coords.dedup(); // Видаляємо дублікати

    let mut total_occupied_area = 0i32;

    // 2. Ітеруємо по кожній горизонтальній смузі, визначеній послідовними Y-координатами
    for i in 0..(y_coords.len() - 1) {
        let current_y = y_coords[i];
        let next_y = y_coords[i + 1];
        let band_height = next_y - current_y;

        if band_height <= 0 { // Пропускаємо смуги нульової висоти або невалідні
            continue;
        }

        // 3. Знаходимо всі прямокутники, які повністю перетинають цю горизонтальну смугу
        let mut spanning_x_intervals: Vec<(i32, i32)> = Vec::new(); // Зберігає (x_min, x_max)
        for rect in &canonical_rects {
            // Прямокутник перетинає цю смугу, якщо його нижній край <= current_y
            // та його верхній край >= next_y
            if rect.1 <= current_y && rect.3 >= next_y {
                spanning_x_intervals.push((rect.0, rect.2)); // Додаємо (x_min, x_max)
            }
        }

        if spanning_x_intervals.is_empty() {
            continue; // Жодні прямокутники не покривають цю смугу
        }

        // 4. Сортуємо X-інтервали та об'єднуємо перекриваючі, щоб знайти загальну покриту довжину по осі X
        spanning_x_intervals.sort_unstable(); // Сортуємо за x_min

        let mut merged_x_length = 0;
        let mut current_merged_start = spanning_x_intervals[0].0;
        let mut current_merged_end = spanning_x_intervals[0].1;

        for j in 1..spanning_x_intervals.len() {
            let (next_start, next_end) = spanning_x_intervals[j];

            if next_start < current_merged_end { // Перекриття: розширюємо поточний об'єднаний сегмент
                current_merged_end = current_merged_end.max(next_end);
            } else { // Немає перекриття: додаємо довжину попереднього об'єднаного сегмента
                merged_x_length += current_merged_end - current_merged_start;
                // Починаємо новий об'єднаний сегмент
                current_merged_start = next_start;
                current_merged_end = next_end;
            }
        }
        // Додаємо довжину останнього об'єднаного сегмента
        merged_x_length += current_merged_end - current_merged_start;

        // 5. Додаємо площу цієї смуги до загальної
        if merged_x_length > 0 {
            total_occupied_area += merged_x_length * band_height;
        }
    }

    total_occupied_area
}

// Тестові дані, як вказано в завданні
fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle { a: Point { x: 2, y: 9 }, b: Point { x: 5, y: 3 } },   // Червоний
        Rectangle { a: Point { x: 1, y: 8 }, b: Point { x: 11, y: 6 } }, // Зелений
        Rectangle { a: Point { x: 9, y: 10 }, b: Point { x: 13, y: 2 } }, // Синій
    ]
}

// Тест, як вказано в завданні
#[test]
fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60); // Очікувана площа за прикладом
}

// Функція main для ручного тестування та демонстрації
fn main() {
    println!("--- Завдання 13: Розрахунок зайнятої площі ---");

    // Використання тестових даних з завдання
    let example_rects = test_data();
    println!("\nПрямокутники з прикладу завдання:");
    for rect in &example_rects {
        println!("{:?}", rect);
    }
    let area = area_occupied(&example_rects);
    println!("Зайнята площа (за прикладом): {}", area); // Очікується 60

    // Додаткові приклади:

    // Приклад 1: Один прямокутник
    println!("\nПриклад 1: Один прямокутник (10x10)");
    let rects1 = vec![
        Rectangle { a: Point { x: 0, y: 10 }, b: Point { x: 10, y: 0 } },
    ];
    let area1 = area_occupied(&rects1);
    println!("Прямокутники: {:?}", rects1);
    println!("Зайнята площа: {}", area1); // Очікується: 100

    // Приклад 2: Два непересічних прямокутники
    println!("\nПриклад 2: Два непересічних прямокутники");
    let rects2 = vec![
        Rectangle { a: Point { x: 0, y: 5 }, b: Point { x: 5, y: 0 } },
        Rectangle { a: Point { x: 6, y: 5 }, b: Point { x: 10, y: 0 } },
    ];
    let area2 = area_occupied(&rects2);
    println!("Прямокутники: {:?}", rects2);
    println!("Зайнята площа: {}", area2); // Очікується: (5*5) + (4*5) = 25 + 20 = 45

    // Приклад 3: Два прямокутники, що повністю перекриваються (один в іншому)
    println!("\nПриклад 3: Два прямокутники, що повністю перекриваються");
    let rects3 = vec![
        Rectangle { a: Point { x: 0, y: 10 }, b: Point { x: 10, y: 0 } }, // 10x10
        Rectangle { a: Point { x: 2, y: 8 }, b: Point { x: 8, y: 2 } },   // 6x6 всередині
    ];
    let area3 = area_occupied(&rects3);
    println!("Прямокутники: {:?}", rects3);
    println!("Зайнята площа: {}", area3); // Очікується: 100

    // Приклад 4: Три прямокутники, що утворюють L-подібну фігуру (без внутрішнього перекриття)
    println!("\nПриклад 4: Три прямокутники (L-подібна фігура)");
    let rects4 = vec![
        Rectangle { a: Point { x: 0, y: 10 }, b: Point { x: 2, y: 0 } },  // Вертикальна частина: 2x10 = 20
        Rectangle { a: Point { x: 2, y: 2 }, b: Point { x: 10, y: 0 } }, // Горизонтальна частина: 8x2 = 16
        Rectangle { a: Point { x: 3, y: 7 }, b: Point { x: 6, y: 4 } },  // Додатковий невеликий прямокутник
    ];
    let area4 = area_occupied(&rects4);
    // Розрахунок вручну:
    // Об'єднання перших двох: (0,0)-(2,10) U (2,0)-(10,2) = 20 + 16 = 36 (без перекриттів між собою)
    // Третій (3,4)-(6,7) має площу 3*3=9.
    // Перевіряємо перекриття з L-фігурою:
    // З (0,0)-(2,10) - немає
    // З (2,0)-(10,2) - немає
    // Очікується 36 + 9 = 45
    println!("Прямокутники: {:?}", rects4);
    println!("Зайнята площа: {}", area4); // Очікується: 45
}
