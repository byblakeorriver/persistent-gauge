#![deny(
  unstable_features,
  unused_must_use,
  unused_mut,
  unused_imports,
  unused_import_braces
)]

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  persistent_gauge::start().await
}
