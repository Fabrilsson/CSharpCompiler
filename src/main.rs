mod token;

use regex::Regex;

fn main() {

    println!("Hello, world!");

    let test = token::get_patterns();

    let text = "if(cont < 5)
                {
                    num = num + cont2;
                }";

    let regex_vec: Vec<String> = test
                            .iter()
                            .map(|i| i.value.as_ref().unwrap().to_owned())
                            .collect();

    let set = Regex::new(&regex_vec.join("|")).unwrap();
    
    let caps = set.captures_iter(text);

    for item in caps {
        println!("{:?}", item.get(0).unwrap().as_str());
    }
}