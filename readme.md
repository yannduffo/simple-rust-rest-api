# Simple Rust REST API 

Simple Rust REST API. Only 2 endpoints for now : GET all books and POST a new one. Use of a JSON file as a Database (in this example a book database).

Build with Artix-web.

## Examples : 

Request :
```
$ curl http://127.0.0.1:8080/books
```
Response form API :
```
[{"id":1,"title":"book1","author":"Steve"},{"id":2,"title":"book2","author":"Carol"}]
```

