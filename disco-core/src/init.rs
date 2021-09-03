use mongodb::Client as MongoClient;
use redis::Client as RedisClient;

pub async fn init_mongo_client() -> mongodb::error::Result<MongoClient> {
    let url = std::env::var("MONGODB_URI").unwrap_or("127.0.0.1".to_string());
    let options = mongodb::options::ClientOptions::parse(url).await?;
    MongoClient::with_options(options)
}

pub fn redis_client() -> redis::RedisResult<RedisClient> {
    todo!()
}
