use super::method::{Method, MethodError};
use std::str::{self, Utf8Error};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display,Formatter,Debug,Result as FmtResult};
use super::{QueryString,QueryStringValue};

#[derive(Debug)]
pub struct Request<'buf>{
    path: &'buf str,
    query_string:Option<QueryString<'buf>>,
    method:Method,
}


impl <'buf> Request<'buf> {
    pub fn path(&self) -> &str{
        &self.path
    }
    pub fn method(&self) -> &Method{
        &self.method
    }
    pub fn query_string(&self) -> Option<&QueryString>{
        self.query_string.as_ref()
    }
}

impl<'buf> TryFrom<&'buf[u8]> for Request<'buf> {
    type Error = ParseError;
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(buf)?;
        //By tradidtional approach we can convert the
        //returned option from get_next_word() to Result
        //with help of match 
        //BUT we have a build in method ok_or() to do same it transforms Option<> to Result<>
        
        
        /*
        match get_next_word(request){
            Some((method,request))=>{},
            None => return Err(ParseError::InvalidRequest)
        }
        */
        let (method,request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path,request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol,_) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1"{
            return Err(ParseError::InvalidProtocol);
        }
        let method:Method = method.parse()?;

        let mut query_string = None;
        
        if let Some(i) = path.find('?'){
            query_string = Some(QueryString::from(&path[i+1..]));
            path = &path[..i];
        }
        
        Ok(Self{
            path,
            query_string,
            method,
        })
        


    }
}
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i,c) in request.chars().enumerate(){
        if c == ' ' || c == '\r'{
            return Some((&request[..i],&request[i+1..]))
        }
    }
    None
}
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self{
            Self::InvalidRequest => "Invalid request",
            Self::InvalidEncoding=> "Invalid encoding",
            Self::InvalidProtocol=> "Invalid protocol",
            Self::InvalidMethod=> "Invalid method",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
        write!(f, "Parse error{}",self.message())
    }
}
impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
        write!(f, "Parse error{}",self.message())
    }
}

impl Error for ParseError {

}