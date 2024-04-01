use std::net::TcpListener;
use std::io::{Read,Write};
use crate::http::{Request,Response,StatusCode, ParseError};
use std::convert::TryFrom;

pub trait Handler{
    fn handle_request(&mut self, request: &Request) ->Response;
    fn handle_bad_request(&mut self, e:&ParseError) ->Response{
        println!("Failed to parse a request: {}",e);
        Response::new(StatusCode::BadRequest,None)
    }
}

pub struct Server {
        address: String,
    }
impl Server {   
        pub fn new(address:String) -> Self{
            Self{
                address
            }
        }
        pub fn run(&self, mut handler: impl Handler){
            println!("Listening on : {}", self.address);

            let listener = TcpListener::bind(&self.address).unwrap();

            loop{
                let res = listener.accept();

                match res{
                    Ok((mut stream,_)) => {
                        let mut buffer = [0;1024];
                        match stream.read(&mut buffer){
                            Ok(_) =>{
                                println!("Recieved request: {}",String::from_utf8_lossy(&buffer));

                                let response = match Request::try_from(&buffer[..]){    
                                    Ok(request) => handler.handle_request(&request),
                                    Err(e) => handler.handle_bad_request(&e),
                                };
                                if let Err(e) = response.send(&mut stream){
                                    println!("Failed to send response: {}",e);
                                }
                            }
                            Err(e) => println!("Error: Failed to read from the connection {}",e),
                        }
                    }
                    Err(err) => println!("error {}", err),
                }
            }
    
        }
        
}