use axum::{
    extract::{Path, State},
    routing::get,
    Router,
    http::StatusCode,
};
use std::env;
use rosu::Osu;

#[derive(Clone)]
struct AppState{
    key: String,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let api_key = args[1].clone();
    let state = AppState{key: api_key};

    let app = Router::new().route("/:user_name", get(global_score)).with_state(state);
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
async fn global_score(Path(user_name): Path<String>, State(state): State<AppState>) -> Result<String, StatusCode> {
    let osu = Osu::new(state.key);

    let user_req = osu.user(user_name).await;

    let user_opt = match user_req {
        Ok(x) => x,
        Err(e) =>  return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let user = match user_opt {
        Some(x) => x,
        None => return Err(StatusCode::NOT_FOUND),
    };

    let score = user.pp_rank.to_string();
    Ok(score)
}