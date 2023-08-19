use std::fmt;

#[derive(Debug)]
pub struct URL<'a> {
    scheme: &'a str,
    host: &'a str,
    path: &'a str
}

impl URL<'_> {
    pub fn new(url: &str) -> Result<URL, UrlError> {
        let (url_scheme, url_without_scheme) = if url.contains("://") {
            let url_collection = url.split("://").collect::<Vec<&str>>();
            if url_collection.len() == 2 {
                (url_collection[0], url_collection[1])
            } else {
                return Err(UrlError { message: String::from("Parsing error") });
            }
        } else {
            return Err(UrlError { message: String::from("'://' not detected") });
        };

        let (url_host, host_length) = if url_without_scheme.contains("/") {
            let h = url_without_scheme.split("/").collect::<Vec<&str>>()[0];
            (h, h.len())
        } else {
            (url_without_scheme, url_without_scheme.len())
        };

        let url_path = &url_without_scheme[host_length..];

        Ok(URL{
            scheme: url_scheme,
            host: url_host,
            path: url_path,
        })
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
