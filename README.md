# Matekasse

This project aims to stay compatiable with
[Mete](https://github.com/chaosdorf/mete) via the  [Space-Market
API](https://space-market.github.io/API/swagger-ui).

The project name is not final. :)

## How to build/run

Simply start the backend with this command 
```sh
cd server
cargo run
```

To build and serve the frontend use [trunk](https://github.com/thedodd/trunk)
with this command. Thrunk will act as a reverse proxy for the backend.

```sh
cd web
thrunk serve
```
