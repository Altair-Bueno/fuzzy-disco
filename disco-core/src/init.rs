use mongodb::Client as MongoClient;
use redis::Client as RedisClient;

pub fn init_mongo_client() -> mongodb::error::Result<MongoClient> {
    let password = std::env::var("MONGO_PASSWORD").ok();
    let username = std::env::var("MONGO_USERNAME").ok();

    let mut options = mongodb::options::ClientOptions::default();
    let mut cretential = mongodb::options::Credential::default();
    cretential.username = username;
    cretential.password = password;
    options.credential = Some(cretential);

    MongoClient::with_options(options)
}

pub fn redis_client() -> redis::RedisResult<RedisClient> {
    todo!()
}
