#![allow(dead_code)]
#![forbid(unsafe_code)]
use select::document::Document;
use select::predicate::Name;
use std::cmp;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader, Cursor};

const MAX_URLS: usize = 5;

pub(crate) async fn scrape() -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut set: HashSet<String> = HashSet::new();
    let mut urls =
        google_search_url("https://www.google.com/search?q=why+is+it+called+rust").await?;
    for url in urls {
        set.insert(url);
    }
    urls = set.into_iter().collect();
    let mut contents: Vec<Vec<String>> = Vec::new();
    for i in 0..cmp::min(MAX_URLS, urls.len()) {
        let ps = get_contents(&urls[i][..]).await?;
        contents.push(ps);
    }
    Ok(contents)
}

async fn get_html(url: &str) -> Result<Document, Box<dyn Error>> {
    let html = reqwest::get(url).await?.text().await?;
    Ok(Document::from_read(Cursor::new(html))?)
}

async fn google_search_url(url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let html = get_html(url).await?;
    let links: Vec<String> = html
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter(|x| (*x).starts_with("/url?q="))
        .map(str::to_string)
        .filter_map(|x| {
            let tmp: Vec<&str> = (*x).split("/url?q=").collect();
            if tmp[1].len() == 0 || !is_trusted("urls.txt", tmp[1]) {
                return None;
            }
            return Some(tmp[1].to_string());
        })
        .filter(|x| (*x).contains("&sa="))
        .filter_map(|x| {
            let tmp: Vec<&str> = (*x).split("&sa=").collect();
            if tmp[0].len() == 0 {
                return None;
            }
            return Some(tmp[0].to_string());
        })
        .collect();
    Ok(links)
}

fn is_trusted(path: &str, str: &str) -> bool {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let tmp = line.unwrap();
        if str.contains(&tmp[..]) {
            return true;
        }
    }
    false
}

async fn get_urls(starting_url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut slices: Vec<&str> = starting_url.split("https://").collect();
    slices = slices[1].split("/").collect();
    let domain = slices[0];
    if domain.len() == 0 {
        return Ok(Vec::new());
    }
    let html = get_html(starting_url).await?;
    let links: Vec<String> = html
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter(|x| (*x).starts_with("https://"))
        .map(str::to_string)
        .filter(|x| {
            let mut tmp: Vec<&str> = (*x).split("https://").collect();
            tmp = tmp[1].split("/").collect();
            if tmp.len() == 1 || tmp[0].len() == 0 || tmp[1].len() == 0 {
                return false;
            }
            return tmp[0] == domain;
        })
        .collect();
    Ok(links)
}

async fn get_contents(url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let webpage = get_html(url).await?;
    let contents: Vec<String> = webpage
        .find(Name("p"))
        .filter_map(|n| Some(n.text()))
        .filter(|x| x.len() > 0 && !x.trim().is_empty())
        .collect();
    Ok(contents)
}
