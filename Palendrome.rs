use std::io;

fn main() {
    loop {
        println!("\nPlease input your word.");
        println!("Press 'q' to quit");
                                                            //
        let mut pal = String::new();                        //------+-- 'pal
                                                            //      |
        io::stdin()                                         //      |
            .read_line(&mut pal)                            //      |
            .expect("Failed to read line");                 //      |
                                                            //      |
        let pal = pal.trim().to_lowercase().chars()         //      |
            .filter(|c| c.is_alphanumeric()).collect();     //      |
                                                            //      |
        if pal == "q" {                                     //      |
            println!("\n");                                 //      |
            return;                                         //      |
        }                                                   //      |
                                                            //      |
        let chars: Vec<char> = pal.chars().collect();       //------+
        let len = chars.len();                              //
        let mut palindrome = true;                          //------------------+--'palindrome
                                                            //                  |
        for i in 0..len / 2 {                               //------+-- 'i      |
            if chars[i] != chars[len - 1 - i] {             //      |           |
                palindrome = false;                         //      |           |
                break;                                      //      |           |
            }                                               //------+           |
        }                                                   //                  |
        if palindrome {                                     //                  |
            println!("\nThis word is a palindrome");        //                  |
        }                                                   //                  |
        else {                                              //                  |
            println!("\nThis word is not a palindrome");    //                  |
        }                                                   //------------------+
    }                                                       //
}
