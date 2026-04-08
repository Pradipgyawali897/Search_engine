#[tokio::main]
async fn main() -> Result<(), ab_web::error::AppError> {
    ab_web::run().await
}
