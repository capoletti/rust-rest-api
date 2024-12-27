use crate::{
    app_state::AppState,
    models::user_model::{User, UpdateUserSchema},
    response::{GenericResponse, SingleResponse, ListResponse},
};
use chrono::prelude::*;
use rocket::{
    delete, get, http::Status, patch, post, response::status::Custom, serde::json::Json, State,
};
use uuid::Uuid;

#[get("/users?<page>&<limit>")]
pub async fn user_list_handler(
    page: Option<usize>,
    limit: Option<usize>,
    data: &State<AppState>,
) -> Result<Json<ListResponse<User>>, Status> {
    let vec = data.user_db.lock().unwrap();

    let limit = limit.unwrap_or(10);
    let offset = (page.unwrap_or(1) - 1) * limit;

    let list: Vec<User> = vec.clone().into_iter().skip(offset).take(limit).collect();

    let json_response = ListResponse{
        status: "success".to_string(),
        results: list.len(),
        list: list,
    };
    
    Ok(Json(json_response))
}


#[post("/users", data = "<body>")]
pub async fn create_user_handler(
    mut body: Json<User>,
    data: &State<AppState>,
) -> Result<Json<SingleResponse<User>>, Custom<Json<GenericResponse>>> {
    let mut vec = data.user_db.lock().unwrap();

    for user in vec.iter() {
        if user.name == body.name {
            let error_response = GenericResponse {
                status: "fail".to_string(),
                message: format!("User with name: '{}' already exists", user.name),
            };
            return Err(Custom(Status::Conflict, Json(error_response)));
        }
    }

    let uuid_id = Uuid::new_v4();
    let datetime = Utc::now();

    body.id = Some(uuid_id.to_string());
    body.completed = Some(false);
    body.created_at = Some(datetime);
    body.updated_at = Some(datetime);

    let user = body.to_owned();

    vec.push(body.into_inner());

    let json_response = SingleResponse {
        status: "success".to_string(),
        data: user.into_inner(),
    };

    Ok(Json(json_response))
}


#[get("/users/<id>")]
pub async fn get_user_handler(
    id: String,
    data: &State<AppState>,
) -> Result<Json<SingleResponse<User>>, Custom<Json<GenericResponse>>> {
    let vec = data.user_db.lock().unwrap();

    for user in vec.iter() {
        if user.id == Some(id.to_owned()) {
            let json_response = SingleResponse {
                status: "success".to_string(),
                data: user.clone() ,
            };

            return Ok(Json(json_response));
        }
    }

    let error_response = GenericResponse {
        status: "fail".to_string(),
        message: format!("Todo with ID: {} not found", id),
    };
    Err(Custom(Status::NotFound, Json(error_response)))
}

#[patch("/users/<id>", data = "<body>")]
pub async fn edit_user_handler(
    id: String,
    body: Json<UpdateUserSchema>,
    data: &State<AppState>,
) -> Result<Json<SingleResponse<User>>, Custom<Json<GenericResponse>>> {
    let mut vec = data.user_db.lock().unwrap();

    for user in vec.iter_mut() {
        if user.id == Some(id.clone()) {
            let datetime = Utc::now();
            let name = body.name.to_owned().unwrap_or(user.name.to_owned());
            let address = body.address.to_owned().unwrap_or(user.address.to_owned());
            let payload = User {
                id: user.id.to_owned(),
                name: if !name.is_empty() {
                    name
                } else {
                    user.name.to_owned()
                },
                address: if !address.is_empty() {
                    address
                } else {
                    user.address.to_owned()
                },
                completed: if body.completed.is_some() {
                    body.completed
                } else {
                    user.completed
                },
                created_at: user.created_at,
                updated_at: Some(datetime),
            };
            *user = payload;

            let json_response = SingleResponse {
                status: "success".to_string(),
                data: user.clone(),
            };
            return Ok(Json(json_response));
        }
    }

    let error_response = GenericResponse {
        status: "fail".to_string(),
        message: format!("Todo with ID: {} not found", id),
    };

    Err(Custom(Status::NotFound, Json(error_response)))
}

#[delete("/users/<id>")]
pub async fn delete_user_handler(
    id: String,
    data: &State<AppState>,
) -> Result<Status, Custom<Json<GenericResponse>>> {
    let mut vec = data.user_db.lock().unwrap();

    for user in vec.iter_mut() {
        if user.id == Some(id.clone()) {
            vec.retain(|user| user.id != Some(id.to_owned()));
            return Ok(Status::NoContent);
        }
    }

    let error_response = GenericResponse {
        status: "fail".to_string(),
        message: format!("Todo with ID: {} not found", id),
    };
    Err(Custom(Status::NotFound, Json(error_response)))
}