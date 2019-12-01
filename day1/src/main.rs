use std::io;
use std::io::Write;

fn main() {
    let mut total_fuel: i32 = 0;
    loop {
        let mut mass_str = String::new();
        print!("Enter mass (enter nothing to finish adding modules): ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut mass_str)
            .expect("failed to get mass from terminal");

        if mass_str.len() == 0 {
            break;
        }

        let mass: i32 = mass_str
            .trim()
            .parse()
            .expect("failed to parse mass from terminal");

        // Div 3, floor, minus 2
        let fuel: i32 = (mass as f32 / 3f32).floor() as i32 - 2;
        println!("Fuel required for mass {}: {}", mass, fuel);

        total_fuel += fuel;
    }

    println!("Total fuel for modules: {}", total_fuel);
}
