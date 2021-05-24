//! # Testing tools
//!
//! `del_test_base` - Remove test databases.
//!

use crate::store::MONGODB_CLIENT_STORE;

/// Remove test databases
/// Hint: See the tests in the `test-drive` section for an example.
pub fn del_test_db(
    project_name: &str,
    unique_project_key: &str,
    models: &Vec<crate::models::Meta>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Name of the technical database for testing
    let db_mango_tech: String = format!("mango_tech__{}__{}", project_name, unique_project_key);
    //
    let client_store = MONGODB_CLIENT_STORE.read()?;
    // Removing databases
    for meta in models {
        let client: &mongodb::sync::Client =
            client_store.get(meta.db_client_name.as_str()).unwrap();
        let database_names: Vec<String> = client.list_database_names(None, None)?;
        //
        if database_names.contains(&db_mango_tech) {
            client.database(db_mango_tech.as_str()).drop(None)?;
        }
        if database_names.contains(&meta.database_name) {
            client.database(meta.database_name.as_str()).drop(None)?;
        }
    }
    //
    Ok(())
}
