// use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

mod zkp; // this module contains ZKP-related functions

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Proposal {
    id: Uuid,
    title: String,
    description: String,
    approved: bool,
}

#[derive(Debug, Clone)]
struct AppState {
    proposals: Arc<Mutex<Vec<Proposal>>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        proposals: Arc::new(Mutex::new(vec![])),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/proposals", web::post().to(create_proposal))
            .route("/proposals", web::get().to(get_proposals))
            .route("/approve/{id}", web::post().to(approve_proposal))
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    // println!("ok")
}

async fn create_proposal(item: web::Json<Proposal>, data: web::Data<AppState>) -> impl Responder {
    let mut proposals = data.proposals.lock().unwrap();
    let new_proposal = Proposal {
        id: Uuid::new_v4(),
        title: item.title.clone(),
        description: item.description.clone(),
        approved: false,
    };
    proposals.push(new_proposal.clone());
    HttpResponse::Created().json(new_proposal)
}

async fn get_proposals(data: web::Data<AppState>) -> impl Responder {
    let proposals = data.proposals.lock().unwrap();
    HttpResponse::Ok().json(proposals.clone())
}

async fn approve_proposal(path: web::Path<Uuid>, data: web::Data<AppState>) -> impl Responder {
    let mut proposals = data.proposals.lock().unwrap();
    let id = path.into_inner();
    if let Some(proposal) = proposals.iter_mut().find(|p| p.id == id) {
        // Use ZKP to verify approval
        if zkp::verify_proposal_approval(&proposal) {
            proposal.approved = true;
            HttpResponse::Ok().json(proposal)
        } else {
            HttpResponse::Unauthorized().body("ZKP verification failed")
        }
    } else {
        HttpResponse::NotFound().body("Proposal not found")
    }
}
