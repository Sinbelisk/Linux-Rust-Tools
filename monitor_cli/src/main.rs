use clap::{Parser};
use std::process::Command;

/// CLI to get or set monitor VCP features using ddcutil.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Feature VCP code or name (e.g., "brightness", "contrast", or hex code like "10").
    feature: String,

    /// Value to set the feature to. If none, the feature will be read instead.
    #[arg(long, short, conflicts_with = "up", conflicts_with = "down")]
    value: Option<String>,

    /// Adds to the current value of the feature.
    #[arg(long,short, conflicts_with = "down")]
    up: Option<String>,

    /// Subtracts from the current value of the feature.
    #[arg(long,short, conflicts_with = "up")]
    down: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mode : String = get_mode(&args);
    let code = get_vcp_code(&args.feature).expect("Invalid code name");

    // Create the command, first phase
    let mut cmd = Command::new("ddcutil");
    cmd.arg("--bus=6")
        .arg(mode)
        .arg(code.to_string());

    // Second phase: add value if needed
    if let Some(val) = args.value {
        cmd.arg(val);
    }
    else if let Some(up_val) = args.up {
        cmd.arg("+");
        cmd.arg(up_val);
    }
    else if let Some(down_val) = args.down {
        cmd.arg("-");
        cmd.arg(down_val);
    }

    // execute.
    cmd.status()
        .expect("failed to execute process");

}
/// Converts a feature name or code to a valid VCP hex code string.
/// TODO: optimize
fn get_vcp_code(code: &str) -> Option<String> {
    // first, check if the code is already a valid hex code.
    if code.chars().all(|c| c.is_ascii_hexdigit()) && code.len() <= 2 {
        return Some(code.to_uppercase());
    }

    // check if the code is prefixed with "0x"
    if let Some(rest) = code.strip_prefix("0x") {
        if rest.chars().all(|c| c.is_ascii_hexdigit()) {
            return Some(rest.to_uppercase());
        }
    }

    // first, try to parse the code to a number (decimal).
    if let Ok(num) = code.parse::<u8>() {
        return Some(format!("{:02X}", num));
    }

    // otherwise, match known code names.
    match code {
        "brightness" => Some(10.to_string()),
        "contrast" => Some(12.to_string()),
        _ => None,
    }
}

/// Determines the mode (getvcp or setvcp) based on the presence of a value.
fn get_mode(value : &Args) -> String {
    if value.value.is_some() || value.up.is_some() || value.down.is_some() {
        "setvcp".to_string()
    } else {
        "getvcp".to_string()
    }
}



