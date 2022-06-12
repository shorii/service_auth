use crate::controller::model::{
    LoginRequest, LoginResponse, RegisterRequest, RegisterResponse, UserModel,
};
use crate::domain::{IUserRepository, User};
use crate::infra::repository::UserRepository;
use crate::state::AppState;
use actix_web::{post, web, HttpResponse, Responder};

#[post("/login")]
async fn login(data: web::Data<AppState>, request: web::Json<LoginRequest>) -> impl Responder {
    let req = request.into_inner();
    let state = data.into_inner();
    let repository = UserRepository::new(state.as_ref().clone());
    let user = repository
        .find_by_credential(req.username, req.password)
        .await;
    match user {
        Some(u) => HttpResponse::Ok().json(LoginResponse {
            user: UserModel {
                username: u.username,
            },
        }),
        None => HttpResponse::Unauthorized().finish(),
    }
}

#[post("/register")]
async fn register(
    data: web::Data<AppState>,
    request: web::Json<RegisterRequest>,
) -> impl Responder {
    let req = request.into_inner();
    let state = data.into_inner();
    let repository = UserRepository::new(state.as_ref().clone());
    let new_user = User::new(req.username, req.password);
    let user = repository.create(new_user).await;
    match user {
        Ok(u) => HttpResponse::Ok()
            .append_header(("Set-Cookie", u.jwt()))
            .json(RegisterResponse {
                user: UserModel {
                    username: u.username,
                },
            }),
        Err(_) => HttpResponse::UnprocessableEntity().finish(),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(register);
}
