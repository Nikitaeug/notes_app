use std::fs::OpenOptions;
use std::fs; // 
use std::io::{self, Write};  // Import Write trait for file writing
use std::process::Command;


fn main() {
    loop {
        // Show the menu
        println!("What would you like to do?");
        println!("1. Make a new note");
        println!("2. View all notes");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        // Try to parse the input, and handle non-number inputs
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,  // return the number
            Err(_) => { // handle non-number inputs
                println!("Please enter a valid number.");
                continue; // restart the loop if parsing fails
            }
        };

        match choice {
            1 => make_note(),
            2 => view_notes(),
            3 => {
                println!("Exiting...");
                break; // exit the loop and program
            }
            _ => {
                println!("Invalid choice. Please enter a number between 1 and 4.");
            }
        }
    }
}

fn make_note() {
    let mut filename = String::new();

    loop {
        println!("Enter the filename:");
        io::stdin().read_line(&mut filename).expect("Failed to read filename");

        // Trim the filename input to avoid unwanted spaces or newlines
        filename = filename.trim().to_string();

        // Check if the filename is empty
        if filename.is_empty() {
            println!("Filename cannot be empty. Please try again.");
            continue; // restart the loop if filename is empty
        }

        // Ensure the filename has a .txt extension
        if !filename.ends_with(".txt") {
            filename.push_str(".txt");
        }

        break; // Exit the loop when a valid filename is entered
    }

    let mut note = String::new();

    loop {
        println!("Enter your note:");
        io::stdin().read_line(&mut note).expect("Failed to read note");

        // Trim the note input to avoid unwanted spaces or newlines
        note = note.trim().to_string();

        // Check if the note is empty
        if note.is_empty() {
            println!("Note cannot be empty. Please try again.");
            continue; // restart the loop if note is empty
        }

        break; // Exit the loop when a valid note is entered
    }

    // Open the file in append mode, or create it if it doesn't exist
    let mut file = OpenOptions::new()
        .append(true)    // Open in append mode to avoid overwriting existing content
        .create(true)    // Create the file if it doesn't exist
        .open(&filename) // Use the user-provided filename with ".txt"
        .expect("Failed to open file");

    // Write the note to the file
    writeln!(file, "{}", note).expect("Failed to write to file");

    println!("Note saved to {}!", filename);
}

fn view_notes() {
    let paths = fs::read_dir(".").expect("Failed to read directory");

    let mut notes_found = false;
    println!("Notes in the current directory:");
    for path in paths {
        let path = path.expect("Failed to read path");
        let filename = path.file_name();
        let filename_str = filename.to_str().expect("Failed to convert filename to string");

        // Check if the file has a .txt extension
        if filename_str.ends_with(".txt") {
            println!("{}", filename_str);
            notes_found = true;
        }
    }

    if !notes_found {
        println!("No notes found.");
        return;
    }

    println!("Enter the filename to view, edit, or delete:");
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).expect("Failed to read filename");
    let filename = filename.trim().to_string();

    // Check if the file exists
    if !fs::metadata(&filename).is_ok() {
        println!("File does not exist. Please try again.");
        return;
    }

    loop {
        println!("What would you like to do with the note?");
        println!("1. View the file");
        println!("2. Delete the file");
        println!("3. Cancel");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        match choice {
            1 => open_file(&filename),
            2 => {
                delete_file(&filename);
                break;
            }
            3 => break,
            _ => println!("Invalid choice. Please enter a number between 1 and 4."),
        }
    }
}

fn open_file(filename: &str) {
    // Open the file with the preferred notes app
    Command::new("notepad")
        .arg(filename)
        .spawn()
        .expect("Failed to open file with notes app");
}


fn delete_file(filename: &str) {
    fs::remove_file(filename).expect("Failed to delete file");
    println!("File {} deleted!", filename);
}