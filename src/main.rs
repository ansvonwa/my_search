extern crate reqwest;
extern crate regex;
use std::io::{Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};
use std::thread;
use reqwest::Url;
use regex::Regex;

fn main() {
  let mut count = 0_i32;
  let listener = TcpListener::bind("127.0.0.1:8080").unwrap();// TODO change back to port 8080 before building
  for stream in listener.incoming() {
    count += 1;
    let im_count = count;
    thread::spawn(move || {
      handle_client(stream.unwrap(), im_count);
    });
  }
}

fn handle_client(mut out: TcpStream, count: i32) {
  let mut get: Option<String> = None;
  let mut cookies: Option<String> = None;
  for header in BufReader::new(&mut out).lines() {
    let header = header.unwrap();
    if header.len() > 4 && header[..4] == "GET "[..] {
      println!("yeah");
      let mut query = String::from(&header[4..]);
      let new_len = query.find(' ').unwrap_or((&query).len());
      query.truncate(new_len);
      get = Some(query);
      println!("get: {:?}", get);
    }
    println!("header: {}", header);
    if header.len() > 8 && header[..9] == "Cookie: "[..] {
      cookies = Some(String::from(&header[..]))
    }
    if header == "" { break }
  }
  if get.is_none() { return; }
  let get = get.unwrap();

  let yt_re = Regex::new("(%21yt%21|yt%21%21|%21%21yt)").unwrap();
  if yt_re.find(&get[..]).is_some() {
    println!("MATCHED !yt!{}", get);
    if bang_youtube(&out, &yt_re.replace(&get[..], "")[4..]) { return; }
  }
  let url = Url::parse(&format!("https://duckduckgo.com{}",&get)[..]).expect("could not parse url");
  let mut res = reqwest::get(url).unwrap();//TODO add cookies
  out.write("HTTP/1.1 200 OK\ncontent-type:\ttext/html; charset=UTF-8\n\n".as_bytes()).expect("failed to write to client2");

  for line in BufReader::new(&mut res).lines() {
    let line: String = line.unwrap();
    println!("line: {}", line.replace("href=\"/", "href=\"https://duckduckgo.com/").replace("src=\"/", "src=\"https://duckduckgo.com/"));
    out.write(line.replace("href=\"/", "href=\"https://duckduckgo.com/").replace("src=\"/", "src=\"https://duckduckgo.com/").as_bytes()).expect("failed to write search results to client");
  }
  println!("count: {}", count);
}

fn bang_youtube(mut out: &TcpStream, query: &str) -> bool {
  println!("BANG {}", query);
  let watch_id_re = Regex::new("<div class=.yt-lockup-content.><h3 class=.yt-lockup-title .><a href=./watch\\?v=([a-zA-Z0-9-_]+)").unwrap();
  let url = Url::parse(&format!("https://www.youtube.com/results?search_query={}",&query)[..]).expect("could not parse url");
  println!("URL: {}", url);
  let mut res = reqwest::get(url).unwrap();

  for line in BufReader::new(&mut res).lines() {
    let line: String = line.unwrap();
    match watch_id_re.captures(&line[..]) {
      Some(m) => {
          let vid = m.get(1).unwrap();
          println!("MATCH: {:?}", vid.as_str());
          out.write("HTTP/1.1 200 OK\ncontent-type:\ttext/html; charset=UTF-8\n\n".as_bytes()).expect("failed to write to client2");
  
          out.write(&format!("<head><meta http-equiv=\"refresh\" content=\"0; url=https://youtube.com/watch?v={}\" /></head><body><p><a href=\"https://youtube.com/watch?v={}\">Redirect</a></p></body>", vid.as_str(), vid.as_str()).as_bytes());
          println!("{}", &format!("<head><meta http-equiv=\"refresh\" content=\"0; url=https://youtube.com/watch?v={}\" /></head><body><p><a href=\"https://youtube.com/watch?v={}\">Redirect</a></p></body>", vid.as_str(), vid.as_str()));
          return true;
        },
      None => {},
    }
  }
  /*
  println!("line: {}", line.replace("href=\"/", "href=\"https://duckduckgo.com/").replace("src=\"/", "src=\"https://duckduckgo.com/"));
  out.write(line.replace("href=\"/", "href=\"https://duckduckgo.com/").replace("src=\"/", "src=\"https://duckduckgo.com/").as_bytes()).expect("failed to write search results to client");
  
  out.write("HTTP/1.1 200 OK\ncontent-type:\ttext/html; charset=UTF-8\n\n".as_bytes()).expect("failed to write to client2");
  
  out.write("<head><meta http-equiv=\"refresh\" content=\"0; url=http://example.com/\" /></head><body><p><a href=\"http://example.com/\">Redirect</a></p></body>".as_bytes());
  */
  return false;
}
