Last update 07/11/2024 11:16 - Assignment6.2

# Simple Rust REST API

A simple REST API built with Rust. It uses **Actix Web** to create an HTTP server and use a JSON file as a database. The API allows retrieving information about restaurants and songs. It was developed as part of a cross-platform development project for the INFT2508 course at NTNU.

## Technologies Used

- **Rust**
- **Actix Web** - Web framework for building the REST API.
- **Serde** - For serializing and deserializing JSON data.
- **JSON** - Used as the database to store information about restaurants and songs.

## Data Structure

The JSON file stores two sets of data:
- **Dishes**: Each dish has a title, description, allergens, an image link, category, and price.
- **Bookings**: Each booking has a name, phone prefix, phone number, date, and time.

### Example of the JSON structure:

```json
{
    "dishes": [
        {
            "title": "Margherita Pizza",
            "description": "Tomato, Mozzarella, Basil, Olive Oil",
            "allergens": "Cheese, Olive oil",
            "image": "dishImages.pizza_margherita",
            "category": "pizza",
            "price": 10
        },
        {
            "title": "Four Cheese Pizza",
            "description": "Tomato, Mozzarella, Gorgonzola, Parmesan, Fontina, Provolone",
            "allergens": "Cheese",
            "image": "dishImages.pizza_four_cheese",
            "category": "pizza",
            "price": 13
        }
    ],
    "bookings": [
        {
            "name": "Olivier Michel",
            "phonePrefix": "+33",
            "phoneNumber": "123456789",
            "date": "12/12/2024",
            "time": "18h"
        },
        {
            "name": "Jean Mathieu",
            "phonePrefix": "+33",
            "phoneNumber": "123456789",
            "date": "13/12/2024",
            "time": "20h30"
        }
    ]
}
```

## Endpoints

The API endpoints are : 
- ```GET /dishes``` : Retrieve all available dishes.
- ```GET /bookings``` : Retrieve all bookings.
- ```POST /bookings``` : Create a new booking by submitting booking details in JSON format.
- ```PUT /bookings/{phone_number}``` : Modify the booking identified by the indicated phone number.
- ```DELETE /bookings/{phone_number}``` : Delete the booking identified by the indicated phone number.

### Examples :
#### GET the dishes
Request : 
```
curl http://127.0.0.1:8080/dishes
```
Response : 
```json
[
    {
        "title": "Margherita Pizza",
        "description": "Tomato, Mozzarella, Basil, Olive Oil",
        "allergens": "Cheese, Olive oil",
        "image": "dishImages.pizza_margherita",
        "category": "pizza",
        "price": 10
    },
    {
        "title": "Four Cheese Pizza",
        "description": "Tomato, Mozzarella, Gorgonzola, Parmesan, Fontina, Provolone",
        "allergens": "Cheese",
        "image": "dishImages.pizza_four_cheese",
        "category": "pizza",
        "price": 13
    }
]
```

#### GET the bookings
Request : 
```
curl http://127.0.0.1:8080/bookings
```
Response : 
```json
[
    {
        "name": "Olivier Michel",
        "phonePrefix": "+33",
        "phoneNumber": "123456789",
        "date": "12/12/2024",
        "time": "18h"
    },
    {
        "name": "Jean Mathieu",
        "phonePrefix": "+33",
        "phoneNumber": "123456789",
        "date": "13/12/2024",
        "time": "20h30"
    }
]
```


#### POST a booking
Request : 
```
curl -X POST http://127.0.0.1:8080/bookings \
-H "Content-Type: application/json" \
-d '{
  "name": "Alice Martin",
  "phonePrefix": "+33",
  "phoneNumber": "987654321",
  "date": "14/12/2024",
  "time": "19h30"
}'
```
Response from API (data file has been updated): 
```
Booking successfully added
```

#### PUT on a booking (modify a booking)
Request (change the name of the reservation (identified by phone_number)) : 
```
curl -X PUT http://127.0.0.1:8080/bookings/123456789 \
-H "Content-Type: application/json" \
-d '{
    "name": "Alice Martin",
    "phonePrefix": "+33",
    "phoneNumber": "123456789",
    "date": "15/12/2024",
    "time": "20h00"
}'
```
Response from API (data file has been updated): 
```
Booking successfully updated
```

#### DELETE a booking
Request (delete a reservation (identified by phone number)) : 
```
curl -X DELETE http://127.0.0.1:8080/bookings/123456789
```
Response from API (data file has been updated): 
```
Booking successfully deleted
```

## Running the app 

1. Ensure Rust and Cargo are installed on your machine.
2. Clone the project 
3. Run the following command to start the server: ```cargo run```

The local dev server will be available at ```http://127.0.0.1:8080```.


<br>
<br>
<br>
Created by Yann Duffo