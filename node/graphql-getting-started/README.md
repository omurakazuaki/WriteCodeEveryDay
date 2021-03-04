# GraphQL getting started

## Run server
```sh
ts-node index.ts
```

## Sample request

### Query

**graphql**
```sh
curl -H "Content-Type:application/graphql" -X POST http://localhost:3000/blog -d '
  query {
    users {
      name
      email
      posts {
        title
      }
    }
  }'
```

**json**
```sh
curl -H "Content-Type:application/json" -X POST http://localhost:3000/blog -d '
  { "query" : "query { user(id: 1){name} }" }'
```

### Mutation

```sh
curl -H "Content-Type:application/graphql" -X POST http://localhost:3000/blog -d '
  mutation {
    registerPost(authorId: 2, title:"test") {
      id
      title
      link
    }
  }
'
```
