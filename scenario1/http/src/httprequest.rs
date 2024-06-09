use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}
impl HttpRequest {
    fn process_req_line(s: &str) -> Option<(Method, Resource, Version)> {
        let mut words = s.split_whitespace();
        let method: Method = words.next()?.into();
        let res: Resource = Resource::Path(words.next()?.to_string());
        let version: Version = words.next()?.into();

        Some((method, res, version))
    }
    fn process_header_line(s: &str) -> Option<(String, String)> {
        let mut words = s.split(':');
        let key = words.next()?.to_string();
        let value = words.next()?.to_string();
        Some((key, value))
    }
}
impl From<String> for HttpRequest {
    fn from(value: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        for line in value.lines() {
            if line.contains("HTTP") {
                // 요청 행이라면
                let (method, resource, version) = HttpRequest::process_req_line(line).unwrap();
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                // 헤더 행이라면
                let (key, value) = HttpRequest::process_header_line(line).unwrap();
                parsed_headers.insert(key, value);
            } else {
                // 바디 행이라면
                parsed_msg_body = line;
            }
        }
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}
impl From<&str> for Method {
    fn from(value: &str) -> Self {
        let s = value.trim().to_uppercase();
        match s.as_str() {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitalized,
}
impl From<&str> for Version {
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitalized,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);

        let m: Method = "POST".into();
        assert_eq!(m, Method::Post);
        let m: Method = "Post".into();
        assert_eq!(m, Method::Post);
        let m: Method = "poSt".into();
        assert_eq!(m, Method::Post);

        let m: Method = "ERROR".into();
        assert_eq!(m, Method::Uninitialized);
    }

    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
        let v: Version = "HTTP/2.0".into();
        assert_eq!(v, Version::V2_0);
        let v: Version = "HTTP/2.2".into();
        assert_eq!(v, Version::Uninitalized);
    }

    #[test]
    fn test_read_http() {
        let s = String::from("POST /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\nbody message");
        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), " localhost".into());
        headers_expected.insert("Accept".into(), " */*".into());
        headers_expected.insert("User-Agent".into(), " curl/7.64.1".into());
        let req: HttpRequest = s.into();

        assert_eq!(Method::Post, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!("body message".to_string(), req.msg_body);
        assert_eq!(headers_expected, req.headers);
    }
}
