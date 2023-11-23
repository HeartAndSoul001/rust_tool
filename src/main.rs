use std::fs;
use regex::Regex;
extern crate ipnet;
use std::net::Ipv4Addr;
use ipnet::Ipv4Subnets;


fn iprange_to_ipcidr(ipranges:&String) {
    let seperator = Regex::new(r"([^\d\.\-]+)").expect("Invalid regex");
    let result = seperator.split(&ipranges);
    //let iprange_list: Vec<Vec<String>> = Vec::new();
    for iprange in result {
        let ip_vec: Vec<&str> = iprange.split(&['.','-']).collect();
        let iprange_0_3 = ip_vec[0..3].to_vec().join(".");
        let iprange_head = format!("{iprange_0_3}.{}", ip_vec[3]);
        let iprange_tail = format!("{iprange_0_3}.{}", *ip_vec.last().unwrap());
        println!("{}", iprange_head);
        println!("{}", iprange_tail);
    }
}

fn main() {
    let file_path= "./iprange.txt";
    let contents = fs::read_to_string(file_path).expect("open the file!");
    iprange_to_ipcidr(&contents)
}
