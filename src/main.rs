//! summit - a bare-bones static HTTP server

use std::env;
use std::fs;
use std::io::prelude::*;
use std::net;
use std::thread;

const DEFAULT_PORT:  u32  = 8080;
const DEFAULT_FILE:  &str = "index.html";
const DEBUG:         bool = false;

/// Serves files in the local directory using HTTP at the port informed as
/// argument. In case missing, the DEFAULT_PORT is used. Each request is served
/// by a specific separate thread.
fn main() {

  let port = {
    let arguments: Vec<String> = env::args().collect();
    match arguments.get(1) {
      None       => DEFAULT_PORT,
      Some(arg)  => arg.parse().expect("Argument must be integer"),
  }};

  let listener = {
    let address = format!("0.0.0.0:{}", port);
    net::TcpListener::bind(address).expect("Failure to listen")
  };

  eprintln!("Serving files in current directory via HTTP using port: {}", port);

  for stream in listener.incoming() {
    let stream = stream.expect("Failure to read stream");
    thread::spawn(move || { handle_connection(stream); });
  }
}

/// Serves local files as per URLs in the TCP requests. In case associated path
/// ends in '/' or is a directory, the DEFAULT_FILE will be appended to the path
/// instead. In case of any failure in opening the file, an 404 Not Found error
/// will be returned. No other error code is provided.
fn handle_connection(mut stream: net::TcpStream) {

  let request_content = {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    String::from_utf8_lossy(&buffer[..]).to_string()
  };

  let url = match request_content.split_whitespace().nth(1) {
    Some(url) => url.to_string(),
    None      => return,
  };

  let path = if url.chars().last().unwrap() == '/' {
    format!(".{}{}", url, DEFAULT_FILE)

  } else if fs::metadata(format!(".{}", url)).is_ok() &&
            fs::metadata(format!(".{}", url)).unwrap().is_dir() {
      format!(".{}/{}", url, DEFAULT_FILE)

  } else {
    format!(".{}", url)
  };

  let (http_result, file_contents) = match fs::read(&path) {
    Ok(file) => ("200 OK",        file       ),
    Err(_)   => ("404 Not Found", Vec::new() ),
  };

  if DEBUG {
    eprintln!(" {}: {} = {} => {}",
      stream.peer_addr().unwrap(), url, path, http_result);
  }

  let response = [
    format!("HTTP/1.1 {}\r\n\r\n", http_result).as_bytes().to_vec(),
    file_contents].concat();

  stream.write(&response).expect("Failure sending response");
  stream.flush().expect("Failure flushing response");
}
