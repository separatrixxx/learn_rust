use std::io;

fn main() {
    loop {
        println!("\nВведите температуру:\n");

        let mut temp: String = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Не удалось прочитать строку\n");

        let temp_len: usize = temp.trim().chars().count();

        let temp_number: &str = &temp[0..temp_len - 1];
        let temp_type: &str = &temp[temp_len - 1..temp_len];

        let temp_number: f64 = match temp_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("Температура должна состоять из числа и обозначения шкалы в конце\n");
                continue;
            },
        };

        if temp_type == "C" || temp_type == "c" {
            let temp_new: f64 = (temp_number * 9.0 / 5.0) + 32.0;
            print!("{temp_number}°C = {temp_new}°F\n");
        } else if temp_type == "F" || temp_type == "f" {
            let temp_new: f64 = (temp_number - 32.0) * 5.0 / 9.0;
            print!("{temp_number}°F = {temp_new}°C\n");
        } else {
            print!("Температура должна состоять из числа и обозначения шкалы в конце\n");
        }
    }
}
