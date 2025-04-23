use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    pub sub: String, //id
    pub exp: usize,  
    pub email: String,
    pub role: String,
}


