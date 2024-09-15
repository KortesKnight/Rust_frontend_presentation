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
    match serde_json::to_string(&form_data) {
        Ok(body) => {
            let response = Request::post("https://httpbin.org/post")
                .header("Content-Type", "application/json")
                .body(body)
                .unwrap()
                .send()
                .await;

            match response {
                Ok(resp) => match resp.json::<ResponseData>().await {
                    Ok(data) => {
                        if let Some(form) = data.json {
                            Ok(form)
                        } else {
                            Err("Данные не найдены в поле 'json'.".to_string())
                        }
                    }
                    Err(err) => Err(format!("Ошибка парсинга ответа: {:?}", err)),
                },
                Err(err) => Err(format!("Ошибка запроса: {:?}", err)),
            }
        }
        Err(err) => Err(format!("Ошибка сериализации данных формы: {:?}", err)),
    }
}
