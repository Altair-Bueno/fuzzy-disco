# Fuzzy-disco

A post sharing social network focused on simplicity. 

# Services
Fuzzy-disco is made up different components that work together to provide a
smooth experience:

- [`disco-core`](disco-core): The API backend and static server. Provides a 
reliable interface to interact with the database, as well as the static webpage.
It's written in Rust using [Rocket](https://rocket.rs), making the server fast
and reliable
- [`disco-vue`](disco-vue): The static websithe that provides a user-friendly
interface to the API, written in Vue.js
- [`mongodb`](https://www.mongodb.com/es): A Mongo Database provides fast and
reliable persistent data storage for fuzzy-disco
- [`redis`](https://redis.io): a redis instance speeds up database access on 
fuzzy-disco, making the API even faster

# Run on Docker
To deploy a private instance of fuzzy-disco, clone this repo and run 
`docker-compose up -d`. This will start the different services. After building
is complete, it should be available on `0.0.0.0:8000`

To bring down the service, just run `docker-compose down`