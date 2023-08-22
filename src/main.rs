use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Postgres};
use std::{env::Args, net::SocketAddr, path::PathBuf};
use tokio::time::Duration;
mod config;
mod models;
use axum::extract::State;
use serde_json::json;
mod handler;
use std::sync::Arc;
use tokio::sync::RwLock;
#[derive(Debug, Clone)]
pub struct AppState {
    pool: sqlx::Pool<Postgres>,
    content: String,
}
#[tokio::main]
async fn main() {
    println!("hh：{:?}", "ab");
    dotenvy::dotenv().unwrap();
    let pool: sqlx::Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:123456@localhost/postgres")
        .await
        .unwrap();
    let mut app_state =Arc::new(RwLock::new(AppState { pool,content: r#"5
    <img src="data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAAAQABAAD/2wCEAAkGBwgHBgkIBwgKCgkLDRYPDQwMDRsUFRAWIB0iIiAdHx8kKDQsJCYxJx8fLT0tMTU3Ojo6Iys/RD84QzQ5OjcBCgoKDQwNGg8PGjclHyU3Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3Nzc3N//AABEIAIQAhAMBIgACEQEDEQH/xAAcAAAABwEBAAAAAAAAAAAAAAAAAQIDBAUGBwj/xAA7EAABAwIDBgMGAwcFAQAAAAABAAIDBBEFEiEGMUFRcZETImEHFDJCgaEjUrEVJGJywdHwM0ST4fEW/8QAGgEAAgMBAQAAAAAAAAAAAAAAAgMAAQQFBv/EACYRAAMAAgEEAgICAwAAAAAAAAABAgMREgQhMUEFEyJRYeEygaH/2gAMAwEAAhEDEQA/AMW4m51O88Um55nug46nqUXBc09aAk8z3QueZ7pJK2ezWwNVjNKyomnbTMeLszakjnZEpb8CsuaMS3bMdc8z3R3PM91otrdj63Zx7XOPjUrtGzDnyt/nFZwKgoubnlLDF+Z7o9eZ7okd9LKgwXPM90NeZ7oIKEBrzPdC55nugrjAdmsUx6Qigp7xt+KV5ytb6XVpN+AaqZW6ekU9zzPdC55nurbH9ncRwGYR10Ja12rXg3B+qqFC5apbQdzzPdC55nuggqLDBPM90EQRqEEneeqSlO3nqkqEHaRjZKyBj/gfI1ruhIC7hhdUI6nwmizW6NA4ALhRvwNjwPJdVwrERV+61sZ0lYCR+U8R3un43+FI5PyMN3Feu5qdtaWPENksQZI3MY4TK3Ti3VcCG4L0UwNrMPlgefLLG5h6EWXBqDBMTrag0tJRzTTRuyPyN0a7dqeG5Z534G9DSmaTZXo10LCvZjUyPBxSuihb+SIZnd9y0EPsuwWIh8tVWSW3tu0A+u5N+qvY19f06elWzjtkdiuzzezHAJYnCGSqjcdzvEB/ULOYl7LK6O7sOroajXRkgLDb7hC5aCjrunv3o51kc7ysF3u0aBxPBd/wekhwegp6KBoa2JtjbieJ7rlez+zWIM2qoaXEKOWEMk8V5c3SzNdDuOth9V1aa4eSnY+2OqOd8pavLjxy+3krPaMyKo2QqpHtbmhLXMJG4k207riC6j7UMTbHhEGGMN5aiQSPHJjf7ut2K5gQskVtbZ0uihzi/wBhIIIIzUAI0QRqEEneeqTxSjvPVHZQoLRaLZHEDFK+hkccjzniJ+V/EfUD7eqzwCt9mcOfiGMQRtcY2xuEkjxva0G+nqjhvlpCeomXhp34Xc65heIfugcTuVnRVlNFG4RsZGHOLnBotck3J+qpq6SnbTtjiDWcGhvBJwk0zbh7i53E3WpxMT28nludZK2/H6LGSZ5qC5soYw7hxVm1x8DOXl4trqsni7jTzsMbyWHXRT6CvlMGUsIBG92iRltNJj8WN914LqjqoZHZWgi3G6kTSxseLuOvIrNRSMppC51XDv3b09JiFNM9v74y43clnrIuRoXT012NIwh7A74uo1VVi+SigkqpXWgjaXvJ+UBKpKvyWY9r/wCVyl/hVlNJTzsD2SNLXtdxB3goHk5JzvQKj66VUjguN182LYnPWzXGc+Rv5GjcP843Ve5i0O1WCuwXF5afI4QOOaFx4t/zRU80QY8ta9rxYeZu4qSz1UKahOPBBLUkiykvYmXtRpgOdCAjRWQRAA4nqUYCPLqeqejADXAsBJtY3OihJQlrVstkKN9Ph0tYG+ed2Rl/yt4jqT9lk2tW7oajwMJw5jdcsQP1PHuU/pkne36Ob8xVTgUz7f8AZLmp6iCMufqT67lXUlTUOqS2MEnieScnxVzvw7kuOij1VV7vEIYf9R97kJmW9I5GDFTa2WtRisUFtfEmO7jZHSw4jiZBlkMTDwbvVXhtO1pEkvmedSStTh87dALBZuG+7NTvh2lEqi2XpS0OnLnn+Iqf/wDMYaW28ED1BUmkmFhqp7X3CCpkX9uT9mWrNmJqe8uG1D2kfK43SMHxmVtR7niDfDqBoCeK1xcCsjtxRBtMMQh0kgNyRxbxWe4T8GjFm+z8MhB9plF7xhdLXNB8SF/hn1Dv/PuubzRtbI5rS4gGwzNsfqF0TaGpkxPYzxGeYtkZ4gB4Dj+iwBYhmn4Z3fjYf08X6bIjmeiYexT3tF9L29UxIxNTNVwQC3VBPlmqCPYjgNtbqeqkBrbC178bpDBr9VIa25Nha/DkrbKhBNatlSUodQUjhpaAE2WSDbLT0VZbCYmg6xtyG3of+wtPSNcns5fzUU8MufTI0YDZnSDeNAmaS1RUOkdc8GoCXyvd6OP2SMPl8OIE8AqyvdmDBDUfyxG1GNyYJQslggEr5HhjbmwGhP8ARSth9oJ8ZglfND4UkL8rgDcG4uLKRS4lFNmgngilido5kkYcD9CralNLFGI6anhgZe+WKMMHYKtriMyYrx3qkaGiqTpqpmD4s+ulkidQ1VP4e90zMoPTn9FSQ5JWhud7CDcOYRf7q3oYImFzmTzXdrJ5h5zuudNNAN1llvZHMcX27kD2jbQV2zuznvmGRMkqHzsiDpGktjBucxA6W6kJ+OefFdkWTV0PgzVFKHPjIIsS3dqrsSNAtv6qjxmtkbHOXfA1hOqXtdl7Axw2ZvCpHO2WxGN27wQR1uAsplWpLjQbMzA/FUlsYH3KzeVDWuTPSfFpvE6/bI72hMPYp+d7I3xtIDX2zCw1sbhR3tVpm6pIJZqgnyzVBM2J4kZjdT1UhgSGN0vpv3cU8waI2JgXExpe0PdkaTq617fRTKGYR543khknG248FHiyCRpkDi0HzBpsSEoAbyD9FU05e0FkwzlhxXhj+kUpjl0abg9CFDilNM7wZyA4aejuiaxWR0dGQHOB+Wx/RZZ85v5pHj+cJ/Ln+Rx30X1Vw5G4glhBu0hToqwC2q54yodwkCkRzv4vHdA60OXR1kfejpdPiIb81lZ0+NRsAvKO65Syc21eAl+MPmkHZIq/4NMfF780dZdtRRQj8SobflvKrKvaamxKojpS2RlMTd78uptray542Rp3Oe70a1WOGAulJc0tFvmOpSG2ns2T8Tg4vk9l/itea6Voa3JTxaRs/qfVQSAl2QIVm3HjnHCifCGrlt7cRY6Jp7VIcE08a6BWmVSIz2FriCCCDZBLLdUEexWiJG0kOdppvuU/GASATYc1Fa4XPVPseE7RiiiQ3gnGlw+E2uLFNNcE614sRpqhaNEtEeuoRXRiPMWvv5HDmqOXZ7FWyvjDYn5DbR+/7LUwODZo3HcHg/dO4hMYap0jNddQtGCU5ezjfK5rxXPD3sxztncXa3N+z84/gIKgvY+B5ZNSyMc3QtdcELpmE47FE4Nk3eqTtTU4dXPp307WmYNIkcBw4KqX5aM+Pq8qWznlOHSxyyR0+ZkIBkO/KDxKeikJHkjb9AtVs/BhE+I5cSyCKxAuBoeysMWggp2RGliaYs9/KNHAFZm5duDoR8hanfErcD2NxzF2skytpqd2ueY7x6Df+iZ2mw1mzW0kNBTzvltSMle5+8uLnA9NANF03AMabVQRshiLSBYjgFzH2mPLdvqh4N/wYrn1yoUk20TD13UX1MzfZfolQVMczQ5rhrwTwFyBcC/EqhpA58jRF8Lt9vlV4LAABJWz0F69CmlgPnaSLHS9tbaJh6W5wTT3I0KrQhBILtUaLQvZSeOQT1ShV2TeW5PUpQjB4LZ2ONq/THPfyEf7StxTfgNO8JBpGlTcE45vTJH7U9VMbibaoB1xmA8wv91TOoGu/N9E2cNe1wcyd7SNxARTcLwZs/T58qSruXZkhc65Iv1T0ckFtHAKPhlfLRQiOWkp6iw+J12k6/XgomNGXEJWPo4YqANvcR+bNrpe/JC8k7FLosyX+JZU2H0stWJZZSGje1ptfqtcKuiMLWPyZWiwC5i2lxJv++P/ABj+yfbQVhaJJK6VwzZbBw39Eq1Le9mrH02XWmv+nWMJxBhJ90he8NFyWMJ0XMto8UixmpkrgwtmfJm9cu4C/oLdlPpMYxmlp2QU9e+KNosAxjQe9r8FBZA1u4d0iUpbZvw9NU06eh3DXtij+IFzuXBT/eAd36qC1gG4BODRUdJP9kky3SHPTWZILlaApjudEmcyCIXsh21PVLCJBaGc6RYSwEEEDHIMBHYaIkEDGigAjDQgghDSBYIABBBRl7DA0RoIIWGGiRoKFiSklBBEgGJugggiBP/Z" alt="Embedded Image">
5"#.to_string()}));

    let app = Router::new()
        // `POST /users` goes to `create_user`
        .route("/bind", post(handler::users::bind))
        .route("/updatecontent", post(handler::users::update_content))
        .route("/sendemail", post(handler::users::send_email))
        .with_state(app_state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
