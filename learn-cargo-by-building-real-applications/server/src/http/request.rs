use super::method::Method;

pub struct Request {
    path: String,
    quesry_string: Option<String>,
    method: Method,
}