use rand::Rng;

// 2. Функція, яка рахує мінімальну кількість перенесень грузу,
// щоб на всіх кораблях був однаковий груз.
// Повертає Option<usize>:
//   - Some(кількість перенесень) якщо груз можна розподілити порівну.
//   - None якщо груз неможливо розподілити порівну.
fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    let num_ships = shipments.len();
    if num_ships == 0 {
        return Some(0); // Нуль перенесень для нуля кораблів
    }

    let total_cargo: u32 = shipments.iter().sum();

    // Перевіряємо, чи можливо рівномірно розподілити груз.
    // Це можливо лише тоді, коли загальний груз ділиться на кількість кораблів.
    if total_cargo % (num_ships as u32) != 0 {
        return None; // Неможливо розподілити порівну
    }

    let target_per_ship = total_cargo / (num_ships as u32);
    let mut transfers = 0;
    let mut current_balance: i32 = 0; // Відстежуємо накопичений "борг" або "надлишок"

    // Алгоритм для мінімальної кількості перенесень:
    // Проходимо по кораблях, розраховуючи відхилення кожного корабля від цільового значення.
    // Кожного разу, коли накопичений "баланс" (current_balance) стає ненульовим
    // після обробки чергового корабля, це означає, що потрібно зробити переміщення
    // грузу до/від наступних кораблів, щоб вирівняти цей баланс.
    // Це рахує кількість "ділянок" у послідовності, де груз не збалансований.
    for &cargo in shipments {
        current_balance += (cargo as i32) - (target_per_ship as i32);
        // Якщо поточний баланс не нульовий, це означає, що ми або маємо надлишок,
        // який потрібно перенести, або дефіцит, який потрібно заповнити з наступних кораблів.
        // Отже, потрібен 1 переніс.
        if current_balance != 0 {
            transfers += 1;
        }
    }
    Some(transfers)
}

// 5. Функція генерації Vec<u32>, які можуть бути розподілені однаково.
// Генерує вектор довжиною `n` зі значеннями [10..99],
// сума яких ділиться на `n`.
fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let range_min = 10;
    let range_max = 99; // Включно

    let mut shipments = Vec::with_capacity(n);

    if n == 0 {
        return Vec::new();
    }
    if n == 1 {
        // Для одного корабля завжди можна розподілити, 0 перенесень.
        shipments.push(rng.gen_range(range_min..=range_max));
        return shipments;
    }

    loop { // Цикл, доки не згенеруємо вектор, сума якого ділиться на `n` і всі елементи в діапазоні
        shipments.clear();
        for _ in 0..n {
            shipments.push(rng.gen_range(range_min..=range_max));
        }

        let total_sum: u32 = shipments.iter().sum();
        let remainder = total_sum % (n as u32);

        if remainder == 0 {
            break; // Сума вже ділиться на `n`, все добре
        }

        // Якщо сума не ділиться на `n`, потрібно скоригувати один елемент.
        // Спробуємо скоригувати останній елемент.
        let last_idx = n - 1;
        let original_last_cargo = shipments[last_idx];

        // Варіант 1: Відняти remainder.
        // Це зробить суму кратною `n`. Перевіряємо, чи результат >= range_min.
        let subtract_candidate = original_last_cargo.saturating_sub(remainder);
        if subtract_candidate >= range_min as u32 {
            shipments[last_idx] = subtract_candidate;
            break; // Знайшли рішення, виходимо з циклу
        }

        // Варіант 2: Додати (n - remainder).
        // Це також зробить суму кратною `n`. Перевіряємо, чи результат <= range_max.
        let add_candidate = original_last_cargo.saturating_add((n as u32) - remainder);
        if add_candidate <= range_max as u32 {
            shipments[last_idx] = add_candidate;
            break; // Знайшли рішення, виходимо з циклу
        }

        // Якщо жоден з варіантів коригування останнього елемента не працює (виходить за діапазон),
        // тоді ми перезапускаємо цикл і генеруємо повністю новий вектор.
        // Це гарантує, що вектор завжди буде згенерований відповідно до умов.
    }
    shipments
}

// Функція для виводу результатів у зрозумілому вигляді
fn print_shipment_analysis(shipments: &Vec<u32>) {
    println!("\n--- Аналіз вантажів ---");
    println!("Вантажі на кораблях: {:?}", shipments);
    println!("Кількість кораблів: {}", shipments.len());

    let total_cargo: u32 = shipments.iter().sum();
    println!("Загальний вантаж: {}", total_cargo);

    match count_permutation(shipments) {
        Some(transfers) => {
            let num_ships = shipments.len();
            let target_per_ship = if num_ships > 0 { total_cargo / (num_ships as u32) } else { 0 };
            println!("Цільовий вантаж на корабель: {}", target_per_ship);
            println!("Мінімальна кількість перенесень: {}", transfers);
        },
        None => {
            println!("Неможливо розподілити вантаж порівну (загальний вантаж не ділиться на кількість кораблів).");
        }
    }
}

fn main() {
    println!("--- Завдання 12: Балансування грузу ---");

    // 6. Приклади з поясненнями
    // Приклад 1: Можливо розподілити, потрібні перенесення
    let shipments1 = vec![10, 30, 20]; // Сума = 60, Кораблі = 3, Ціль = 20
    // Відхилення: [-10, +10, 0]
    // Баланс:
    // 1. -10 (≠ 0) -> transfers = 1
    // 2. -10 + 10 = 0 (== 0)
    // 3. 0 + 0 = 0 (== 0)
    // Очікується: 1 переніс
    print_shipment_analysis(&shipments1); // Очікується: 1 переніс

    // Приклад 2: Можливо розподілити, але вже рівномірно
    let shipments2 = vec![25, 25, 25, 25]; // Сума = 100, Кораблі = 4, Ціль = 25
    // Відхилення: [0, 0, 0, 0]
    // Баланс: завжди 0
    // Очікується: 0 перенесень
    print_shipment_analysis(&shipments2); // Очікується: 0 перенесень

    // Приклад 3: Неможливо розподілити
    let shipments3 = vec![10, 20, 25]; // Сума = 55, Кораблі = 3, Ціль = 55/3 (не ціле)
    // Очікується: Неможливо
    print_shipment_analysis(&shipments3); // Очікується: Неможливо

    // Приклад 4: Складніший випадок з кількома перенесеннями
    let shipments4 = vec![0, 0, 0, 100]; // Сума = 100, Кораблі = 4, Ціль = 25
    // Відхилення: [-25, -25, -25, 75]
    // Баланс:
    // 1. -25 (≠ 0) -> transfers = 1
    // 2. -25 + (-25) = -50 (≠ 0) -> transfers = 2
    // 3. -50 + (-25) = -75 (≠ 0) -> transfers = 3
    // 4. -75 + 75 = 0 (== 0)
    // Очікується: 3 перенесення
    print_shipment_analysis(&shipments4); // Очікується: 3 перенесення

    // Приклад 5: Згенерований вектор
    let n_generated = 10;
    let generated_shipments = gen_shipments(n_generated);
    print_shipment_analysis(&generated_shipments);

    let n_generated_large = 20;
    let generated_shipments_large = gen_shipments(n_generated_large);
    print_shipment_analysis(&generated_shipments_large);
}
