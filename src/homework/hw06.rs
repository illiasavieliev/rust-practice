const NUM_TRIANGLES: usize = 3; // Кількість ярусів ялинки

fn main() {
    let mut output = String::new();

    // Висота кожного сегмента трикутника
    let segment_height = 3;
    // Ширина стовбура
    let trunk_width = 3;
    // Висота стовбура
    let trunk_height = 3;

    // Обчислюємо максимальну ширину ялинки для центрування
    let max_tree_width = 1 + (segment_height - 1) * 2 + (NUM_TRIANGLES - 1) * 4;

    // Малюємо ялинкові сегменти
    for i in 0..NUM_TRIANGLES {
        // Базова ширина поточного сегмента
        let current_segment_start_width = 1 + i * 4;
        for j in 0..segment_height {
            let num_stars = current_segment_start_width + j * 2;
            let num_spaces = (max_tree_width - num_stars) / 2;

            output.push_str(&" ".repeat(num_spaces));
            output.push_str(&"*".repeat(num_stars));
            output.push('\n');
        }
    }

    // Малюємо стовбур
    let trunk_spaces = (max_tree_width - trunk_width) / 2;
    for _ in 0..trunk_height {
        output.push_str(&" ".repeat(trunk_spaces));
        output.push_str(&"#".repeat(trunk_width));
        output.push('\n');
    }

    // Виводимо весь сформований рядок
    print!("{}", output);
}
