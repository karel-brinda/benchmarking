use std::env;
use std::process;
use std::thread::sleep;
use std::time::Duration;
use std::ptr;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Ensure the user provided exactly one argument
    if args.len() != 2 {
        eprintln!("Usage: {} <amount>", args[0]);
        process::exit(1);
    }

    let amount_str = &args[1];

    // Parse the memory amount
    match parse_amount(amount_str) {
        Ok(size) => {
            // Allocate the memory
            let mut memory: Vec<u8> = Vec::with_capacity(size as usize);
            unsafe { memory.set_len(size as usize); }

            // Touch each page to ensure the memory is actually allocated
            touch_memory(&mut memory);

            // Wait for one second
            sleep(Duration::from_secs(1));
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

// Function to parse the input amount with optional suffix
fn parse_amount(s: &str) -> Result<u64, String> {
    let s = s.trim();

    // Split the numeric part and the suffix
    let mut chars = s.chars().peekable();
    let mut num_str = String::new();

    while let Some(&c) = chars.peek() {
        if c.is_digit(10) || c == '.' {
            num_str.push(c);
            chars.next();
        } else {
            break;
        }
    }

    let suffix: String = chars.collect();

    if num_str.is_empty() {
        return Err("Invalid number format".to_string());
    }

    let num: f64 = num_str.parse().map_err(|_| "Invalid number format")?;

    let multiplier = match suffix.trim().to_uppercase().as_str() {
        "B" | "" => 1.0,
        "KB" => 1e3,
        "MB" => 1e6,
        "GB" => 1e9,
        "TB" => 1e12,
        _ => return Err(format!("Unknown suffix: {}", suffix)),
    };

    let bytes = num * multiplier;

    if bytes.fract() != 0.0 {
        return Err("Amount must be an integer number of bytes".to_string());
    }

    if bytes <= 0.0 {
        return Err("Amount must be positive".to_string());
    }

    if bytes > u64::MAX as f64 {
        return Err("Amount is too large".to_string());
    }

    Ok(bytes as u64)
}

// Function to touch each memory page to ensure allocation
fn touch_memory(vec: &mut Vec<u8>) {
    // Define the page size (common default is 4096 bytes)
    let page_size = 4096;

    let ptr = vec.as_mut_ptr();
    let size = vec.len();

    let mut i = 0;

    unsafe {
        while i < size {
            ptr::write_volatile(ptr.add(i), 0);
            i += page_size;
        }
    }
}
