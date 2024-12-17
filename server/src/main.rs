use poem::{listener::TcpListener, Result, Route, Server};
use poem_openapi::{param::Query, payload::PlainText, ApiResponse, OpenApi, OpenApiService};

struct Api;

#[derive(ApiResponse, Debug)]
enum FetchResponse {
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

    #[oai(path = "/fetch", method = "post")]
    async fn fetch(&self) -> Result<FetchResponse> {
        let res = flake_info::data::Source::nixpkgs("24.11".to_string())
            .await
            .map_err(|err| FetchResponse::Error(PlainText(err.to_string())))?;
        println!("{:?}", res);

        let i = flake_info::process_nixpkgs(
            &flake_info::data::Source::Nixpkgs(res),
            &flake_info::data::import::Kind::All,
        )
        .map_err(|err| {
            println!("{:?}", err);
            FetchResponse::Error(PlainText("".to_string()))
        });
        println!("{:?}", i);
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
