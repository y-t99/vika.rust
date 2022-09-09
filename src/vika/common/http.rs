use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Response<T> {
    pub code: i32,

    pub message: String,

    pub data: T,
}

#[derive(Deserialize, Serialize)]
pub struct BaseResponse {

    pub code: i32,

    pub message: String,

}