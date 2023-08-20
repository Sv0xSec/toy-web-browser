use std::fmt;
use std::net::{TcpStream, ToSocketAddrs};
use std::io::{Write, Read};
use httparse::{Response, EMPTY_HEADER};

#[derive(Debug)]
pub struct URL<'a> {
    scheme: &'a str,
    host: &'a str,
    path: &'a str
}

impl URL<'_> {
    pub fn new(url: &str) -> Result<URL, UrlError> {
        // Check '://' pattern is present once in url and split the latter or raise an error
        let (url_scheme, url_without_scheme) = if url.contains("://") {
            let url_collection = url.split("://").collect::<Vec<&str>>();
            // Check if pattern appears multiple time
            if url_collection.len() == 2 {
                (url_collection[0], url_collection[1])
            } else {
                return Err(UrlError { message: String::from("Parsing error") });
            }
        } else {
            return Err(UrlError { message: String::from("'://' not detected") });
        };

        //Get the host part of the url and its length
        let (url_host, host_length) = if url_without_scheme.contains("/") {
            let h = url_without_scheme.split("/").collect::<Vec<&str>>()[0];
            (h, h.len())
        } else {
            (url_without_scheme, url_without_scheme.len())
        };

        //Get the path part of the url
        let url_path = &url_without_scheme[host_length..];

        Ok(URL{
            scheme: url_scheme,
            host: url_host,
            path: url_path,
        })
    }

    pub fn request(&self) -> String {
        //let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP)).unwrap();
        //TO-DO : Error if no port, even 80
        //let address : SockAddr = if let Some(socket_address) = self.host.to_socket_addrs().unwrap().next() {
        //    socket_address.into()
        //} else {
        //    panic!("Hostname could not be resolved");
        //};
        //println!("{:?}", address);
        //socket.connect(&address).unwrap();
        //socket.send(String::from(format!("GET {} HTTP/1.0\r\nHost: {}\r\n\r\n", self.path, self.host)).as_bytes()).unwrap();
        //let mut response = [MaybeUninit::uninit(); 1024];
        //let response_length = socket.recv(&mut response).unwrap();
        //let transmuted = unsafe { transmute::<_, [u8; 1024]>(response) };
        //let data = &transmuted[0..response_length];
        //println!("{:?}",data);

        let mut stream = TcpStream::connect(self.host).unwrap();

        let _ = stream.write(String::from(format!("GET {} HTTP/1.0\r\nHost: {}\r\n\r\n", self.path, self.host)).as_bytes());
        let mut raw_response: [u8; 4096] = [0; 4096];
        let resp_length= stream.read(&mut raw_response).unwrap();
        let data = std::str::from_utf8(&raw_response[0..resp_length]).unwrap();

        let collection = data.split("\r\n\r\n").collect::<Vec<&str>>();
        if collection.len() == 2 {
            String::from(collection[1])
        } else {
            panic!("Multiple '\r\n\r\n' pattern");
        }
    }
}

// TO-DO : reimplement based on errors6.rs of rustlings
#[derive(Debug)]
pub struct UrlError {
    message: String
}

impl fmt::Display for UrlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_correctly() {
        let url1 = URL::new("http://localhost:8080").unwrap();
        assert_eq!(url1.scheme, "http");
        assert_eq!(url1.host, "localhost:8080");
        assert_eq!(url1.path, "");

        let url2 = URL::new("http://localhost:8080/path/to/file").unwrap();
        assert_eq!(url2.scheme, "http");
        assert_eq!(url2.host, "localhost:8080");
        assert_eq!(url2.path, "/path/to/file");
    }

    #[test]
    fn error_risen_when_no_scheme() {
        let url_result = URL::new("http:/localhost:8080");
        assert!(url_result.is_err());
    }
}
