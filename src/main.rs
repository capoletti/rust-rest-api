use app_state::AppState;
use handlers::health_handler::health_checker_handler;
use handlers::todo_handler::{create_todo_handler, delete_todo_handler, edit_todo_handler, get_todo_handler, todos_list_handler,};
use handlers::user_handler::{create_user_handler, user_list_handler,get_user_handler,edit_user_handler,delete_user_handler,};

#[macro_use]
extern crate rocket;

mod handlers;
mod models;
mod response;
//mod todo_response;
pub mod app_state;

#[launch]
fn rocket() -> _ {
    //let app_data = model::AppState::init();
    let app_data = AppState::init();
    println!("{}", "🚀 The server is ready to accept requests");
    rocket::build().manage(app_data)
    .mount(
        "/api",
        routes![
            health_checker_handler,
            todos_list_handler,
            create_todo_handler,
            get_todo_handler,
            edit_todo_handler,
            delete_todo_handler,
            create_user_handler,
            user_list_handler,
            edit_user_handler,
            delete_user_handler,
        ],
    )
    .mount(
        "/api/v2", 
        routes![
            get_user_handler,
        ],
    )

    
}