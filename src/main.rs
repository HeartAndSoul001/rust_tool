use std::fs;
use regex::Regex;
extern crate ipnet;
use std::net::Ipv4Addr;
use ipnet::Ipv4Subnets;


fn iprange_to_ipcidr(ipranges:&String) {
    let seperator = Regex::new(r"([^\d\.\-]+)").expect("Invalid regex");
    let result = seperator.split(&ipranges);
    let iprange_list: Vec<Vec<String>> = Vec::new();
    for iprange in result {
        let iprange_start: String= [iprange.split(&['.','-']).take(4).collect::Vec<&String>()].join(",");
 //       let a = iprange_start.join(",");
        println!("{:?}", iprange_start);
    }
}

fn main() {
    let file_path= "./iprange.txt";
    let contents = fs::read_to_string(file_path).expect("open the file!");
    iprange_to_ipcidr(&contents)
}
