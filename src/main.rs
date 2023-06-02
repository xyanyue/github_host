use std::collections::HashMap;
use std::{format, path::Path};
use std::{println, vec};

use regex::Regex;
use reqwest::header::{self, REFERER, USER_AGENT};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Error, Write};
const GITHUB_URLS: [&str; 39] = [
    "github.com",
    "github.io",
    "alive.github.com",
    "api.github.com",
    "assets-cdn.github.com",
    "avatars.githubusercontent.com",
    "avatars0.githubusercontent.com",
    "avatars1.githubusercontent.com",
    "avatars2.githubusercontent.com",
    "avatars3.githubusercontent.com",
    "avatars4.githubusercontent.com",
    "avatars5.githubusercontent.com",
    "camo.githubusercontent.com",
    "central.github.com",
    "cloud.githubusercontent.com",
    "codeload.github.com",
    "collector.github.com",
    "desktop.githubusercontent.com",
    "favicons.githubusercontent.com",
    "gist.github.com",
    "github-cloud.s3.amazonaws.com",
    "github-com.s3.amazonaws.com",
    "github-production-release-asset-2e65be.s3.amazonaws.com",
    "github-production-repository-file-5c1aeb.s3.amazonaws.com",
    "github-production-user-asset-6210df.s3.amazonaws.com",
    "github.blog",
    "github.community",
    "github.githubassets.com",
    "github.global.ssl.fastly.net",
    "github.map.fastly.net",
    "githubstatus.com",
    "live.github.com",
    "media.githubusercontent.com",
    "objects.githubusercontent.com",
    "pipelines.actions.githubusercontent.com",
    "raw.githubusercontent.com",
    "user-images.githubusercontent.com",
    "vscode.dev",
    "education.github.com",
];
const DNS_API: &str = "https://www.ipaddress.com/site/";

const START: &str = "# =========Github Start==========";
const END: &str = "# =========Github End==========";
fn main() {
    let ips = get_host_ip();
    if ips.len() > 2 {
        write_hosts("/etc/hosts", ips);
    }
}

fn get_host_ip() -> Vec<String> {
    println!("START=====");
    io::stdout().flush().unwrap();
    let mut ips = vec![START.to_owned()];

    for website in GITHUB_URLS {
        let url = format!("{}{}", DNS_API, website);
        let client = reqwest::blocking::Client::new();
        match client
            .get(url)
            .header(USER_AGENT, rand_user_agent::UserAgent::random().to_string())
            .header(REFERER, "https://www.ipaddress.com/")
            .send()
        {
            Ok(response) => {
                // is available via the hostname that resolves the IP address 140.82.113.3.
                let re = Regex::new(r"<li>(\d+\.\d+\.\d+\.\d+)<\/li>").unwrap();
                let text = response.text().unwrap();
                let mut tmp = HashMap::new();
                for cap in re.captures_iter(&text) {
                    tmp.insert(cap[1].to_string(), website.to_string());
                }
                for ele in tmp.iter() {
                    ips.push(format!("{}     {}", ele.0, ele.1));
                    println!("{}      {}", ele.0, ele.1);
                }
                io::stdout().flush().unwrap();
                // response.text()
            }
            Err(e) => {
                println!("ERR:{}", e.to_string());
                io::stdout().flush().unwrap();
            }
        }
    }
    ips.push(END.to_owned());
    ips
}

fn clear_history_hosts(path: &str) -> Vec<String> {
    let mut source = Vec::new();
    // 在生成输出之前，文件主机必须存在于当前路径中
    if let Ok(lines) = read_lines(path) {
        // 使用迭代器，返回一个（可选）字符串
        let mut github_flag = false;
        for line in lines {
            if let Ok(ip) = line {
                if ip == START {
                    github_flag = true;
                }
                if github_flag == false {
                    source.push(ip.clone());
                }
                if ip == END {
                    github_flag = false;
                }
            }
        }
    }
    source
}

fn write_hosts(path: &str, mut github_ip: Vec<String>) {
    let mut source = clear_history_hosts(path);

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .unwrap();

    source.append(&mut github_ip);
    let mut new_string = String::new();
    for line in source {
        line.chars().for_each(|c| {
            new_string.push(c);
        });
        new_string.push('\n');
    }
    // println!("{:?}", new_string);
    file.write_all(new_string.as_bytes());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[test]
fn test() {
    let client = reqwest::blocking::Client::new();
    match client
        .get("https://www.ipaddress.com/site/github-production-release-asset-2e65be.s3.amazonaws.com")
        .header(USER_AGENT, rand_user_agent::UserAgent::random().to_string())
        .header(REFERER, "https://www.ipaddress.com/")
        .send()
    {
        Ok(response) => {
            // is available via the hostname that resolves the IP address 140.82.113.3.
            let re = Regex::new(r"<li>(\d+\.\d+\.\d+\.\d+)<\/li>").unwrap();
            let text = response.text().unwrap();
            for cap in re.captures_iter(&text) {
                println!("--------{:?}", &cap);
                println!("====================={}", &cap[1]);
            }
            // println!("{}", text);
            // response.text()
        }
        Err(e) => {
            println!("{}", e.to_string())
        }
    }
}
