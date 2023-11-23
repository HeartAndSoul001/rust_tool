use std::{fs, clone};
use regex::Regex;
extern crate ipnet;
use std::net::Ipv4Addr;
use ipnet::Ipv4Subnets;


fn iprange_to_ipcidr(ipranges:&String) {
    let seperator = Regex::new(r"([^\d\.\-]+)").expect("Invalid regex");
    let result = seperator.split(&ipranges);
    let iprange_list: Vec<Vec<String>> = Vec::new();
    for iprange in result {
        let ip_vec: Vec<&str> = iprange.split(&['.','-']).collect();
        let (iprange_0_3, iprange_head,iprange_tail) = (ip_vec[0..3].to_vec(), ip_vec[3], ip_vec.last().unwrap());
        //let (mut iprange_start_list, mut iprange_end_list) = (iprange_0_3.clone(), iprange_0_3.clone());
        //iprange_start_list.push(iprange_head);
        //iprange_end_list.push(iprange_tail);
        //let (iprange_start, iprange_end) = (iprange_start_list.join("."), iprange_end_list.join("."));
        //println!("{}",iprange_start);
        //println!("{}",iprange_end);
    }
}

fn main() {
    let file_path= "./iprange.txt";
    let contents = fs::read_to_string(file_path).expect("open the file!");
    iprange_to_ipcidr(&contents)
}
