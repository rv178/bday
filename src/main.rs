use chrono::prelude::*;
use prettytable::{color, Attr, Cell, Row, Table};
use serde_derive::{Deserialize, Serialize};
use std::{cmp::Ordering, env, fs, io, process::exit};

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
    let path: &str = &format!(
        "{}/.config/bdays.json",
        env::var("HOME").expect("Failed to get home directory.")
    );

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
            "ls" | "list" => {
                let mut table = Table::new();
                table.add_row(Row::new(vec![
                    Cell::new("ID")
                        .with_style(Attr::ForegroundColor(color::GREEN))
                        .with_style(Attr::Bold),
                    Cell::new("Name")
                        .with_style(Attr::ForegroundColor(color::GREEN))
                        .with_style(Attr::Bold),
                    Cell::new("Birthday")
                        .with_style(Attr::ForegroundColor(color::GREEN))
                        .with_style(Attr::Bold),
                    Cell::new("Till next bday")
                        .with_style(Attr::ForegroundColor(color::GREEN))
                        .with_style(Attr::Bold),
                    Cell::new("Age at next bday")
                        .with_style(Attr::ForegroundColor(color::GREEN))
                        .with_style(Attr::Bold),
                ]));

                let now = Utc::now().naive_local().date();

                // sort birthdays by remaining days
                bdays.list.sort_by(|a, b| {
                    let count_a = next_occurance(now, parse_date(&a.bday))
                        .signed_duration_since(now)
                        .num_days();
                    let count_b = next_occurance(now, parse_date(&b.bday))
                        .signed_duration_since(now)
                        .num_days();
                    count_a.cmp(&count_b)
                });

                for person in &bdays.list {
                    let bday = parse_date(&person.bday);
                    let next_bday = next_occurance(now, bday);

                    table.add_row(Row::new(vec![
                        Cell::new(&person.id.to_string())
                            .with_style(Attr::ForegroundColor(color::BRIGHT_BLUE)),
                        Cell::new(&person.name)
                            .with_style(Attr::ForegroundColor(color::BRIGHT_CYAN)),
                        Cell::new(&get_formatted_date(&person.bday))
                            .with_style(Attr::ForegroundColor(color::BRIGHT_CYAN)),
                        Cell::new(&format!(
                            "{} days",
                            next_bday.signed_duration_since(now).num_days() - 1
                        ))
                        .with_style(Attr::ForegroundColor(color::BRIGHT_CYAN)),
                        Cell::new(&format!("{} years", (next_bday.year() - bday.year())))
                            .with_style(Attr::ForegroundColor(color::BRIGHT_CYAN)),
                    ]));
                }
                table.printstd();
                exit(0);
            }
            "add" => {
                if args.len() < 4 {
                    println!("Enter the required arguments! \"bday add [name] [day-month-year]\"");
                    exit(0);
                }

                let name = &args[2];
                let bday = &args[3];

                let split = bday.split('-').collect::<Vec<&str>>();
                let day = split[0].parse::<u32>().unwrap();
                let month = split[1].parse::<u32>().unwrap();
                let year = split[2].parse::<i32>().unwrap();

                let date = NaiveDate::from_ymd(year, month, day)
                    .format("%d %B %Y")
                    .to_string();

                let id = (bdays.index + rand::random::<u32>()) % 1000;
                bdays.index += 1;

                println!(
                    "Added \"{}\" with birthday on {}. (ID: {}).",
                    name, date, id
                );
                let person = Person::new(name.to_owned(), date, id);

                bdays.list.push(person);
            }
            "rm" | "remove" => {
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

fn help() {
    let help_msg = format!(
        "\x1b[32m\x1b[1mBday \x1b[0m {}
    Birthday tracker.

\x1b[33mUSAGE:\x1b[0m
    bday \x1b[32m[OPTIONS]\x1b[0m

\x1b[33mOPTIONS:\x1b[0m
    \x1b[32mhelp\x1b[0m
        Show this help message.
    \x1b[32mrm/remove [id]\x1b[0m
        Remove a person.
    \x1b[32mls/list\x1b[0m
        List birthdays.
    \x1b[32madd [name] [day-month-year]\x1b[0m
        Add a person.

Link: \x1b[4m\x1b[34mhttps://github.com/rv178/bday\x1b[0m",
        env!("CARGO_PKG_VERSION")
    );
    println!("{}", help_msg);
}

fn get_formatted_date(date: &str) -> String {
    parse_date(date).format("%d %B").to_string()
}

fn parse_date(date: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date, "%d %B %Y").expect("Birthday not formatted properly!")
}

fn next_occurance(now: NaiveDate, date: NaiveDate) -> NaiveDate {
    if date.month() <= now.month() && date.day() <= now.day() {
        date.with_year(now.year() + 1)
            .expect("Oops something went wrong!")
    } else if date.month() < now.month() && date.day() >= now.day() {
        date.with_year(now.year() + 1)
            .expect("Oops something went wrong!")
    } else {
        date.with_year(now.year())
            .expect("Oops something went wrong!")
    }
}
