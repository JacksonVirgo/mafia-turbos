use mafia_turbos::app::{database::database_init, server::start_server};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_pool = match database_init().await {
        Ok(pool) => pool,
        Err(err) => {
            println!("{:?}", err);
            return Ok(());
        }
    };

    start_server(db_pool).await?;

    Ok(())
}
