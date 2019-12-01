use std::env;
use std::io;
use std::io::Write;

fn main() {
    // Call with --fuel-mass to calculate part 2 answer
    let use_fuel_mass = env::args()
        .nth(1)
        .map_or(false, |arg| "--fuel-mass".eq(&arg));

    let mut total_fuel: i32 = 0;
    loop {
        let mut mass_str = String::new();
        print!("Enter mass (enter nothing to finish adding modules): ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut mass_str)
            .expect("failed to get mass from terminal");

        let trimmed_mass_str = mass_str.trim();

        if trimmed_mass_str.len() == 0 {
            break;
        }

        let mass: i32 = trimmed_mass_str
            .parse()
            .expect("failed to parse mass from terminal");

        let mut fuel = calculate_fuel_mass(mass);

        if use_fuel_mass {
            let mut last_fuel = fuel;
            while last_fuel > 0 {
                last_fuel = calculate_fuel_mass(last_fuel);
                fuel += last_fuel.max(0);
            }
        }

        println!("Fuel required for mass {}: {}", mass, fuel);
        total_fuel += fuel;
    }

    println!("Total fuel for modules: {}", total_fuel);
}

fn calculate_fuel_mass(mass: i32) -> i32 {
    // Div 3, floor, minus 2
    (mass as f32 / 3f32).floor() as i32 - 2
}
