extern crate reqwest;
use std::io::{Read, Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};
use std::thread;
use reqwest::Url;

fn main() {
  let mut count = 0_i32;
  let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
  for stream in listener.incoming() {
    count += 1;
    let imCount = count;
    thread::spawn(move || {
      handle_client(stream.unwrap(), imCount);
    });
  }
}

fn foo(s: &mut String) {
  s.push_str(" bar");
}

fn handle_client(mut out: TcpStream, count: i32) {
  let mut get: Option<String> = None;
  let mut cookies: Option<String> = None;
  for header in BufReader::new(&mut out).lines() {
    let header = header.unwrap();
    if header.len() > 4 && header[..4] == "GET "[..] {
      println!("yeah");
      let mut query = String::from(&header[4..]);
      let newLen = query.find(' ').unwrap_or((&query).len());
      query.truncate(newLen);
      get = Some(query);
      println!("get: {:?}", get);
    }
    println!("header: {}", header);
    if header.len() > 8 && header[..9] == "Cookie: "[..] {
      cookies = Some(String::from(&header[..]))
    }
    //res.write(header.as_bytes());
    if header == "" { break }
  }
  if get.is_none() { return; }
  let get = get.unwrap();
  let url = Url::parse(&format!("https://duckduckgo.com{}",&get)[..]).expect("could not parse url");
  let mut res = reqwest::get(url).unwrap();//TODO add cookies
  //out.write("foo bar\n".as_bytes()).expect("failed to write to client1");
  //out.write("foo bar\n".as_bytes()).expect("failed to write to client2");
  out.write("HTTP/1.1 200 OK\ncontent-type:\ttext/html; charset=UTF-8\n\n".as_bytes()).expect("failed to write to client2");
  //let mut buffer = [0; 100512];

  for line in BufReader::new(&mut res).lines() {
    let line: String = line.unwrap();
    println!("line: {}", line.replace("href=\"/", "href=\"https://duckduckgo.com/").replace("src=\"/", "src=\"https://duckduckgo.com/"));
    //if header == "" { break }
    out.write(line.replace("href=\"/", "href=\"https://duckduckgo.com/").replace("src=\"/", "src=\"https://duckduckgo.com/").as_bytes());
  }

  //::std::io::copy(&mut res, &mut out).expect("copy failed");
  
  /*let mut buffer = [0; 512];
  res.read(&mut buffer).unwrap();
  //out.write(&buffer[..]).unwrap();
  out.write("foo bar".as_bytes()).expect("failed to write to client");
  out.flush().
 */
  println!("count: {}", count);
}

fn load_ddg(query: &str) {
  
}
