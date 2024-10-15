# Simple Rust REST API

A simple REST API built with Rust. It uses **Actix Web** to create an HTTP server and use a JSON file as a database. The API allows retrieving information about restaurants and songs. It was developed as part of a cross-platform development project for the INFT2508 course at NTNU.

## Technologies Used

- **Rust**
- **Actix Web** - Web framework for building the REST API.
- **Serde** - For serializing and deserializing JSON data.
- **JSON** - Used as the database to store information about restaurants and songs.

## Data Structure

The JSON file stores two sets of data:
- **Restaurants**: Each restaurant has a name, an address, and a menu.
- **Songs**: Each song has a name and an associated artist.

### Example of the JSON structure:

```json
{
    "restaurants": [
        {
            "name": "ChineseRestaurant",
            "address": "Prinsengata 1, 7010 Trondheim",
            "menu": [
                {"dish": "Pizza", "price": 50},
                {"dish": "Burger", "price": 25}
            ]
        },
        {
            "name": "MexicanRestaurant",
            "address": "Kongensgata 10, 7020 Trondheim",
            "menu": [
                {"dish": "French Fries", "price": 250},
                {"dish": "Onion Rings", "price": 150}
            ]
        }
    ],
    "songs": [
        {
            "name": "song1",
            "artist": "Nils"
        },
        {
            "name": "song2",
            "artist": "Jean"
        }
    ]
}
```

## Endpoints

The API endpoints are : 
- ```GET /restaurants``` : Retrieve all restaurants.
- ```GET /restaurants/{name}/menu``` : Retrieve the menu of a specific restaurant.
- ```GET /songs``` : Retrieve all songs.
- ```GET /songs/{name}``` : Retrieve information about a specific song

### Example :
Request : 
```
curl http://127.0.0.1:8080/songs/song1
```
Response : 
```json
{
    "name": "song1",
    "artist": "Nils"
}
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