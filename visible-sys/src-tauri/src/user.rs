use serde::Serialize;

#[derive(Serialize)]
struct CommonResponse {
  res: u32,
  msg: String
}

#[tauri::command]
pub fn login(username: &str, password: &str) -> String {
    if username == "root" && password == "123456"{
        return serde_json::to_string(&CommonResponse{res: 200, msg: "Login success".to_string()}).unwrap_or_default();
    } else {
        return serde_json::to_string(&CommonResponse{res: 1, msg: "Login failed".to_string()}).unwrap_or_default();
    }
}
