use std::net::{SocketAddr, ToSocketAddrs};

use actix_web::{web, App, HttpServer};
use clap::builder::TypedValueParser;
use clap::Parser;
use consul::kv::KV;
use consul::{Client, Config};
use state::AppState;

mod controller;
mod domain;
mod infra;
mod state;

#[derive(Clone)]
pub struct CustomSocketAddrParser;

impl TypedValueParser for CustomSocketAddrParser {
    type Value = SocketAddr;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let addr = value.to_str().unwrap().to_socket_addrs()?.nth(0).unwrap();
        Ok(addr)
    }
}

#[derive(Parser)]
#[clap(
    name = "Service auth application",
    author = "s-horii",
    version = "0.1.0",
    about = "Run application which provides authenticate service"
)]
struct ServiceArgs {
    #[clap(short, long, value_parser=CustomSocketAddrParser)]
    consul_addr: Option<SocketAddr>,

    #[clap(short, long, value_parser=CustomSocketAddrParser)]
    bind: SocketAddr,
}

macro_rules! get_consul_kv {
    ($client: expr, $key: expr) => {
        $client.get($key, None).unwrap().0.unwrap().Value
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = ServiceArgs::parse();

    let client = {
        let config = match args.consul_addr {
            Some(addr) => {
                Config::new_from_consul_host(&addr.ip().to_string(), Some(addr.port()), None)
                    .unwrap()
            }
            None => Config::new().unwrap(),
        };
        Client::new(config)
    };

    let conn = {
        let user = get_consul_kv!(client, "service_auth/db/user");
        let password = get_consul_kv!(client, "service_auth/db/password");
        let host = get_consul_kv!(client, "service_auth/db/host");
        let port = get_consul_kv!(client, "service_auth/db/port");
        let database = get_consul_kv!(client, "service_auth/db/host");
        sea_orm::Database::connect(format!(
            "postgresql://{user}:{password}@{host}:{port}/{database}"
        ))
        .await
        .unwrap()
    };

    let state = AppState { conn };

    HttpServer::new(move || {
        App::new().service(
            web::scope("/user")
                .app_data(web::Data::new(state.clone()))
                .configure(controller::init_routes),
        )
    })
    .bind(args.bind)?
    .run()
    .await
}
