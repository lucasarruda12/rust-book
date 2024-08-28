use std::io;

enum Degree {
    Celsius (f64),
    Fahrenheit (f64),
}

impl Degree {
    fn to_fahrenheit(&self) -> Degree {
        match self {
            Degree::Celsius(c) => {
                Degree::Fahrenheit((*c * 9.0/5.0) + 32.0)
            }

            Degree::Fahrenheit(f) => Degree::Fahrenheit(*f),
        }
    }

    fn to_celsius(&self) -> Degree {
        match self {
            Degree::Celsius(c) => Degree::Celsius(*c),

            Degree::Fahrenheit(f) => {
                Degree::Celsius((*f - 32.0) * 5.0/9.0)
            }
        }
    }
}

fn main() {
    println!("Converting from Fahrenheit to Celsius");

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let input: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let deg: Degree = Degree::Fahrenheit(input);

        match deg.to_celsius() {
            Degree::Celsius(c) => {
                println!("{}", c);
            }
            Degree::Fahrenheit(_) => {
                panic!();
            }
        }
    }
}
