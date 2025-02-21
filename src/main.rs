use adbc_core::{Driver, Statement}; 
use adbc_core::{Database, Connection};
use adbc_dummy::DummyDriver;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the driver
    let mut driver = DummyDriver::default();
    
    // Create database connection
    let mut db = driver.new_database()?;
    
    // Create connection
    let mut conn = db.new_connection()?;
    
    // Create and execute a statement
    let mut stmt = conn.new_statement()?;
    stmt.set_sql_query("SELECT 1")?;
    let results = stmt.execute()?;
    
    // Print results
    for batch in results {
        println!("Batch: {:?}", batch?);
    }
    
    Ok(())
}
