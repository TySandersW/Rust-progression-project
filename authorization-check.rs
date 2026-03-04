fn main() {
    let valid = vec![
        ("Ty", "kjsD!nefl10"),
        ("Jaeni", "Adfkjz@fs3"),
        ("Carson", "Adfklj42$"),
        ("Grace", "Asdlfkj!3ff"),
        ("Issac", "Seflkj!32lkj"),
        ("Jaron", "Sjfoiwef3##"),
        ("Elijah", "Sdfkj@332jj"),
        ("Hannah", "Sdflkjoi2#@@"),
        ("Ezra", "wEfkjweio22#!"),
        ("Joel", "aElkjsdf@2312"),
    ];

    println!("Valid tests:");
    for pair in valid.iter() {
        let i: &str = pair.0;
        let j: &str = pair.1;
        let i_string: String = i.to_string();
        let j_string: String = j.to_string();

        println!("{} / {} => {}", i, j, test(i_string, j_string));
    }

    let invalid = vec![
        ("bad_user", "aa1!aa"),
        ("bad=user", "aa1!aaaa"),
        ("user", "aaaaaaaa!"),
        ("user", "sssssa"),
        ("user", "aaaaaaa"),
        ("user", "dfdfsdfss"),
        ("user", "sdfsdf"),
        ("user", "z*fsdfs!"),
        ("user", " "),
        ("user", "aaaaaa!"),
        ("user", "aaaaaaa"),
        ("user", "aaaaaa"),
        ("user", "aaaaaaaa"),
        ("user", "aaaaaa"),
        ("user", "aaaaaaa"),
        ("user", "aaaaaaa"),
        ("user", "aaaaaaaa"),
        ("user", "aaaaaaaa"),
        ("user", "aaaaaaaa"),
        ("user", "aaaaaaaa"),
    ];

    println! ("\n\ninvalid tests");
    for (i, j) in invalid.iter() {
        println!("{} / {} => {}", i, j, test(i.to_string(), j.to_string()));
    }

}

fn test(username: String, password: String) -> String {
    if !user_check(&username) {
        return "Invalid username".to_string();
    }
    if !pass_check(&password) {
        return "Invaled password".to_string();
    }

    format!(
        "SELECT * \nFROM accounts \nWHERE userId = '{}' AND pswd = '{}'",
        username, password
    )
}

fn user_check (username: &String) -> bool {
    for i in username.chars() {
        if i == '\'' || i == '=' {
            return false;
        }
    }
    true
}

fn pass_check(password: &String) -> bool {
    if password.len() < 8 {
        return false;
    }

        let mut upper = false;
        let mut lower = false;
        let mut digit = false;
        let mut spec = false;

        for i in password.chars() {
            if i >= 'A' && i <= 'Z' {
                upper = true;
            } 
            else if i >= 'a' && i <= 'z' {
                lower = true;
            } 
            else if i >= '0' && i <= '9' {
                digit = true;
            } 
            else if i == '~' || i == '!' || i == '@' || i == '$' || i == '%' || i == '^' || i == '&' {
                spec = true;
            } 
            else {
                return false;
            }
        }

        upper && lower && digit && spec
}
