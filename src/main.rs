use std::io::{self, Write};

fn main() {
    println!("🔐 Secure Password Generator\n");

    let length = get_password_length();
    let options = get_password_options();

    let password = generate_password(length, options);

    println!("\n✨ Your generated password:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("{}", password);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("\n📊 Password strength: {}", calculate_strength(&password));
}

struct PasswordOptions {
    include_uppercase: bool,
    include_lowercase: bool,
    include_numbers: bool,
    include_symbols: bool,
}

fn get_password_length() -> usize {
    loop {
        print!("Enter password length (8-128): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<usize>() {
            Ok(num) if num >= 8 && num <= 128 => return num,
            _ => println!("❌ Please enter a number between 8 and 128"),
        }
    }
}

fn get_password_options() -> PasswordOptions {
    println!("\nSelect character types (y/n):");

    PasswordOptions {
        include_uppercase: prompt_yes_no("Include uppercase letters (A-Z)?"),
        include_lowercase: prompt_yes_no("Include lowercase letters (a-z)?"),
        include_numbers: prompt_yes_no("Include numbers (0-9)?"),
        include_symbols: prompt_yes_no("Include symbols (!@#$%^&*)?"),
    }
}

fn prompt_yes_no(question: &str) -> bool {
    loop {
        print!("{} ", question);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("❌ Please enter 'y' or 'n'"),
        }
    }
}

fn generate_password(length: usize, options: PasswordOptions) -> String {
    let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let lowercase = "abcdefghijklmnopqrstuvwxyz";
    let numbers = "0123456789";
    let symbols = "!@#$%^&*()_+-=[]{}|;:,.<>?";

    let mut charset = String::new();

    if options.include_uppercase {
        charset.push_str(uppercase);
    }
    if options.include_lowercase {
        charset.push_str(lowercase);
    }
    if options.include_numbers {
        charset.push_str(numbers);
    }
    if options.include_symbols {
        charset.push_str(symbols);
    }

    if charset.is_empty() {
        charset.push_str(lowercase); // Default fallback
    }

    let chars: Vec<char> = charset.chars().collect();
    let mut password = String::new();

    // Simple pseudo-random generation using system time
    use std::time::{SystemTime, UNIX_EPOCH};
    let mut seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    for _ in 0..length {
        seed = (seed.wrapping_mul(1103515245).wrapping_add(12345)) % (1 << 31);
        let index = (seed as usize) % chars.len();
        password.push(chars[index]);
    }

    password
}

fn calculate_strength(password: &str) -> &str {
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_numeric());
    let has_symbol = password.chars().any(|c| !c.is_alphanumeric());

    let variety = [has_upper, has_lower, has_digit, has_symbol]
        .iter()
        .filter(|&&x| x)
        .count();

    match (password.len(), variety) {
        (16.., 4) => "💪 Very Strong",
        (12.., 3..) => "🔒 Strong",
        (10.., 2..) => "⚠️  Medium",
        _ => "❌ Weak",
    }
}
