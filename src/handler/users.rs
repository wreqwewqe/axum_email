use std::sync::Arc;

use crate::config;
use crate::models;
use crate::AppState;
use axum::extract::State;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use lettre::message::header::ContentType;
use lettre::message::MultiPart;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde_json::json;
use tokio::time::Duration;

pub async fn bind(
    State(app_state): State<Arc<tokio::sync::RwLock<AppState>>>,
    Json(params): Json<models::users::Bind>,
) -> Result<impl IntoResponse, config::AppError> {
    println!("params:{:?}", params);
    let row = sqlx::query("insert into users (address,email) values ($1,$2)")
        .bind(&params.address)
        .bind(&params.email)
        .execute(&app_state.write().await.pool)
        .await;

    match row {
        Ok(i) => {
            println!("666");
            Ok(Json(json!({
                "code":200,
            })))
        }
        Err(err) => Err(config::AppError::new(500, err.to_string())),
    }
}

pub async fn update_content(
    State(mut state): State<Arc<tokio::sync::RwLock<AppState>>>,
    Json(update_content): Json<models::users::UpdateContent>,
) -> Result<impl IntoResponse, String> {
    state.write().await.content = update_content.content;
    println!("修改过后的值:{:?}", state.write().await.content);
    Ok(Json(json!({
        "code":200,
        "msg":"修改成功"
    })))
}
pub async fn send_email(
    State(state): State<Arc<tokio::sync::RwLock<AppState>>>,
    Json(params): Json<models::users::Bind>,
) {
    // println!("std:{:?}", app_state.env.A);
    //设置延时
    match params.delay {
        0 => (),
        _ => tokio::time::sleep(Duration::from_secs(params.delay)).await,
    }
    if params.email.contains("@qq") {
        let creds = Credentials::new(
            "1984850802@qq.com".to_owned(),
            std::env::var("QQSECRET").unwrap().to_owned(),
        );

        let email = Message::builder()
            .from("1984850802@qq.com".parse().unwrap())
            .to(params.email.parse().unwrap())
            .subject("欢迎")
            .multipart(MultiPart::alternative_plain_html(
                String::from(""),
                String::from(&state.write().await.content),
            ))
            .unwrap();

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay("smtp.qq.com")
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }
    }
    if params.email.contains("@gmail") {
        let email = Message::builder()
            .from("chengpeng107@gmail.com".parse().unwrap())
            .to(params.email.parse().unwrap())
            .subject("Happy new years")
            .header(ContentType::TEXT_PLAIN)
            .body(String::from("Be happy!"))
            .unwrap();

        let creds = Credentials::new(
            "chengpeng107@gmail.com".to_owned(),
            std::env::var("GMAILSECRET").unwrap().to_owned(),
        );

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => println!("gEmail sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }
    }
}
