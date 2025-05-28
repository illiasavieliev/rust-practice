use std::io;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        if a > b {
            a %= b;
        } else {
            b %= a;
        }
    }
    a
}

fn main() {
    println!("Введіть перше число:");
    let mut num1_str = String::new();
    io::stdin().read_line(&mut num1_str).expect("Не вдалося прочитати рядок");
    let num1: u64 = num1_str.trim().parse().expect("Будь ласка, введіть дійсне число.");

    println!("Введіть друге число:");
    let mut num2_str = String::new();
    io::stdin().read_line(&mut num2_str).expect("Не вдалося прочитати рядок");
    let num2: u64 = num2_str.trim().parse().expect("Будь ласка, введіть дійсне число.");

    let result = gcd(num1, num2);
    println!("Найбільший спільний дільник {} і {} дорівнює: {}", num1, num2, result);
}
