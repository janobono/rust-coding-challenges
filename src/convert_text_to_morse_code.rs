enum Pulse {
    Short,
    Long,
}

type Letter = Vec<Pulse>;

type Message = Vec<Letter>;

trait MorseCode {
    fn to_morse_code(&self) -> Message;
}

impl std::fmt::Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::Short => write!(f, "."),
            Pulse::Long => write!(f, "_"),
        }
    }
}

impl MorseCode for String {
    fn to_morse_code(&self) -> Message {
        let mut result: Vec<Vec<Pulse>> = Vec::with_capacity(self.len());
        for c in self.chars() {
            result.push(match c {
                'A' | 'a' => vec![Pulse::Short, Pulse::Long],
                'B' | 'b' => vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Short],
                'C' | 'c' => vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Short],
                'D' | 'd' => vec![Pulse::Long, Pulse::Short, Pulse::Short],
                'E' | 'e' => vec![Pulse::Short],
                'F' | 'f' => vec![Pulse::Short, Pulse::Short, Pulse::Long, Pulse::Short],
                'G' | 'g' => vec![Pulse::Long, Pulse::Long, Pulse::Short],
                'H' | 'h' => vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Short],
                'I' | 'i' => vec![Pulse::Short, Pulse::Short],
                'J' | 'j' => vec![Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Long],
                'K' | 'k' => vec![Pulse::Long, Pulse::Short, Pulse::Long],
                'L' | 'l' => vec![Pulse::Short, Pulse::Long, Pulse::Short, Pulse::Short],
                'M' | 'm' => vec![Pulse::Long, Pulse::Long],
                'N' | 'n' => vec![Pulse::Long, Pulse::Short],
                'O' | 'o' => vec![Pulse::Long, Pulse::Long, Pulse::Long],
                'P' | 'p' => vec![Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Short],
                'Q' | 'q' => vec![Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Long],
                'R' | 'r' => vec![Pulse::Short, Pulse::Long, Pulse::Short],
                'S' | 's' => vec![Pulse::Short, Pulse::Short, Pulse::Short],
                'T' | 't' => vec![Pulse::Long],
                'U' | 'u' => vec![Pulse::Short, Pulse::Short, Pulse::Long],
                'V' | 'v' => vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Long],
                'W' | 'w' => vec![Pulse::Short, Pulse::Long, Pulse::Long],
                'X' | 'x' => vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Long],
                'Y' | 'y' => vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Long],
                'Z' | 'z' => vec![Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Short],
                _ => continue,
            })
        }
        return result;
    }
}

pub fn print_morse_code(data: &str) {
    let morse = data.to_string().to_morse_code();
    for letter in morse.iter() {
        for pulse in letter.iter() {
            print!("{}", pulse);
        }
        print!(" ");
    }
    println!();
}
