#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn deactivate(&mut self) {
        self.active = false;
    }

    fn from(email: &str) -> User {
        let mut username: String = String::from("");
        for c in email.chars() {
            if c == '@' {
                break;
            }
            username.push(c);
        }
        User {
            username,
            email: String::from(email),
            sign_in_count: 0,
            active: false,
        }
    }
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 0,
    }
}

fn main() {
    let user1 = build_user(String::from("user1"), String::from("user1@gmail.com"));
    let user2 = User {
        username: String::from("user2"),
        email: String::from("user2@gmail.com"),
        ..user1
    };
    let mut user3 = User::from("user3@gmail.com");
    user3.deactivate();
    println!("Hello, user! {:#?}", user1);
    println!("Hello, user! {:#?}", user2);
    println!("Hello, user! {:#?}", user3);
}
