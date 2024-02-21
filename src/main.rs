use std::io;

struct User {
    email: String,
    password: String,
}

fn build_user(email:String, password: String) -> User {
    User {
        email: email,
        password: password,
    }
}

fn register_user() -> User {
    println!("Please enter your email:");
    let mut email: String = String::new();
    io::stdin()
        .read_line(&mut email)
        .expect("Failed to read line");

    println!("Please enter your password:");
    let mut password: String = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");

    let email:String = email.trim().parse().expect("failed to parse");
    let password: String = password.trim().parse().expect("failed to parse");

    build_user(email, password)
}

fn login(user: Option<&User>) -> bool {
    println!("Please enter your email:");
    let mut email: String = String::new();
    io::stdin()
        .read_line(&mut email)
        .expect("Failed to read line");

    println!("Please enter your password:");
    let mut password: String = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");

    if let Some(user) = user {
        email.trim() == user.email && password.trim() == user.password
    } else {
        println!("User in None");
        return false;
    }
    
}

fn main() {

    let mut user1: Option<Box<User>> = None;

    loop {
        
        println!("Would you like to register, login, or exit?");

        let mut log_reg: String = String::new();
        io::stdin()
            .read_line(&mut log_reg)
            .expect("Failed to read line");

        let log_reg = log_reg.trim();

        
        
        if log_reg == "r" {
            user1 = Some(Box::new(register_user()));
            println!("User ({0}) has been registered", user1.as_ref().unwrap().email);
        } else if log_reg == "l" {
            if let Some(user) = &user1 {
                let auth = login(Some(&user));
                if auth {
                    println!("Login Successful, you are now logged in as {}", user.email);
                    break;
                } else {
                    println!("Failed to login");
                    continue;
                }
            } else {
                println!("No user registered, please create one before attempting to log in...");
            }
        } else if log_reg == "e" {
                println!("exiting...");
                break
        }
    }
}