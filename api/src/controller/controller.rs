use crate::controller::model::{LoginRequest, RegisterRequest};
use crate::domain::{IUserRepository, User};
use crate::infra::repository::UserRepository;
use crate::state::AppState;
use actix_web::{post, web, HttpResponse, Responder};

#[post("/login")]
async fn login(data: web::Data<AppState>, request: web::Json<LoginRequest>) -> impl Responder {
    let req = request.into_inner();
    let state = data.into_inner().as_ref().clone();
    let repository = UserRepository::new(state.conn);
    let user = repository
        .find_by_credential(req.username, req.password)
        .await;
    match user {
        Some(u) => HttpResponse::TemporaryRedirect()
            .append_header(("location", state.location))
            .append_header(("Set-Cookie", u.jwt()))
            .finish(),
        None => HttpResponse::Unauthorized().finish(),
    }
}

#[post("/register")]
async fn register(
    data: web::Data<AppState>,
    request: web::Json<RegisterRequest>,
) -> impl Responder {
    let req = request.into_inner();
    let state = data.into_inner().as_ref().clone();
    let repository = UserRepository::new(state.conn);
    let new_user = User::new(req.username, req.password);
    let user = repository.create(new_user).await;
    match user {
        Ok(u) => HttpResponse::TemporaryRedirect()
            .append_header(("location", state.location))
            .append_header(("Set-Cookie", u.jwt()))
            .finish(),
        Err(_) => HttpResponse::UnprocessableEntity().finish(),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(register);
}
