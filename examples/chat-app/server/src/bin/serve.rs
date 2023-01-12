use appwrite::prelude::*;
use actix_web::{HttpServer, App};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let endpoint = std::env::var("APPWRITE_ENDPOINT")?;
    let project_id = std::env::var("APPWRITE_PROJECT_ID")?;
    let api_key = std::env::var("APPWRITE_API_KEY")?;
    let client = AppWriteClient::builder(&endpoint, &project_id)
        .set_key(&api_key)?
        .build()?;
    let data = server::data::Data {
        client,
    };
    
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .configure(server::routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
