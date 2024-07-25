#![allow(unused)]

use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};
use std::error::Error;
use tokio;

const DATABASE_NAME: &str = "passmanager";
const COLLECTION_NAME: &str = "passwords";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    let db = client.database(DATABASE_NAME);
    let collection = db.collection(COLLECTION_NAME);

    let login = "user@example.com";
    let password = "supersecurepassword";

    create_password(&collection, login, password).await?;

    let retrieved_password = get_password(&collection, login).await?;
    println!("Retrieved password: {}", retrieved_password);

    Ok(())
}

async fn create_password(collection: &Collection<Document>, login: &str, password: &str) -> Result<(), Box<dyn Error>> {
    let document = doc! {
        "login": login,
        "password": password
    };

    collection.insert_one(document).await?;

    Ok(())
}

async fn get_password(collection: &Collection<Document>, login: &str) -> Result<String, Box<dyn Error>> {
    let filter = doc! { "login": login };

    if let Some(result) = collection.find_one(filter).await? {
        if let Some(password) = result.get("password").and_then(|v| v.as_str()) {
            return Ok(password.to_string());
        }
    }

    Err("Password not found".into())
}

async fn remove_password(collection: &Collection<Document>, login: &str) -> Result<(), Box<dyn Error>> {
    let filter = doc! { "login": login };

    collection.delete_one(filter).await?;

    Ok(())
}