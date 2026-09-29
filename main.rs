// Lab d'expérimentation API REST
// Test des principes de sécurité et d'allocation mémoire avec Rust

use actix_web::{web, App, HttpServer, Responder};

async fn health_check() -> impl Responder {
    "Statut de l'API REST : Fonctionnel et sécurisé."
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Démarrage du serveur local...");
    HttpServer::new(|| {
        App::new().route("/status", web::get().to(health_check))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
