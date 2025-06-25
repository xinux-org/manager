use std::io::Write;

use flake_info::data::{import::Package, Derivation, Export};
use poem::{listener::TcpListener, Result, Route, Server};
use poem_openapi::{
    param::Query, payload::PlainText, types::ToJSON, ApiResponse, OpenApi, OpenApiService,
};

struct Api;

#[derive(ApiResponse, Debug)]
enum FetchResponse {
    #[oai(status = 200)]
    Ok(PlainText<String>),

    #[oai(status = 500)]
    Error(PlainText<String>),
}

#[derive(ApiResponse, Debug)]
enum ProcessResponse {
    #[oai(status = 200)]
    Ok(PlainText<String>),

    #[oai(status = 500)]
    Error(PlainText<String>),
}

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {name}!")),
            None => PlainText("hello!".to_string()),
        }
    }

    #[oai(path = "/process", method = "post")]
    async fn process(&self) -> Result<ProcessResponse> {
        let f = std::fs::read_to_string("./temp/nix.json")
            .map_err(|_| ProcessResponse::Error(PlainText("file read error".to_string())))?;

        let r: Vec<flake_info::data::Export> = serde_json::from_str(&f).map_err(|e| {
            println!("{:?}", e);
            ProcessResponse::Error(PlainText("json error".to_string()))
        })?;

        println!("{}", r.len());

        Ok(ProcessResponse::Ok(PlainText("ok".to_string())))
    }

    #[oai(path = "/fetch", method = "post")]
    async fn fetch(&self) -> Result<FetchResponse> {
        let res = flake_info::data::Source::nixpkgs("24.11".to_string())
            .await
            .map_err(|err| FetchResponse::Error(PlainText(err.to_string())))?;
        println!("{:?}", res);

        let i = flake_info::process_nixpkgs(
            &flake_info::data::Source::Nixpkgs(res),
            &flake_info::data::import::Kind::Package,
        )
        .map_err(|err| {
            println!("{:?}", err);
            FetchResponse::Error(PlainText("".to_string()))
        })?;

        i.iter().for_each(|Export { flake, item }| {
            match flake {
                Some(flake) => {}
                _ => (),
            };

            match item {
                Derivation::App {
                    app_attr_name,
                    app_platforms,
                    app_type,
                    app_bin,
                } => {}
                Derivation::Package {
                    package_attr_name,
                    package_attr_set,
                    package_pname,
                    package_pversion,
                    package_platforms,
                    package_outputs,
                    package_default_output,
                    package_programs,
                    package_license,
                    package_license_set,
                    package_maintainers,
                    package_maintainers_set,
                    package_description,
                    package_longDescription,
                    package_hydra,
                    package_system,
                    package_homepage,
                    package_position,
                } => {
                    println!("{:?}", package_homepage);
                }
                Derivation::Option {
                    option_source,
                    option_name,
                    option_description,
                    option_type,
                    option_default,
                    option_example,
                    option_flake,
                } => {}
            }
        });

        let mut f = std::fs::File::create("temp/nix.json").map_err(|e| {
            eprintln!("{:?}", e);
            FetchResponse::Error(PlainText("file open error".to_string()))
        })?;

        let j = serde_json::to_string(&i)
            .map_err(|_| FetchResponse::Error(PlainText("serialize error".to_string())))?;
        let _ = f.write_all(j.as_bytes());

        Ok(FetchResponse::Ok(PlainText("".to_string())))
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(Route::new().nest("/api", api_service).nest("/", ui))
        .await
}
