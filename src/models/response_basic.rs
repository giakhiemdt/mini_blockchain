use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseModel<T> {  
    pub is_success: bool,
    pub result: Option<T>,  
    pub message: String,
}
