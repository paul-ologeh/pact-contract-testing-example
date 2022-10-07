mod lib;

use crate::lib::{create_user, get_user};
use user_provider::UserIn;

#[tokio::main]
async fn main() {
    let base_url = "http://localhost:8000/".to_string();

    let new_user = UserIn {
        username: "paulologeh".to_string(),
        name: "Paul".to_string(),
        surname: "Ologeh".to_string(),
        password: "mypassword".to_string(),
        age: 25,
        job_title: Some("Software Engineer".to_string()),
        phone_number: Some("123456890".to_string()),
    };

    let created_user = create_user(base_url.clone(), new_user)
        .await
        .expect("failed to create user");
    println!("created_user {:?}", created_user);
    let fetched_user = get_user(base_url, created_user.id)
        .await
        .expect("failed to get user");
    println!("fetched_user {:?}", fetched_user);
}
