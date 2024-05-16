use std::{fs::{OpenOptions, File}, io::{Write, BufReader, BufRead}};
use rand::seq::SliceRandom;
use std::io;
use rocket::{serde::{Deserialize, Serialize, json::Json}, post, fs::NamedFile, response::Redirect};
use rand::thread_rng;
use rocket::form::{Form, FromForm};

#[macro_use] extern crate rocket;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Company {
    name: String,
    rank: u32, 
    market_value_mil: u64,
}

fn read_csv() -> Result<Vec<Company>, io::Error> {
    let file = File::open("f500data.csv").expect("Unable to open file");
    let mut reader = csv::Reader::from_reader(file);
    let mut companies = Vec::new();

    for record in reader.deserialize::<Company>() {
        let record = record?;
        // println!("{} {} {}", record.name, record.rank, record.market_value_mil);
        companies.push(record);  
    }
    Ok(companies)
}


#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Task<'r> {
    item: &'r str
}

fn get_random_company(companies: &Vec<Company>) -> Option<&Company> {
    let mut rng = thread_rng();
    companies.choose(&mut rng)
}

#[get("/")]
fn index() -> Result<Json<Company>, &'static str> {
    let companies = read_csv().map_err(|_| "Failed to read CSV")?;
    let company = get_random_company(&companies).ok_or("No companies available")?;
    Ok(Json(company.clone()))
}

#[get("/readtasks")]
fn read_tasks() -> Json<Vec<String>> {
    let tasks = OpenOptions::new()
                    .read(true)
                    .append(true)
                    .create(true)
                    .open("tasks.txt")
                    .expect("unable to access tasks.txt");
    let reader = BufReader::new(tasks);
    Json(reader.lines()
            .map(|line| {
                let line_string: String = line.expect("could not read line");
                let line_pieces: Vec<&str> = line_string.split(",").collect();
                // line_pieces[1].to_string()
                format!("ID: {}, Task: {}", line_pieces[0], line_pieces[1])
            })
            .collect())
}

#[post("/addtask", data="<task>")]
fn add_task(task: Json<Task<'_>>) -> &'static str {
    let mut tasks = OpenOptions::new()
                    .read(true)
                    .append(true)
                    .create(true)
                    .open("tasks.txt")
                    .expect("unable to access tasks.txt");   
    let reader = BufReader::new(&tasks);
    let id = reader.lines().count();
    let task_item_string = format!("{},{}\n", id, task.item);
    let task_item_bytes = task_item_string.as_bytes();
    tasks.write(task_item_bytes).expect("unable to write to tasks.txt");
    "Task added succesfully"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, add_task, read_tasks])
}