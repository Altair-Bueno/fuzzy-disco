# Creating a dev enviroment

## Creating a MongoDB docker container

```bash
docker run -dp "27017:27017"  mongo

# Set this enviroment variables for the rust client
MONGO_PASSWORD=""
MONGO_USERNAME=""
```


You can connect using Mongo compass. Use this URL `mongodb://127.0.0.1:27017`

## Creating a redis docker container

```bash
docker run -dp "6379:6379" redis
```

redis ready on `127.0.0.1:6379`