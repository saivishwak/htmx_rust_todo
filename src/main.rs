use askama::Template;
use axum::{
    extract::State,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    string,
    sync::{Arc, Mutex},
};
use tower_http::services::ServeDir;

#[derive(Template, Debug)]
#[template(path = "todo.html")]
struct TodoTemplate {
    todos: Vec<TodoItem>,
}

#[derive(Template, Debug)]
#[template(path = "todo_item.html")]
struct TodoItemTemplate {
    name: String,
    id: String,
    checked: bool,
}

#[derive(Deserialize, Serialize, Debug)]
struct AppState {
    todos: Vec<TodoItem>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct TodoItem {
    id: String,
    value: String,
    checked: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Item {
    id: String,
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

async fn todos(State(state): State<Arc<Mutex<AppState>>>) -> Html<&'static str> {
    let todo = TodoTemplate {
        todos: state.lock().unwrap().todos.clone(),
    }; // instantiate your struct
    println!("{:#?}", todo);
    Html(string_to_static_str(todo.render().unwrap()))
}

async fn add(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<TodoItem>,
) -> Html<&'static str> {
    let val = payload.value;
    let todos = &mut state.lock().unwrap().todos;
    println!("{:?}", todos);
    todos.push(TodoItem {
        id: payload.id.clone(),
        value: val.clone(),
        checked: false,
    });
    let item = TodoItemTemplate {
        name: val.clone(),
        id: payload.id.clone(),
        checked: false,
    };
    Html(string_to_static_str(item.render().unwrap()))
}

async fn mark_check(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<Item>,
) -> Html<&'static str> {
    let val = payload.id.clone();
    let id = val.as_str();
    let todos = &mut state.lock().unwrap().todos;
    println!("{:?}", todos);
    let mut to = TodoItem {
        id: String::from("test"),
        value: String::from("a"),
        checked: false,
    };
    for todo in todos.iter_mut() {
        if todo.id.as_str() == id {
            println!("Match Found!");
            todo.checked = !todo.checked;
            to = todo.clone();
        };
    }
    let item = TodoItemTemplate {
        name: to.value,
        id: to.id,
        checked: to.checked,
    };
    Html(string_to_static_str(item.render().unwrap()))
}

#[tokio::main]
async fn main() {
    let app_state = AppState { todos: vec![] };
    // build our application with a route
    let my_context = Arc::new(Mutex::new(app_state));
    let app = Router::new()
        .route("/", get(todos))
        .route("/add", post(add))
        .route("/markCheck", post(mark_check))
        .with_state(my_context)
        .nest_service("/assets", ServeDir::new("assets"));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
