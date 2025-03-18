mod compress;
mod database;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    database::generate_database_tables()?;
    compress::compress()?;
    tauri_build::build();

    Ok(())
}
