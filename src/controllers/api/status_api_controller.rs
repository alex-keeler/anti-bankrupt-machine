#[get("/api/status")]
pub async fn report_status() -> String {
    String::from("OK")
}