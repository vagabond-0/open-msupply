use super::validate_request;
use actix_web::{
    web::{self, Data},
    HttpRequest, HttpResponse,
};
use service::{auth_data::AuthData, service_provider::ServiceProvider};

#[derive(serde::Deserialize)]
pub struct LabelData {
    code: String,
    message: Option<String>,
}

pub async fn print_label_qr(
    request: HttpRequest,
    data: web::Json<LabelData>,
    service_provider: Data<ServiceProvider>,
    auth_data: Data<AuthData>,
) -> HttpResponse {
    let auth_result = validate_request(request.clone(), &service_provider, &auth_data);
    if auth_result.is_err() {
        return HttpResponse::Unauthorized().body("Access Denied");
    }

    match validate_request(request, &service_provider, &auth_data) {
        Ok((_user, _store_id)) => {}
        Err(error) => {
            let formatted_error = format!("{:#?}", error);
            return HttpResponse::Unauthorized().body(formatted_error);
        }
    };

    // get printer settings

    // print label
    let message = data.message.clone().unwrap_or_default();
    println!(
        "Printing QR label with code: {} message: {}",
        data.code, message
    );

    HttpResponse::Ok().body("QR label printed")
}
