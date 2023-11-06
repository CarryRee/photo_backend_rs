use axum::response::Html;

pub async fn root() -> Html<&'static str> {
    Html("<H1>This is photo root!<H1>")
}