#[tokio::main]
async fn main() -> anyhow::Result<()> {
    young_ethereum::run().await
}
