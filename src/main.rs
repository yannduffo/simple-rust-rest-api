use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;

//------------------------------------------- data structure declaration --------------------------------------------------------
#[derive(Serialize, Deserialize)]
struct Song{
    name:String,
    artist:String
}

#[derive(Serialize, Deserialize)]
struct MenuItem{
    dish:String,
    price:i32
}

#[derive(Serialize,Deserialize)]
struct Restaurant{
    name:String,
    address:String,
    menu: Vec<MenuItem>
}

//datafile
#[derive(Serialize, Deserialize)]
struct DataFile{
    restaurants:Vec<Restaurant>,
    songs:Vec<Song>,
}

struct AppState {
    data_file: Mutex<String>,  //JSON filepath put on a mutex to limit concurents acceses
}

//-------------------------------------------------- read/write data functions --------------------------------------------------

//read data from a JSON file function
fn read_data(file_path: &str) -> DataFile {
    let data = fs::read_to_string(file_path).expect("Error, can't read file.");
    serde_json::from_str(&data).expect("Error in the JSON conversion")
}

//write data on a JSON file function
fn write_data(file_path: &str, data_file:&DataFile) {
    let data = serde_json::to_string_pretty(&data_file).expect("Error while serializing the data");
    fs::write(file_path, data).expect("Error can't write in file");
}

//-------------------------------------------------- endpoints ------------------------------------------------------------------

#[get("/restaurants")]
async fn get_restaurants(data: web::Data<AppState>) -> impl Responder {
    let data_file = read_data(&data.data_file.lock().unwrap());
    
    println!("Received GET request: /restaurants"); //for basic log

    HttpResponse::Ok().json(data_file.restaurants)
}

#[get("/restaurants/{name}/menu")]
async fn get_restaurant_menu(data: web::Data<AppState>, name: web::Path<String>) -> impl Responder {
    let data_file = read_data(&data.data_file.lock().unwrap());

    println!("Received GET request: /restaurants/{}/menu", name); //for basic log

    if let Some(restaurant) = data_file.restaurants.iter().find(|r| r.name == *name) {
        HttpResponse::Ok().json(&restaurant.menu)
    } else {
        HttpResponse::NotFound().body("Restaurant not found")
    }
}

#[get("/songs/{name}")]
async fn get_song_info(data: web::Data<AppState>, name: web::Path<String>) -> impl Responder {
    let data_file = read_data(&data.data_file.lock().unwrap());

    println!("Received GET request: /songs/{}", name); //for basic log

    if let Some(song) = data_file.songs.iter().find(|r| r.name == *name) {
        HttpResponse::Ok().json(&song)
    } else {
        HttpResponse::NotFound().body("Song not found")
    }
}

#[get("/songs")]
async fn get_songs(data: web::Data<AppState>) -> impl Responder {
    let data_file = read_data(&data.data_file.lock().unwrap());

    println!("Received GET request: /songs"); //for basic log

    HttpResponse::Ok().json(data_file.songs)
}

//-------------------------------------------------------- main -----------------------------------------------------------------

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //filepath of the JSON "database"
    let data_file = "data/data.json".to_string();

    println!("Starting server on http://127.0.0.1:8080"); //for basic log

    //HTTP server strating
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                data_file: Mutex::new(data_file.clone()),
            }))
            .service(get_restaurants)
            .service(get_restaurant_menu)
            .service(get_songs)
            .service(get_song_info)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
