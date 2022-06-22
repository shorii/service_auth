use crate::controller::model::{
    JwksResponse, LoginRequest, LoginResponse, RegisterRequest, RegisterResponse,
};
use crate::domain::{IUserRepository, User};
use crate::infra::repository::UserRepository;
use crate::state::AppState;
use actix_web::{cookie, get, post, web, HttpResponse, Responder};

#[post("/login")]
async fn login(data: web::Data<AppState>, request: web::Json<LoginRequest>) -> impl Responder {
    let req = request.into_inner();
    let state = data.into_inner().as_ref().clone();
    let repository = UserRepository::new(state.conn);
    let user = repository
        .find_by_credential(req.username, req.password)
        .await;
    match user {
        Some(u) => {
            let c = cookie::Cookie::build("cookie", u.jwt(state.secret_key))
                .same_site(cookie::SameSite::Lax)
                .path("/")
                .finish();
            HttpResponse::Ok()
                .append_header(("Set-Cookie", c.to_string()))
                .json(LoginResponse {
                    location: state.location,
                })
        }
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
        Ok(u) => {
            let c = cookie::Cookie::build("cookie", u.jwt(state.secret_key))
                .same_site(cookie::SameSite::Lax)
                .path("/")
                .finish();
            HttpResponse::Ok()
                .append_header(("Set-Cookie", c.to_string()))
                .json(RegisterResponse {
                    location: state.location,
                })
        }
        Err(_) => HttpResponse::UnprocessableEntity().finish(),
    }
}

#[get("/jwks")]
async fn jwks(data: web::Data<AppState>) -> impl Responder {
    let state = data.as_ref().clone();
    HttpResponse::Ok().json(JwksResponse {
        keys: vec![state.secret_key],
    })
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(register);
    cfg.service(jwks);
}
