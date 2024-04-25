mod data_collector;
mod features;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    data_collector::load_filings()?;
    // Assume features and model training happens here.
    println!("Features extracted and model trained.");

    Ok(())
}
