use std::collections::HashMap;
use std::io;

fn main() {
    let mut company = HashMap::new();

    loop {
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        if command.trim() != "Print company" && command.trim() != "print company" {
            let add = &command.trim()[0..3];
            if add != "add" && add != "Add" {
                println!("Команда должна начинаться либо с add (Add), либо с print (Print)!\n");
            } else {
                let mut name = String::from("");
                let mut department = String::from("");
                let mut spaces = 0;

                for i in command[3..command.len()].trim().chars() {
                    if i == ' ' {
                        spaces += 1;
                    } else {
                        if spaces == 0 {
                            name.push(i);
                        } else if spaces == 2 {
                            department.push(i);
                        }
                    }
                }

                if spaces < 2 {
                    println!("Введите название отдела!\n");
                } else {
                    let exist: String = match company.get(&department) {
                        Some(n) => String::from(n),
                        None => String::from(""),
                    };

                    if exist != "" {
                        company.insert(department, exist + ", " + &name);
                    } else {
                        company.insert(department, name);
                    }
                }
            }
        } else {
            println!("\n");
            for (dep, names) in &company {
                println!("{dep}: {names}");
            }
            println!("\n");
        }
    }
}
