use mongodb::bson::doc;
use mongodb::Client as MongoClient;
use mongodb::Database as MongoDatabase;

pub async fn init_mongo_db() -> mongodb::error::Result<MongoDatabase> {
    let url = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://127.0.0.1/".to_string());
    let options = mongodb::options::ClientOptions::parse(&url).await?;
    #[cfg(debug_assertions)]
    println!("[MONGO]: Expecting mongo on {}", url);
    let db = MongoClient::with_options(options).map(|x| x.database("fuzzy-disco"))?;
    // FIXME rust driver version 2.0 should allow index creation more easily
    let index_response = db
        .run_command(
            doc! {
                "createIndexes": "Users",
                "indexes": [
                    {
                        "key": { "alias": 1 },
                        "name": "alias",
                        "unique": true
                    },
                    {
                        "key": { "email": 1 },
                        "name": "email",
                        "unique": false
                    }
                ]
            },
            None,
        )
        .await?;
    #[cfg(debug_assertions)]
    println!("[MONGO]: Index creation response {:?}", index_response);
    let index_response = db
        .run_command(
            doc! {
                "createIndexes": "Sessions",
                "indexes": [
                    {
                        "key": { "sub": 1 },
                        "name": "sub",
                        "unique": false
                    },
                ]
            },
            None,
        )
        .await?;

    #[cfg(debug_assertions)]
    println!("[MONGO]: Index creation response {:?}", index_response);

    let index_response = db
        .run_command(
            doc! {
                "createIndexes": "Media",
                "indexes": [
                    {
                        "key": { "status": 1 },
                        "name": "status",
                        "unique": false
                    },
                ]
            },
            None,
        )
        .await?;

    #[cfg(debug_assertions)]
    println!("[MONGO]: Index creation response {:?}", index_response);

    Ok(db)
}
