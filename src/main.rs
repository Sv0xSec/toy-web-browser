use crate::url::URL;

pub mod url;

fn main() {
    let url = URL::new("http://example.org:80/index.html").unwrap();
    println!("{:?}", url);

    let body = url.request();
    show(body);
}

fn show(body: String) -> () {
    let mut in_angle = false;
    for c in body.chars() {
        match c {
            '<' => in_angle = true,
            '>' => in_angle = false,
            _ if in_angle == false => print!("{c}"),
            _ => (),
        }
    }
}
