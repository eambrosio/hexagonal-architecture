mod api;
mod domain;
mod repositories;

#[tokio::main]
async fn main() {
    api::serve("0.0.0.0:8888").await;
}
