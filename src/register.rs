use clap::builder::Str;
use rusqlite::{params, Connection, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use rpassword::read_password;
use std::io::{self, Read, Write};
use std::fs::{self, OpenOptions};



// File to store the session username
const SESSION_FILE: &str = "session.txt";

// Function to save the username to a session file
 fn save_username_to_file(username: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(SESSION_FILE)?;
    file.write_all(username.as_bytes())?;
    Ok(())
}

// Function to read the username from a session file
pub fn read_username_from_file() -> io::Result<Option<String>> {
    let mut file = match OpenOptions::new().read(true).open(SESSION_FILE) {
        Ok(f) => f,
        Err(_) => return Ok(None), // If file does not exist, return None
    };
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    if username.is_empty() {
        Ok(None)
    } else {
        Ok(Some(username))
    }
}

// Function to remove the session file (log out)
pub fn remove_session_file() -> io::Result<()> {
    fs::remove_file(SESSION_FILE)?;
    Ok(())
}





// Create or connect to a SQLite database
pub fn create_connection() -> Result<Connection> {
    let conn = Connection::open("users.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            signed_in BOOLEAN NOT NULL DEFAULT 0
        )",
        [],
    )?;
    Ok(conn)
}

pub fn register_user(conn: &Connection) ->Result<String> {
    let mut username = String::new();
    print!("Enter your username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).expect("Failed to read username");
    let username = username.trim();

    print!("Enter your password: ");
    io::stdout().flush().unwrap();
    let password = read_password().expect("Failed to read password");

    let hashed_password = hash(password, DEFAULT_COST).unwrap();
    conn.execute(
        "INSERT INTO users (username, password) VALUES (?1, ?2)",
        params![username, hashed_password],
    )?;
    
    println!("Registration successful.");
   Ok(username.to_string())
}


pub fn sign_in_user(conn: &Connection) -> Result<Option<String>> {
    let mut username = String::new();
    print!("Enter your username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).expect("Failed to read username");
    let username = username.trim().to_string();

    let mut stmt = conn.prepare("SELECT password FROM users WHERE username = ?1")?;
    let stored_password: String = stmt.query_row(params![username], |row| row.get(0))?;
    print!("Enter your password: ");
    io::stdout().flush().unwrap();
    let password = read_password().expect("Failed to read password");
    // Verify the password
    if verify(password, &stored_password).unwrap() {
        conn.execute(
            "UPDATE users SET signed_in = 1 WHERE username = ?1",
            params![username],
        )?;
        println!("Sign in successful! Welcome, {}!", username);
        let _ = save_username_to_file(&username);
        Ok(Some(username))
    } else {
        println!("Invalid password.");
        Ok(None)  
    }

}

pub fn register() ->Result<String> {
    let conn = create_connection()?;
    let mut  username = String::new();
    loop {
        println!("\nDo you want to (r)egister or (s)ign in? (q to quit): ");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read choice");

        match choice.trim().to_lowercase().as_str() {
            "r" => {
             username =   register_user(&conn)?;
            }
            "s" => {
            if sign_in_user(&conn)?.is_some(){
                break;
                }
            }
            "q" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid option. Please enter 'r', 's', or 'q'.");
            }
        }
       
    }

    Ok(username)
}
