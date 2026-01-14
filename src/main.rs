mod scrabble;
use axum::Router;
use axum::extract::Form;
use axum::response::Html;
use axum::routing::{get, post};
use lazy_static::lazy_static;
use serde::Deserialize;
use tera::Tera;
use tower_http::services::ServeDir;
use tower_http::services::ServeFile;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "templates/**/*";
        Tera::new(source).unwrap()
    };
}

#[tokio::main]
async fn main() {
    // build our application with a route

    let app = Router::new()
        .route("/", get(index))
        .route("/about", get(about))
        .route("/demo_page", get(demo_page))
        .route("/cheat_page", get(cheat_page))
        .route("/guess_page", get(guess_page))
        .route("/get_full_answer", get(get_full_answer))
        .route("/get_guess_answer", get(get_guess_answer))
        .route("/anagram_page", get(anagram_page))
        .route(
            "/for_anagram_get_full_answer_from_user_tiles",
            post(for_anagram_get_full_answer_from_user_tiles),
        )
        .route(
            "/get_full_answer_from_user_tiles",
            post(get_full_answer_from_user_tiles),
        )
        .route("/check_guess", post(check_guess))
        .fallback_service(
            ServeDir::new("/assets").not_found_service(ServeFile::new("assets/404.html")),
        )
        .nest_service("/favicon.ico", ServeFile::new("image/favicon.svg"))
        .nest_service("/style.css", ServeFile::new("css/style.css"))
        .nest_service("/js/htmx.min.js", ServeFile::new("js/htmx.min.js"));

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:443").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

//ABOUT_PAGE
async fn about() -> Html<String> {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("about.html", &context).unwrap();
    Html(page_content)
}

//ANAGRAM_PAGE
async fn anagram_page() -> Html<String> {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("anagram_scrabble.html", &context).unwrap();
    Html(page_content)
}

async fn for_anagram_get_full_answer_from_user_tiles(Form(data): Form<FormData>) -> Html<String> {
    let mut context = tera::Context::new();
    let tiles = &data.the_tiles;
    let answer = scrabble::anagram_run(tiles.chars().collect()).await;
    context.insert("tiles", &answer.tiles);
    context.insert("valid_word", &answer.valid_words);
    let page_content = TEMPLATES.render("anagram_answer.html", &context).unwrap();
    Html(page_content)
}

//CHEAT_PAGE
#[derive(Deserialize)]
struct FormData {
    the_tiles: String,
}

async fn cheat_page() -> Html<String> {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("cheat_scrabble.html", &context).unwrap();
    Html(page_content)
}

async fn get_full_answer_from_user_tiles(Form(data): Form<FormData>) -> Html<String> {
    let mut context = tera::Context::new();
    let tiles = &data.the_tiles;
    let answer = scrabble::cheat_run(tiles.chars().collect()).await;
    context.insert("tiles", &answer.tiles);
    context.insert("longest_word", &answer.longest_valid_words);
    context.insert("highest_word", &answer.highest_scoring_words);
    context.insert("valid_word", &answer.valid_words);
    let page_content = TEMPLATES.render("full_answer.html", &context).unwrap();
    Html(page_content)
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct FormGuessData {
    the_guess: String,
}

//GUESS_PAGE
async fn guess_page() -> Html<String> {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("guess_scrabble.html", &context).unwrap();
    Html(page_content)
}

async fn get_guess_answer() -> Html<String> {
    let answer = scrabble::demo_run().await;
    let mut context = tera::Context::new();
    context.insert("tiles", &answer.tiles);
    context.insert("longest_word", &answer.longest_valid_words);
    context.insert("highest_word", &answer.highest_scoring_words);
    context.insert("valid_word", &answer.valid_words);
    let page_content = TEMPLATES.render("guess_answer.html", &context).unwrap();
    Html(page_content)
}

async fn check_guess(Form(data): Form<FormGuessData>) -> Html<String> {
    let mut context = tera::Context::new();
    let guess = &data.the_guess;
    context.insert("guess", &guess);
    let page_content = TEMPLATES.render("guess.html", &context).unwrap();
    Html(page_content)
}

//DEMO_PAGE
async fn demo_page() -> Html<String> {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("demo_scrabble.html", &context).unwrap();
    Html(page_content)
}

async fn get_full_answer() -> Html<String> {
    let answer = scrabble::demo_run().await;
    let mut context = tera::Context::new();
    context.insert("tiles", &answer.tiles);
    context.insert("longest_word", &answer.longest_valid_words);
    context.insert("highest_word", &answer.highest_scoring_words);
    context.insert("valid_word", &answer.valid_words);
    let page_content = TEMPLATES.render("full_answer.html", &context).unwrap();
    Html(page_content)
}

//INDEX_PAGE
async fn index() -> Html<String> {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("index.html", &context).unwrap();
    Html(page_content)
}
