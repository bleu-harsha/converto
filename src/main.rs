use std::env;
use colored::*;// make sure you're using `coloured`, not `colored`

fn main() {
    let args: Vec<String> = env::args().collect();

    let all_args = args[1..].join(" ").to_lowercase(); // join all args after program name

    if all_args.contains("help") || all_args.contains("-h") || all_args.contains("--help") {
        show_help();
    } else if all_args.contains("health") {
        println!("Health check requested (not implemented yet).");
     // call `check_file_health()` here if you'd like
    } else if all_args.contains("-zip"){
        println!("Zip flag detected (zip feature not implemented yet).");
        // zip logic here
    } else {
        println!("Unknown command: '{}'", all_args);
    }
}

// Display help screen
fn show_help() {
    // ASCII Logo
    println!(
        "{}",
        r#"
                                    __         
.----.-----.-----.--.--.-----.----.|  |_.-----.
|  __|  _  |     |  |  |  -__|   _||   _|  _  |
|____|_____|__|__|\___/|_____|__|  |____|_____|
"#.bold().purple()
    );

    // Heading
    println!("{}", "Welcome to the Converto Tool!\n".bold());

    // Commands section
    println!("{}", "Available Commands:".bold().truecolor(255, 165, 0));
    println!("{}", " • converto help                        shows the help screen".dimmed());
    println!("{}", " • converto <file.ext> -flag            converts a file to another filetype from the chosen flag".dimmed());
    println!("{}", " • converto <file.ext> <saveloc> -flag  converts a file to another filetype from the chosen flag in the chosen directory".dimmed());
    println!("{}", " • converto <file.ext> health           checks a file's health\n".dimmed());

    // Flags section
    println!("{}", "Available Flags:".bold().truecolor(255, 165, 0));
    println!("{}", " • -<ext>         convert to that file type (e.g. -png, -mp3)".dimmed());
    println!("{}", " • -rename / -ren rename a file".dimmed());
    println!("{}", " • -force         force an operation (needs sudo on Linux)".dimmed());
    println!("{}", " • -zip / -zip<n> zip a file with compression level 1–10".bold().dimmed());
    println!("{}", " • -help / -h     shows the help screen\n".dimmed());

    println!("{}", "'conv' works the same as 'converto'!".dimmed());
    println!("{}", "Submit issues at https://github.com/bleu-harsha/converto/issues".underline().cyan());
}

// Health checker (not used yet)
fn check_file_health(file_path: &str) {
    use std::fs;

    match fs::metadata(file_path) {
        Ok(metadata) => {
            println!("File: {}", file_path);
            println!("Size: {} bytes", metadata.len());
            println!("Read-only: {}", metadata.permissions().readonly());
        }
        Err(e) => println!("File error: {}", e),
    }
}
