# Strapi application


## Setup
```
strapi new strapi
cd strapi
strapi install graphql
```

## Run
```
npm run develop
```

## Query sample
```
query {
  posts{slug, title, created_at, tags{name}}
}
```

```
query {
  tags{name, posts{slug, title}}
}
```
