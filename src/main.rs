use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(0));
    let db = warp::any().map(move || Arc::clone(&db));
    let routes1 = warp::path("counter1").and(db.clone()).and_then(counter);
    let routes2 = warp::path("counter2").and(db.clone()).and_then(counter);
    let routes = routes1.or(routes2);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn counter(db: Arc<Mutex<u8>>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut counter = db.lock().await;
    *counter += 1;
    Ok(counter.to_string())
}

/*
use warp::{Filter};
use tokio::time::{delay_for, Duration};

#[tokio::main]
async fn main() {
    let delay = warp::path("delay").and_then(delay_handler).map(|input: String| {
        input.to_uppercase()
    });
    let hello = warp::path("hello").map(|| {
        "Hello you"
    });

    let routes = delay.or(hello);
    warp::serve(routes).run(([192, 168, 0, 10], 8080)).await;
}

async fn delay_handler() -> Result<String, warp::Rejection> {
    delay_for(Duration::from_secs(3)).await;
    Ok(String::from("10 seconds has passed!!"))
}*/

/*
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;
use warp::http::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    username: String,
    password: String,
}

#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(HashMap::<String, User>::new()));
    let db = warp::any().map(move || Arc::clone(&db));

    //let routes = warp::path("counter").and(db).and_then(counter);

    //warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn register(new_user: User, db: Arc<Mutex<HashMap<String, User>>>) -> Result<impl warp::Reply, warp::Rejection>{
    let mut users = db.lock().await;
    if users.contains_key(&new_user.username) {
        return Ok(StatusCode::BAD_REQUEST)
    }
    users.insert(new_user.username.clone(), new_user);
    Ok(StatusCode::CREATED)
}

async fn login(credentials: User, db: Arc<Mutex<HashMap<String, User>>>) -> Result<impl warp::Reply, warp::Rejection> {
    let users = db.lock().await;
    match
    Ok()
}
 */