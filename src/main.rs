#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

mod error;
#[cfg(test)] mod tests;
mod utils;

use axum::{
  extract::Query,
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
  Json, Router,
};
use error::MyError;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, info};

async fn hello_world() -> &'static str { "Hello, world!" }

async fn error_handler() -> impl IntoResponse {
  (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
}

// #[axum::debug_handler]
async fn paginate_names(
  Query(params): Query<PaginationQ>,
  Json(names): Json<Vec<String>>,
) -> impl IntoResponse {
  let offset = params.offset.unwrap_or(0);
  // let split = params.split.unwrap_or(names.len());
  let limit = params.limit.unwrap_or(names.len());

  let end = std::cmp::min(offset + limit, names.len());
  debug!("names: {:?}", &names);
  debug!("no split");

  let names = names.into_iter().skip(offset).take(limit).collect::<Vec<_>>();
  match params.split {
    Some(split) => {
      let paginated_names = names.chunks(split).map(|c| c.to_vec()).collect::<Vec<Vec<String>>>();
      Json(json!(paginated_names))
    },
    None => Json(json!(names)),
  }
}

#[derive(Deserialize, Debug)]
struct PaginationQ {
  split:  Option<usize>,
  offset: Option<usize>,
  limit:  Option<usize>,
}

#[derive(Serialize)]
struct PaginatedResponse<T> {
  data: Vec<T>,
}

#[shuttle_runtime::main]
async fn main(
  #[shuttle_secrets::Secrets] secret_store: shuttle_secrets::SecretStore,
) -> shuttle_axum::ShuttleAxum {
  utils::setup(&secret_store).unwrap();

  info!("hello thor");

  let router = Router::new()
    .route("/", axum::routing::get(hello_world))
    .route("/5", post(paginate_names))
    .route("/-1/error", get(error_handler))
    .route("/-1/health", get(|| async { StatusCode::OK }));

  Ok(router.into())
}
