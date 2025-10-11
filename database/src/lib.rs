use leptos::prelude::*;
use std::env::var;
use std::sync::OnceLock;
use supabase_rs::SupabaseClient;

static DB: OnceLock<SupabaseClient> = OnceLock::new();

pub fn database_init() -> Result<(), ServerFnError> {
    let supabase_url = var("SUPABASE_URL").map_err(|e| ServerFnError::new(e))?;
    let supabase_key = var("SUPABASE_KEY").map_err(|e| ServerFnError::new(e))?;

    let supabase_client = SupabaseClient::new(supabase_url, supabase_key)?;
    DB.set(supabase_client)
        .map_err(|e| ServerFnError::new(format!("Failed to set database client: {:?}", e)))?;
    Ok(())
}

pub fn get_database<'a>() -> Result<&'a SupabaseClient, ServerFnError> {
    DB.get()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))
}

pub async fn get_resolved_image(img_id: &String) -> Result<(String, String), ServerFnError> {
    let db = DB
        .get()
        .ok_or_else(|| ServerFnError::new("Database not initialized"))?;

    let image = &db
        .select("images")
        .eq("id", img_id)
        .limit(1)
        .execute()
        .await
        .map_err(|e| ServerFnError::new(e))?[0];

    Ok((
        format!(
            "{}/storage/v1/object/public/{}/{}",
            var("SUPABASE_URL")?,
            image["bucket"].as_str().unwrap(),
            image["name"].as_str().unwrap(),
        ),
        image["alt"].to_string(),
    ))
}
