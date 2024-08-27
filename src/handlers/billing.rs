use std::sync::Arc;

use axum::{
    extract::{Json as ReqJson, Path, Request, State}, response::{IntoResponse, Json}
};
use serde_json::Value;

use crate::{api::external::billing::request_payload, config::application::Application};

/// Обартные запрос о статусах платежей.
pub async fn callbacks_notify_payment(
    ReqJson(request): ReqJson<Value>,
) -> impl IntoResponse {
    
    // TODO: implement
    // -> Пришел запрос на оплату
    // -> Сохрарнили данные заказа
    // -> Отправили со своим ID
    // -> После ответа сложили их ID
    // -> Обновили временно количество билетов для события
    // -> Ждем колбэк от них
    // -> После получения обновляем статус заказа
    // -> высылаем данные на почту
    // -> обновляем количество билетов для меры

    println!("1232132");
    let body = request;

    println!("Body {}", body);

    Json(());
}

/// Запрос на оплату заказа.
pub async fn create_order_handler(
    request: Request,
) -> impl IntoResponse {

    request_payload().await;

    println!("dfdf {}", request.uri());
    Json(());
}

/// Страница куда переходит пользователь после успешной оплаты заказа.
/// 
/// TODO: Нужно учесть возможность, что хук к нам еще не придет, а пользователь уже перейдет на страницу.
/// 
pub async fn callback_success_page(
    Path(order_id): Path<u64>,
    State(app): State<Arc<Application>>
) -> impl IntoResponse {
    Json(());
}
