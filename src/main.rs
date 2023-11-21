use std::fs;
use regex::Regex;

fn iprange_to_ipcidr(iprange:String) -> String {
    let seperator = Regex::new(r"([ ,.]+)").expect("Invalid regex");
    String::from(format!("hello,{}!", iprange))
}

fn main() {
    let file_path= "../iprange.txt";
    let contents = fs::read_to_string(file_path).expect("open the file!");
    println!("{}", contents);
    // println!("{}", iprange_to_ipcidr(String::from("world")));
}
