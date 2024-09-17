use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormData {
    pub name: String,
    pub email: String,
    pub message: String,
}

#[derive(Deserialize, Debug, Clone)]
struct ResponseData {
    pub json: Option<FormData>, // Поле для данных, которые мы отправляем
}

pub async fn httpbin_org_request(form_data: &FormData) -> Result<FormData, String> {
    let body = serde_json::to_string(&form_data).map_err(|e| e.to_string())?;

    let response = Request::post("https://httpbin.org/post")
        .header("Content-Type", "application/json")
        .body(body)
        .map_err(|e| format!("Ошибка построения запроса: {:?}", e))?
        .send()
        .await
        .map_err(|e| format!("Ошибка запроса: {:?}", e))?;

    response
        .json::<ResponseData>()
        .await
        .map_err(|e| format!("Ошибка парсинга ответа: {:?}", e))?
        .json
        .ok_or("Данные не найдены в поле 'json'.".to_string())
}
