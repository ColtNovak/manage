use std::fs::{self, File};
use std::io::{self, Write, BufRead};
use std::env;
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("USAGE: manage <site name here> (or manage new <site name here>)");
        return Ok(());
    }

    if args[1] == "new" {
        if args.len() < 3 {
            println!("USAGE: manage new <site name here>");
            return Ok(());
        }

        let site = &args[2];
        newsite(site)?;
    } else {
        let site = &args[1];
        search(site)?;
    }

    Ok(())
}

fn search(site: &str) -> io::Result<()> {
    let dir = ".creds";
    let file_path = Path::new(dir).join(site);

    if let Ok(file) = File::open(&file_path) {
        let reader = io::BufReader::new(file);
        let mut lines = reader.lines();

        if let Some(Ok(username)) = lines.next() {
            println!("user: {}", username);
        }

        if let Some(Ok(password)) = lines.next() {
            println!("password: {}", password);
        }
    } else {
        println!("Site '{}' does not exist.", site);
    }

    Ok(())
}

fn newsite(site: &str) -> io::Result<()> {
    let dir = ".creds";
    let file_path = Path::new(dir).join(site);

    if !Path::new(dir).exists() {
        fs::create_dir(dir)?;
    }

    if File::open(&file_path).is_ok() {
        println!("'{}' already exists.", site);
        return Ok(());
    }

    let mut file = File::create(&file_path)?;
    println!("Creating '{}'.", site);

    println!("Enter username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;

    println!("Enter password:");
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;

    writeln!(file, "{}", username.trim())?;
    writeln!(file, "{}", password.trim())?;

    println!("Credentials saved to '{}/{}'.", dir, site);
    Ok(())
}
