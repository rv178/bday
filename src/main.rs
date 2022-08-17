use chrono::prelude::*;
use prettytable::Table;
use rand;
use serde_derive::{Deserialize, Serialize};
use std::{cmp::Ordering, env, fs, io, io::Write, process::exit};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Bdays {
    list: Vec<Person>,
    index: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Person {
    name: String,
    bday: String,
    id: u32,
}

impl Bdays {
    fn new() -> Bdays {
        Bdays {
            list: Vec::new(),
            index: 1,
        }
    }
}

impl Person {
    fn new(name: String, bday: String, id: u32) -> Person {
        Person { name, bday, id }
    }
}

fn main() -> io::Result<()> {
    let mut bdays = Bdays::new();
    let path: &String = &format!("/home/{}/.config/bdays.json", env!("USER"));

    if let Ok(json) = fs::read_to_string(path) {
        let bday_list: Vec<Person> = serde_json::from_str(&json)?;
        bdays.list = bday_list;
        bdays.index = bdays.list.len() as u32 + 1;
    } else {
        let json = serde_json::to_vec(&bdays.list)?;
        fs::write(path, json)?;
    }

    let args: Vec<String> = env::args().collect();
    match args.len().cmp(&1) {
        Ordering::Greater => match args[1].as_str() {
            "help" => {
                help();
                exit(0);
            }
            "list" => {
                let mut table = Table::new();
                table.add_row(prettytable::row!["ID", "Name", "Birthday"]);
                for person in &bdays.list {
                    table.add_row(prettytable::row![person.id, person.name, person.bday]);
                }
                table.printstd();
                exit(0);
            }
            "add" => {
                let name = input("Enter name: ").expect("Enter a valid name.");
                let bday = input("Enter birthday date (day-month-year): ")
                    .expect("Enter a valid birthday.");

                let split = bday.split("-").collect::<Vec<&str>>();
                let day = split[0].parse::<u32>().unwrap();
                let month = split[1].parse::<u32>().unwrap();
                let year = split[2].parse::<i32>().unwrap();

                let date = NaiveDate::from_ymd(year, month, day)
                    .format("%A, %d %B %Y")
                    .to_string();

                let id = (bdays.index + rand::random::<u32>()) % 1000;
                bdays.index += 1;

                let person = Person::new(name, date, id);

                bdays.list.push(person);
            }
            "rm" => {
                if args.len() < 3 {
                    println!("Please enter an id.");
                    exit(1);
                }

                let id = args[2].parse::<u32>().expect("Enter a valid id.");

                let mut index = 0;
                let mut found = false;

                for person in &bdays.list {
                    if person.id == id {
                        found = true;
                        break;
                    }
                    index += 1;
                }

                if found {
                    bdays.list.remove(index);
                    println!("Removed person with ID {}.", id);
                } else {
                    println!("No person with ID {} found.", id);
                }
            }
            _ => {
                println!("Invalid option '{}'.", args[1]);
                exit(1);
            }
        },
        Ordering::Equal => {
            help();
        }
        Ordering::Less => {
            exit(1);
        }
    }

    let json = serde_json::to_vec(&bdays.list)?;
    fs::write(path, json)?;

    Ok(())
}

fn input(msg: &str) -> io::Result<String> {
    let mut reply: String = String::new();
    print!("{}", msg);

    io::stdout().flush()?;
    io::stdin().read_line(&mut reply)?;

    Ok(reply.trim_end().to_owned())
}

fn help() {
    let help_msg = format!(
        "\x1b[32m\x1b[1mBday \x1b[0m {}
    Birthday tracker.
\x1b[33mUSAGE:\x1b[0m
    bday \x1b[32m[OPTIONS]\x1b[0m
\x1b[33mOPTIONS:\x1b[0m
    \x1b[32mhelp\x1b[0m
        Show this help message.
    \x1b[32mrm [id]\x1b[0m
        Remove a person.
    \x1b[32mlist\x1b[0m
        List birthdays.
    \x1b[32madd\x1b[0m
        Add a person.
Link: \x1b[4m\x1b[34mhttps://github.com/rv178/rvfetch\x1b[0m",
        env!("CARGO_PKG_VERSION")
    );
    println!("{}", help_msg);
}
