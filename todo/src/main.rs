
mod todo;

use todo::auth::{JwtToken};



fn main() {
    
    let token = todo::auth::JwtToken::new("user_id".into());
    println!("Hello, world!");
}
