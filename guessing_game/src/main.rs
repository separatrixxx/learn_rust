use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Угадай число!\n");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Пожалуйста, введите число :)\n");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Не удалось прочитать строку\n");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Ваше предположение: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком маленькое!\n"),
            Ordering::Greater => println!("Слишком большое!\n"),
            Ordering::Equal => {
                println!("Вы угадали, загаданное число: {}!\n", guess);
                break;
            }
        }
    }
}
