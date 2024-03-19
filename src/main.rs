use actix_web::{web, App, HttpServer, HttpResponse, Responder};

type FligtPath = Vec<Vec<String>>;

async fn calculate_flights(data: web::Json<FligtPath>) -> impl Responder {
    let mut airports_start: Vec<String> = Vec::new();
    let mut airports_end: Vec<String> = Vec::new();
    let error_message = "Invalid Parmeters";
    for flight in data.iter() {
        if flight.len() < 2 {
           return HttpResponse::BadRequest().body(error_message);
        }
        airports_start.push(flight[0].clone());
        airports_end.push(flight[1].clone());
    }

    let airports_start_clone = airports_start.clone();

    airports_start.retain(|x| !airports_end.contains(x));
    airports_end.retain(|y| !airports_start_clone.contains(y));

    if airports_end.len() == 0 || airports_start.len() == 0 {
        return HttpResponse::BadRequest().body("Not Found Source and Destination by given data");
    }

    let result = vec![airports_start[0].clone(), airports_end[0].clone()];
    
    HttpResponse::Ok().json(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new().service(web::resource("/calculate").route(web::post().to(calculate_flights)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}