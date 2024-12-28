use anyhow;
use dnd_5e_client::get_classes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Example usage of your crate
    let _ = get_classes().await?;
    Ok(())
}
