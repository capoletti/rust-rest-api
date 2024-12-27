use std::sync::{Arc, Mutex};

use crate::models::todo_model::Todo;
use crate::models::user_model::User;

pub struct AppState {
    pub todo_db: Arc<Mutex<Vec<Todo>>>,
    pub user_db: Arc<Mutex<Vec<User>>>,
}

impl AppState {
    pub fn init() -> AppState {
        AppState {
            todo_db: Arc::new(Mutex::new(Vec::new())),
            user_db: Arc::new(Mutex::new(Vec::new())),
        }
    }
}