use std::net::{SocketAddr, ToSocketAddrs};

use actix_web::{web, App, HttpServer};
use clap::builder::TypedValueParser;
use clap::Parser;
use consulrs::client::{ConsulClient, ConsulClientSettingsBuilder};
use consulrs::kv;
use state::AppState;
use std::convert::TryInto;

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
        kv::read(&$client, $key, None)
            .await
            .unwrap()
            .response
            .pop()
            .unwrap()
            .value
            .unwrap()
            .try_into()
            .unwrap()
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = ServiceArgs::parse();

    let client = {
        let mut builder = ConsulClientSettingsBuilder::default();
        let config = match args.consul_addr {
            Some(addr) => {
                let consul_http_addr = format!("https://{}", addr);
                builder.address(consul_http_addr)
            }
            None => &mut builder,
        }
        .build()
        .unwrap();
        ConsulClient::new(config).unwrap()
    };

    let conn = {
        let user: String = get_consul_kv!(client, "service_auth/db/user");
        let password: String = get_consul_kv!(client, "service_auth/db/password");
        let host: String = get_consul_kv!(client, "service_auth/db/host");
        let port: String = get_consul_kv!(client, "service_auth/db/port");
        let database: String = get_consul_kv!(client, "service_auth/db/database");
        sea_orm::Database::connect(format!(
            "postgresql://{user}:{password}@{host}:{port}/{database}"
        ))
        .await
        .unwrap()
    };

    let state = {
        let location: String = get_consul_kv!(client, "service_auth/location");
        AppState { conn, location }
    };

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
