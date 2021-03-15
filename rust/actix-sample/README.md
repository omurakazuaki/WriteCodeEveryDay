# ACTIX Web sample

## Run
```
cargo run
```

## Setup data

```
echo '
[
  {
    "id": 1,
    "title": "Tel",
    "description": "description",
    "status": "Open",
    "deadline": "2021-03-14"
  },
  {
    "id": 2,
    "title": "Programming",
    "description": "description",
    "status": "InProgress",
    "deadline": "2021-03-15"
  }
]
' > target/data.json
````

## API

### GET Todos
http://localhost:8080/todos

### GET Todo
http://localhost:8080/todo/1

### POST Todo
```
curl -X POST http://localhost:8080/todo -d '{
    "title": "meeting",
    "description": "description",
    "status": "Close",
    "deadline": "2021-03-12"
}'
```
