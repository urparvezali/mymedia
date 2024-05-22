use mongodb::{options::ClientOptions, Client};


pub async fn dbconnection() -> Result<Client, mongodb::error::Error>{
	let opt = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
	Client::with_options(opt)
}
    
