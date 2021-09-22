# Fuzzy-disco
A post sharing social network focused on simplicity. 

# Services
Fuzzy-disco is made up different components that work together to provide a
smooth experience:

- [`disco-core`](disco-core): The API backend and static server. Provides a 
reliable interface to interact with the database, as well as the static webpage.
It's written in Rust using [Rocket](https://rocket.rs), making the server fast
and reliable
- [`disco-vue`](disco-vue): The static website that provides a user-friendly
interface to the API, written in Vue.js
- [`mongodb`](https://www.mongodb.com): A Mongo Database provides fast and
reliable persistent data storage for fuzzy-disco
- [`redis`](https://redis.io): A redis instance caches the most database 
expensive operations, such as regex search

# API
Documentation for the API can be found on 
[Postman](https://www.postman.com/Altair-Bueno/workspace/fuzzy-disco/overview)


# Run on Docker
To deploy a private instance of fuzzy-disco, clone this repo and run 
`docker-compose up -d`. This will start the different services. After deployment
is complete, it should be available on `0.0.0.0:8000`. To bring down the 
service, just run `docker-compose down`

If you want to build fuzzy-disco from scratch instead of using the precompiled
image, clone this repo and run
`docker-compose -f docker-compose-from-source.yaml up`. This will take a lot of
time (20m~)
