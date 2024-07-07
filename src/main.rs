use actix_web::{App, HttpResponse, HttpServer, Responder, Web};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
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
    proposals: Mutex<Vec<Proposal>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = Web::Data::new(AppState {
        proposals: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/proposals", Web::post().to(create_proposal))
            .route("/proposals", Web::get().to(get_proposals))
            .route("/approve/{id}", Web::post().to(approve_proposal))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn create_proposal(item: Web::Json<Proposal>, data: Web::Data<AppState>) -> impl Responder {
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

async fn get_proposals(data: Web::Data<AppState>) -> impl Responder {
    let proposals = data.proposals.lock().unwrap();
    HttpResponse::Ok().json(proposals.clone())
}

async fn approve_proposal(path: Web::Path<Uuid>, data: Web::Data<AppState>) -> impl Responder {
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
