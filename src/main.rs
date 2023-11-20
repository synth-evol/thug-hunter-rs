use axum::{
    extract::Path,
    routing::get,
    Router,
};
use std::env;
use rosu::Osu;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let api_key = args[1].clone();
    
    let app = Router::new().route("/:user_name", get(
        move |path| global_score(path, api_key)));
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn global_score(Path(user_name): Path<String>, api_key:String) -> String {
    let osu = Osu::new(api_key);

    let user_opt = osu.user(user_name).await;

    let user = user_opt.unwrap().unwrap();
    let score = user.pp_rank.to_string();
    return score;
}