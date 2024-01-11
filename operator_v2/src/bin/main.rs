#[tokio::main]
async fn main() -> eyre::Result<()> {

    operator::subscribe_new_task().await?;

    Ok(())
}
