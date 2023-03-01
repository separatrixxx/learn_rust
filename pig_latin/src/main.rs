use std::io;

fn main() {
    loop {
        let mut word = String::new();

        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line");

        let word = word.trim();

        let first = &word[0..1].trim();

        if first == &"a"
            || first == &"e"
            || first == &"y"
            || first == &"u"
            || first == &"i"
            || first == &"o"
            || first == &"A"
            || first == &"E"
            || first == &"Y"
            || first == &"U"
            || first == &"I"
            || first == &"O"
        {
            println!("{word}-hay\n");
        } else {
            let rest = &word[1..word.len()].trim();

            println!("{rest}-{first}ay\n");
        }
    }
}
