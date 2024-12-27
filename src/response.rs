use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct SingleResponse<T> {
    pub status: String,
    pub data: T,
}

#[derive(Serialize, Debug)]
pub struct ListResponse<T> {
    pub status: String,
    pub results: usize,
    pub list: Vec<T>,
}