//Fingerprint Groups
//C. Wyatt Polasek + Zach Breene

/*For help solving this program, we used the following resources:
GitHub Copilot
https://doc.rust-lang.org/std/collections/struct.HashMap.html
https://doc.rust-lang.org/std/io/struct.Stdin.html
https://doc.rust-lang.org/std/io/trait.BufRead.html
https://doc.rust-lang.org/std/vec/struct.Vec.html#method.join
https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push
https://doc.rust-lang.org/std/vec/struct.Vec.html#method.splitn
https://stackoverflow.com/questions/36362020/what-is-unwrap-in-rust-and-what-is-it-used-for
https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_default
https://doc.rust-lang.org/std/string/trait.ToString.html
https://doc.rust-lang.org/std/primitive.char.html#method.is_whitespace
https://doc.rust-lang.org/stable/nightly-rustc/rustc_lexer/fn.is_whitespace.html
https://www.educative.io/answers/what-is-the-trim-function-in-rust
*/

use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    //Creating HashMap and Vec to store fingerprint groups and final list of names. Made in design document
    let mut fingerprint_groups: HashMap<String, Vec<String>> = HashMap::new(); 
    let mut final_list: Vec<String> = Vec::new();

    //Loop to iterate over stdin and add fingerprints and names into HashMap
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        //Using splitn to split line into fingerprint and name
        //Is_whitespace is used to split on whitespace. It returns true if the character is whitespace
        //I was having a problem in Gradescope for a while with extra unneeded spaces/whitespace before names in the output. I used trim after getting some help and doing more research to remove the whitespace.
        //The trim function is a built-in function in Rust used to trim leading and trailing whitespaces in a string.
        let mut split = line.splitn(2, char::is_whitespace);
        let (fingerprint, name) = (split.next().unwrap(), split.next().unwrap_or_default().trim());

        //Error check to make sure fingerprint is not too long
        if fingerprint.len() <= 512 {
            //Adding names into corresponding fingerprint groups
            fingerprint_groups.entry(fingerprint.to_string()).or_default().push(name.to_string());
        } else {
            //Error message
            eprintln!("Error: fingerprint too long");
        }
    }

    //Loop to iterate over fingerprint groups and add names into final list
    let mut first_group = true;
    for (_fingerprint, names) in fingerprint_groups {
        if names.len() > 1 {
            //If else statement to determine where to put the newline between fgroups
            if first_group {
                first_group = false;
            } else {
                final_list.push("".to_string());
            }
            //Push names into the final list
            final_list.push(names.join("\n"));          
        }
    }
    //Output to terminal
    println!("{}", final_list.join("\n"));
}