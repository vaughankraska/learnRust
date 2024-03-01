use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

fn main() {
    let server = HttpServer::new(|| {
        App::new().route("/", web::get().to(get_index))
    });

    println!("Serving on localhost:3000");
    server.bind("127.0.0.1:3000")
        .expect("ERROR binding server to address")
        .run()
        .expect("ERROR running server");

}

fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
            <title>GCD Calculator</title>
            <form action="/gcd" method="post">
                <input type="text" name="n" />
                <input type="text" name="m" />
                <button type="submit">GO</button>
            </form>
            "#,
        )
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
//     left of at :: With this definition in place, we can write our handler function quite easily:
}