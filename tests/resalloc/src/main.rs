use clap::{ArgGroup, Parser}; // Ensure this is included
use std::process;
use std::ptr;
use std::thread::sleep;
use std::time::Duration;

/// Memory Allocator
///
/// This program allocates a specified amount of RAM and waits for a specified duration before exiting.
/// It supports memory sizes with suffixes like 'KB', 'MB', 'GB', and 'TB'.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(
    ArgGroup::new("time")
        .required(false)
        .args(&["duration", "wait_forever"]),
))]


struct Args {
    /// Amount of RAM to allocate (e.g., 512MB, 2GB)
    #[arg(short, long, value_name = "SIZE", required = true)]
    memory: String,

    /// Duration to wait in seconds (e.g., 10)
    #[arg(short, long, value_name = "SECONDS", conflicts_with = "wait_forever")]
    duration: Option<u64>,

    /// Wait indefinitely until interrupted
    #[arg(short = 'F', long, conflicts_with = "duration")]
    wait_forever: bool,
}

fn main() {
    let args = Args::parse();

    // Parse the memory amount
    match parse_amount(&args.memory) {
        Ok(size) => {
            // Allocate the memory
            let mut memory: Vec<u8> = Vec::with_capacity(size as usize);
            unsafe { memory.set_len(size as usize); }

            // Touch each page to ensure the memory is actually allocated
            touch_memory(&mut memory);

            // Wait for the specified duration
            if args.wait_forever {
                println!("Allocated {} bytes. Waiting indefinitely...", size);
                loop {
                    sleep(Duration::from_secs(1));
                }
            } else {
                let wait_time = args.duration.unwrap_or(1);
                println!("Allocated {} bytes. Waiting for {} second(s)...", size, wait_time);
                sleep(Duration::from_secs(wait_time));
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

/// Parses the memory amount string and returns the size in bytes.
///
/// # Arguments
///
/// * `s` - A string slice that holds the memory amount (e.g., "512MB")
///
/// # Returns
///
/// * `Result<u64, String>` - The size in bytes or an error message
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

    let num: f64 = num_str.parse().map_err(|_| "Invalid number format".to_string())?;

    let multiplier = match suffix.trim().to_uppercase().as_str() {
        "B" | "" => 1.0,
        "KB" => 1e3,
        "K" => 1e3,
        "MB" => 1e6,
        "M" => 1e6,
        "GB" => 1e9,
        "G" => 1e9,
        "TB" => 1e12,
        "T" => 1e12,
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

/// Touches each memory page to ensure that the operating system allocates physical memory.
///
/// # Arguments
///
/// * `vec` - A mutable reference to the vector representing allocated memory
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
