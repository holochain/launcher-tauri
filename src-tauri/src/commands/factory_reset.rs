#[tauri::command]
pub async fn factory_reset() -> Result<(), String> {
  crate::factory_reset::factory_reset().await
}
