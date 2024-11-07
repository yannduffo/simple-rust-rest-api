use actix_web::{get, post, put, delete, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;

//------------------------------------------- data structure declaration --------------------------------------------------------
#[derive(Serialize, Deserialize)]
struct MenuItem {
    title: String,
    description: String,
    allergens: String,
    image: String,
    category: String,
    price: i32,
}

#[derive(Serialize, Deserialize)]
struct Booking {
    name: String,
    phonePrefix: String,
    phoneNumber: String,
    date: String,
    time: String,
}

//datafile
#[derive(Serialize, Deserialize)]
struct DataFile{
    dishes: Vec<MenuItem>,
    bookings: Vec<Booking>,
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

// GET all dishes
#[get("/dishes")]
async fn get_dishes(data: web::Data<AppState>) -> impl Responder {
    let data_file = read_data(&data.data_file.lock().unwrap());

    println!("Received GET request: /dishes"); //for basic log

    HttpResponse::Ok().json(data_file.dishes)
}

// GET all bookings
#[get("/bookings")]
async fn get_bookings(data: web::Data<AppState>) -> impl Responder {
    let data_file = read_data(&data.data_file.lock().unwrap());

    println!("Received GET request: /bookings"); //for basic log

    HttpResponse::Ok().json(data_file.bookings)
}

// POST a new booking
#[post("/bookings")]
async fn post_booking(data: web::Data<AppState>, new_booking: web::Json<Booking>) -> impl Responder {
    let mut data_file = read_data(&data.data_file.lock().unwrap());

    println!("Received POST request: /bookings"); //for basic log

    //add the new booking to the existing list
    data_file.bookings.push(new_booking.into_inner());

    //save the updated data back to the JSON file
    write_data(&data.data_file.lock().unwrap(), &data_file);

    HttpResponse::Ok().body("Booking successfully added")
}

// PUT to update a booking
#[put("/bookings/{phoneNumber}")]
async fn put_booking(
    data: web::Data<AppState>,
    phone_number: web::Path<String>,
    updated_booking: web::Json<Booking>,
) -> impl Responder {
    let mut data_file = read_data(&data.data_file.lock().unwrap());

    println!("Received PUT request: /bookings/{phone_number}"); //for basic log

    //find the booking by phoneNumber
    if let Some(booking) = data_file
        .bookings
        .iter_mut()
        .find(|b| b.phoneNumber == *phone_number)
    {
        //update the booking details
        *booking = updated_booking.into_inner();
        
        //save the updated data back to the JSON file
        write_data(&data.data_file.lock().unwrap(), &data_file);
        HttpResponse::Ok().body("Booking successfully updated")
    } else {
        HttpResponse::NotFound().body("Booking not found")
    }
}

// DELETE to remove a booking
#[delete("/bookings/{phoneNumber}")]
async fn delete_booking(data: web::Data<AppState>, phone_number: web::Path<String>) -> impl Responder {
    let mut data_file = read_data(&data.data_file.lock().unwrap());

    println!("Received DELETE request: /bookings/{phone_number}"); //for basic log

    //find the index of the booking by phoneNumber
    if let Some(index) = data_file
        .bookings
        .iter()
        .position(|b| b.phoneNumber == *phone_number)
    {
        //remove the booking at the found index
        data_file.bookings.remove(index);
        
        //save the updated data back to the JSON file
        write_data(&data.data_file.lock().unwrap(), &data_file);
        HttpResponse::Ok().body("Booking successfully deleted")
    } else {
        HttpResponse::NotFound().body("Booking not found")
    }
}

//-------------------------------------------------------- main -----------------------------------------------------------------

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //filepath of the JSON "database"
    let data_file = "data/data2.json".to_string();

    println!("Starting server on http://127.0.0.1:8080"); //for basic log

    //HTTP server strating
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                data_file: Mutex::new(data_file.clone()),
            }))
            .service(get_dishes)    //GET /dishes
            .service(get_bookings)  //GET /bookings
            .service(post_booking)  //POST /bookings
            .service(put_booking)   // PUT /bookings/{phoneNumber}
            .service(delete_booking) // DELETE /bookings/{phoneNumber}
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
