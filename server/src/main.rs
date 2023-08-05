mod model;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use mongodb::{bson::doc, Client, Collection};
use rand::Rng;
use std::env;
use server::Alphabet;

use model::{Links, ReqBody};

const DB_NAME: &str = "shortifyDB";
const COL_NAME: &str = "Links";

fn get_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphabet)
        .take(length)
        .map(char::from)
        .collect()
}

#[post("/")]
async fn add_link(client: web::Data<Client>, form: web::Json<ReqBody>) -> impl Responder {
    let collection: Collection<Links> =
        client.database(DB_NAME).collection(COL_NAME);
    let random_string = get_random_string(4);

    let new_doc = Links {
        id: None,
        url: form.url.to_owned(),
        label: random_string.clone(),
    };

    let result = collection.insert_one(new_doc, None).await;

    match result {
        Ok(_) => {
            HttpResponse::Ok()
                .body(format!("http://localhost:8000/{}", random_string))
        }
        Err(err) => HttpResponse::InternalServerError()
            .body(err.to_string())
    }
}

#[get("/")]
async fn testing() -> impl Responder {
    HttpResponse::Ok().body("API is up & running...")
}

// get link with supplied id in url
#[get("/{label_id}")]
async fn get_link(
    client: web::Data<Client>,
    label_id: web::Path<String>,
) -> impl Responder {
    let label_id = label_id.into_inner();
    let collection: Collection<Links> =
        client.database(DB_NAME).collection(COL_NAME);

    match collection.find_one(doc! { "label": &label_id }, None).await {
        Ok(Some(link)) => web::Redirect::to(link.url),
        Ok(None) => web::Redirect::to("/"),
        Err(_) => web::Redirect::to("/"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    // env_logger::init();

    let mongo_uri = match env::var("MONGOURI") {
        Ok(v) => v.to_string(),
        Err(_) => "mongodb://localhost:27017".to_string(),
    };

    let client = Client::with_uri_str(mongo_uri)
        .await
        .expect("[ERR]: Failed to connect db!");

    HttpServer::new(move || {
        App::new()
            // .wrap(middleware::Logger::default())
            .app_data(web::Data::new(client.clone()))
            .service(add_link)
            .service(get_link)
            .service(testing)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
