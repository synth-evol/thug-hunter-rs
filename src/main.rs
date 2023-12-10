use axum::{
    extract::{Path, State},
    routing::get,
    Router,
    http::StatusCode,
    response::{IntoResponse, Response}
};
use std::{
    env,
    sync::Arc,
};
use rosu::Osu;

struct AppState{
    client: Osu,
}


enum ApiError {
    ErrorOsu(StatusCode, String),
    ErrorNotFound(StatusCode, String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let variant = match self {
            ApiError::ErrorOsu(code, body) => (code, body),
            ApiError::ErrorNotFound(code, body) => (code, body) 
        };    
    variant.into_response()
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let api_key = args[1].clone();
    let state = Arc::new(AppState{client: Osu::new(api_key)});

    let app = Router::new().route("/:user_name", get(global_score)).with_state(state);
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn global_score(Path(user_name): Path<String>, State(state): State<Arc<AppState>>) -> Result<String, ApiError> {

    let user_req = state.client.user(user_name).await;

    let user_opt = match user_req {
        Ok(x) => x,
        Err(e) => {
            let wrapper = ApiError::ErrorOsu(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()); 
            return Err(wrapper)
        }
    };

    let user = match user_opt {
        Some(x) => x,
        None => {
            let wrapper = ApiError::ErrorNotFound(StatusCode::NOT_FOUND, String::from("No user with that username found"));
            return Err(wrapper);
        }
    };

    let score = user.pp_rank.to_string();
    Ok(score)
}