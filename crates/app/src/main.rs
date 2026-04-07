#[tokio::main]
async fn main() -> app::error::AppResult<()> {
    app::run().await
}
