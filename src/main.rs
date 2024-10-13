use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct Book {
    id: u32,
    title: String,
    author: String,
}

struct AppState {
    data_file: Mutex<String>,  //JSON filepath put on a mutex to limit concurents acceses
}

//read data from a JSON file function
fn read_data(file_path: &str) -> Vec<Book> {
    let data = fs::read_to_string(file_path).expect("Error, can't read file.");
    serde_json::from_str(&data).expect("Error in the JSON conversion")
}

//write data on a JSON file function
fn write_data(file_path: &str, books: &Vec<Book>) {
    let data = serde_json::to_string_pretty(&books).expect("Error while serializing the data");
    fs::write(file_path, data).expect("Error can't write in file");
}

#[get("/books")]
async fn get_books(data: web::Data<AppState>) -> impl Responder {
    let books = read_data(&data.data_file.lock().unwrap());
    HttpResponse::Ok().json(books)
}

#[post("/books")]
async fn add_book(data: web::Data<AppState>, new_book: web::Json<Book>) -> impl Responder {
    let mut books = read_data(&data.data_file.lock().unwrap());

    //add a new book and save data in the JSON file
    books.push(new_book.into_inner());
    write_data(&data.data_file.lock().unwrap(), &books);
    
    HttpResponse::Ok().json("Book added")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //filepath of the JSON "database"
    let data_file = "data/books.json".to_string();

    //HTTP server strating
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                data_file: Mutex::new(data_file.clone()),
            }))
            .service(get_books)
            .service(add_book)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
