use std::io;

fn main() {
    let mut input = String::new();

    println!("hey, I'm learning rust");

    // Take input from user
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
       
    let trimmed = input.trim();
    // Help command check
    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    if trimmed.eq_ignore_ascii_case("conv help")
    || trimmed.eq_ignore_ascii_case("converto help")
    || trimmed.eq_ignore_ascii_case("-h")
    || trimmed.eq_ignore_ascii_case("--help"){
        show_help();
    } else {
        println!("Unknown command. Try 'conv help' or 'converto help'");
    }
}

// Help display function
fn show_help() {
    println!(
        r#"
                                    __         
.----.-----.-----.--.--.-----.----.|  |_.-----.
|  __|  _  |     |  |  |  -__|   _||   _|  _  |
|____|_____|__|__|\___/|_____|__|  |____|_____|

Welcome to the Converto Tool!

Available Commands:
  •converto help                        shows the help screen
  •converto <file.ext> -flag            converts a file to another filetype from the chosen flag
  •converto <file.ext> <saveloc> -flag  converts a file to another filetype from the chosen flag in the chosen directory(renaming a file to convert is also possible)
  •converto <file.ext> health           checks a file's health

Available Flags:
 • -<ext>                               replace <ext> with the desired file type which is to be converted. ex: -png
 • -rename                              rename a file, you can also use -ren to do that
 • -force                               force an operartion requires superuser perms 
 • -zip                                 convert a file to a .zip with the default medium compression level,use -zip<n> to state compression level in a scale of 1 to 10

 • -help                                shows the help screen works with even -h 


You can use conv interchangably fot converto!
To submit issues visit https://github.com/bleu-harsha/converto/issues
"#
    );
}

