use std::fmt;

#[derive(Debug)]
pub struct URL<'a> {
    scheme: &'a str,
    host: &'a str,
    path: &'a str
}

impl URL<'_> {
    fn new(url: &str) -> Result<URL, UrlError> {
        // Check if url has a correct format
        if url.contains("://") {
            let url_collection = url.split("://").collect::<Vec<&str>>();
            Ok(URL {
                scheme: url_collection[0],
                host: "",
                path: ""
            })
        } else {
            Err(UrlError { message: String::from("No scheme detected") })
        }
    }
}


#[derive(Debug)]
struct UrlError {
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
    fn get_correct_scheme() {
        let url = URL::new("http://localhost:8080").unwrap();
        assert_eq!(url.scheme, "http");
    }

    #[test]
    fn error_risen_when_no_scheme() {
        let url_result = URL::new("http:/localhost:8080");
        assert!(url_result.is_err());
    }
}
