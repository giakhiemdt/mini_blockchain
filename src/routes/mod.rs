pub mod transaction_routes;
pub mod wallet_routes;
pub mod mine;
pub mod auth_routes;
pub mod user_routes;

pub use auth_routes::{public_auth_routes, protected_auth_routes};
pub use user_routes::protected_user_routes;
pub  use wallet_routes::protected_wallet_routes;