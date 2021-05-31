use std::io::{Read};
use std::thread;
use std::net::TcpListener;
use std::fs::{read_to_string};
extern crate clap;
use clap::{Arg, App};
use micro_http::{Request, Response, Body, StatusCode, MediaType, Version};

#[derive(Clone)]
struct ConfigServer {
  server_ip: String,
  default_404: String,
  static_folder:String
}

fn cli() -> std::io::Result<clap::ArgMatches<'static>> {
  let matches = App::new("pima")
    .version("1.0")
    .author("LeoDevSecOps")
    .about("Does awesome things")
    .arg(Arg::with_name("root")
      .short("rf")
      .long("root folder")
      .default_value("/tmp/pima")
      .help("Sets a custom root folder")
      .takes_value(true))
    .arg(Arg::with_name("ip")
      .long("ip")
      .default_value("0.0.0.0")
      .help("Sets a custom ip address"))
    .arg(Arg::with_name("port")
      .long("port")
      .default_value("8080")
      .help("Sets a custom port number"))
    .get_matches();
    Ok(matches)
}

fn get_config(matches: clap::ArgMatches) -> std::io::Result<ConfigServer> {
  let ip = matches.value_of("ip").unwrap();
  let port = matches.value_of("port").unwrap();
  let root_folder = matches.value_of("root").unwrap();

  let server_ip = format!("{}:{}",ip, port);
  let default_404 = read_to_string(format!("{}/404.html", root_folder))?;
  let static_folder = format!("{}/static", root_folder);

  let config = ConfigServer { server_ip: server_ip, default_404: default_404, static_folder: static_folder };
  Ok(config)
}

fn make_response(http_request: micro_http::Request, config: ConfigServer) -> micro_http::Response {

      let body = read_to_string(format!("{}{}.html", config.static_folder, http_request.uri().get_abs_path()));
      let (status, body) = match body {
          Ok(file) => (StatusCode::OK, file),
          Err(_error) => (StatusCode::NotFound, config.default_404),
      };
      let mut response = Response::new(Version::Http11, status);
      response.set_body(Body::new(body.clone()));
      response.set_content_type(MediaType::PlainText);
  response
}

fn main() -> std::io::Result<()> {
  let config: ConfigServer = get_config(cli()?)?;
  let server = TcpListener::bind(config.server_ip.clone())?;
  println!("Server started on URI: {}", config.server_ip);
  for stream in server.incoming() {
    let mut stream = stream?;
    let config = config.clone();
    let _:thread::JoinHandle<std::io::Result<()>> = thread::spawn(move || {
      let mut client_data = [0; 1024];
      stream.read(&mut client_data).unwrap();
      println!("New TCP client {:?}", stream.peer_addr().unwrap());
      let http_request = Request::try_from(&client_data[..]).unwrap();
      let response = make_response(http_request, config); 

      response.write_all(&mut stream).unwrap();
      Ok(())
    });
  }

  Ok(())  
}