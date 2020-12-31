use std::error::Error;

use argh::FromArgs;

mod database;
mod bruteforce;
mod charset;


/// Password database
#[derive(FromArgs)]
struct Args {
    #[argh(subcommand)]
    command: Command,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum Command {
    AddUser(AddUser),
    ListUsers(ListUsers),
    Auth(Auth),
    Bruteforce(Bruteforce),
}

#[derive(FromArgs)]
/// Add a user to database
#[argh(subcommand, name = "add-user")]
struct AddUser {
    #[argh(positional)]
    username: String,
    #[argh(positional)]
    password: String,
}

#[derive(FromArgs)]
/// List users
#[argh(subcommand, name = "list-users")]
struct ListUsers {}


#[derive(FromArgs)]
/// Authenticate as a user
#[argh(subcommand, name = "auth")]
struct Auth {
    #[argh(positional)]
    username: String,

    #[argh(positional)]
    password: String,
}

#[derive(FromArgs)]
/// Try to brute-force user accounts
#[argh(subcommand, name = "bruteforce")]
struct Bruteforce {}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let args: Args = argh::from_env();

    match args.command {
        Command::AddUser(args) => {
            let mut db = database::Database::load_or_create()?;
            db.records.insert(args.username.clone(), md5::compute(args.password.trim().as_bytes()).to_vec());
            println!("User {} added", args.username);
            db.save()?;
            Ok(())
        }
        Command::ListUsers(_) => {
            let db = database::Database::load_or_create()?;
            for k in db.records.keys() {
                println!(" - {}", k)
            }
            Ok(())
        }
        Command::Auth(args) => {
            let db = database::Database::load_or_create()?;
            let entered = md5::compute(args.password);
            match db.records.get(&args.username) {
                Some(stored) if stored == &entered.to_vec() => {
                    println!("Authentication successful!");
                }
                Some(_) => {
                    println!("Bad password.");
                }
                None => {
                    println!("No such user.");
                }
            }
            Ok(())
        },
        Command::Bruteforce(_) => {
            bruteforce::bruteforce()
        },
    }
}
