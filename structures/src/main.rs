struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(String::from("username@email.com"), String::from("username"));
   
    let user2 = User {
        email: String::from("another@email.com"),
        ..user1
    };
    println!("User1 email: {}. \nUser2 email: {}", user1.email, user2.email); 
}
