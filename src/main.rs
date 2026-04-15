use oberon::cli::interface::Cli;
use oberon::core::logging;
use oberon::Result;

#[tokio::main]
async fn main() -> Result<()> {
    logging::init();
    Cli::run().await
}