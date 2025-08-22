//use standard libraries

use::std::collections::HashMap;
use::std::io;



fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("\n Enter a command (example: Add Sally to Engineering, List Engineering, List All, or Quit):")


        //read input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read message");
        input.trim();// remove extra spaces

        //quit condition
        if input.eq_ignore_ascii_case("quit"){
            break;
        }

    }

}
