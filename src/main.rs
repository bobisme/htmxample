use axum::{response::Html, routing::get, Router};
use sailfish::TemplateOnce;
use tower_http::services::ServeDir;

#[derive(TemplateOnce)]
#[template(path = "doc.stpl.html")]
struct Simple {
    messages: Vec<String>,
}

#[derive(TemplateOnce)]
#[template(path = "form.stpl.html")]
struct Form;

#[derive(TemplateOnce)]
#[template(path = "contact.stpl.html")]
struct Contact;

async fn root() -> Html<String> {
    let messages = vec![String::from("Message 1"), String::from("<Message 2>")];
    Html(Simple { messages }.render_once().unwrap())
}
async fn form() -> Html<String> {
    Html(Form.render_once().unwrap())
}
async fn contact() -> Html<String> {
    Html(Contact.render_once().unwrap())
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/contact/1", get(contact))
        .route("/contact/1/edit", get(form))
        .nest_service("/assets", ServeDir::new("assets"));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
