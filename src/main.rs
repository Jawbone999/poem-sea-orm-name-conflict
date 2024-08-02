mod entities;

use entities::{team, user};
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{payload::Json, OpenApi, OpenApiService};

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/user", method = "get")]
    async fn get_user(&self) -> Json<user::Model> {
        Json(user::Model {
            id: 1,
            name: "Alice".to_string(),
        })
    }

    #[oai(path = "/team", method = "get")]
    async fn get_team(&self) -> Json<team::Model> {
        Json(team::Model {
            id: 1,
            name: "Team A".to_string(),
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api", api_service).nest("/", ui);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
