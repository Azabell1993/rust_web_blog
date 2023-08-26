use dotenv;

#[async_std::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    let mut app = tide::new();
    app.at("/").get(|_| async move { Ok("지우의 블로그입니다") });
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error(transparent)]
    DbError(#[from] sqlx::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}