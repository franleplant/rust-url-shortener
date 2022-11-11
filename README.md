# rust-url-shortener
A fun learning exercise with Rust


## Run db

```bash

docker run --name url_shortener -e POSTGRES_PASSWORD=postgres -p 54321:5432  -d postgres:13.7

// try out the connection (requires local postgres installed)
psql -p 54321 postgres://postgres:postgres@localhost/postgres
```