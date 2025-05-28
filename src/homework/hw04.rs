const SIZE: usize = 10;

fn main() {
    if SIZE < 5 || SIZE > 40 {
        eprintln!("Помилка: SIZE має бути в діапазоні від 5 до 40.");
        return;
    }

    let mut output = String::new();

    for i in 0..SIZE {
        for _ in 0..(SIZE - 1 - i) {
            output.push(' ');
        }
        for _ in 0..(2 * i + 1) {
            output.push('*');
        }
        output.push('\n');
    }

    for i in (0..SIZE - 1).rev() {
        for _ in 0..(SIZE - 1 - i) {
            output.push(' ');
        }
        for _ in 0..(2 * i + 1) {
            output.push('*');
        }
        output.push('\n');
    }

    print!("{}", output);
}
