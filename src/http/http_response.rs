use std::collections::HashMap;

// HTTP/1.1 200 OK
// Bdpagetype: 1
// Bdqid: 0x87a3a3fe0006909b
// Connection: keep-alive
// Content-Encoding: gzip
// Content-Security-Policy: frame-ancestors 'self' https://chat.baidu.com http://mirror-chat.baidu.com https://fj-chat.baidu.com https://hba-chat.baidu.com https://hbe-chat.baidu.com https://njjs-chat.baidu.com https://nj-chat.baidu.com https://hna-chat.baidu.com https://hnb-chat.baidu.com http://debug.baidu-int.com;
// Content-Type: text/html; charset=utf-8
#[derive(Debug)]
pub struct Response {
    pub(crate) protocol: String,
    pub(crate) code: u32,
    pub(crate) codeDesc: String,
    pub(crate) head: HashMap<String, String>,
    pub(crate) content: String,
}

impl Response {
    // fn new(protocol: &str, code: u32, codeDesc: &str) -> Response {
    //     // let protocol = protocol.to_string();
    //     // let codeDesc = codeDesc.to_string();
    //     let mut protocol = protocol.trim().parse().unwrap();
    //     if protocol.is_empty() {
    //         protocol = "HTTP/1.1".to_string();
    //     }
    //     let codeDesc = codeDesc.trim().parse().unwrap();
    //     Response {
    //         protocol,
    //         code,
    //         codeDesc,
    //         head: Default::default(),
    //     }
    // }

    pub fn add_header(&mut self, k: &str, v: &str) -> u32 {
        self.head.insert(k.parse().unwrap(), v.parse().unwrap());
        return 0;
    }

    pub fn dump(&mut self) -> String {
        let mut res: String = String::new();
        // HTTP/1.1 200 OK
        // Bdpagetype: 1
        // Bdqid: 0x87a3a3fe0006909b
        // Connection: keep-alive
        // Content-Encoding: gzip
        if (self.protocol.is_empty()) {
            self.protocol = "HTTP/1.1".to_string();
        }

        res.push_str(self.protocol.as_str());
        res.push_str(" ");
        res.push_str(self.code.to_string().as_str());
        res.push_str(" ");
        res.push_str(self.codeDesc.as_str());
        res.push_str("\n");

        if !self.head.contains_key("Content-Length") {
            self.head.insert("Content-Length".to_string(), content.len().to_string());
        }

        for (k, v) in self.head.iter() {
            res.push_str(k);
            res.push_str(": ");
            res.push_str(v);
            res.push_str("\n");
        }

        res.push_str("\n");
        if !self.content.is_empty() {
            res.push_str(self.content.as_str());
        }

        return res.to_string();
    }
    pub fn set_protocol(&mut self, protocol: String) {
        self.protocol = protocol;
        if self.protocol.is_empty() {
            self.protocol = "HTTP/1.1".to_string();
        }
    }
    pub fn set_code(&mut self, code: u32) {
        self.code = code;
    }
    pub fn set_codeDesc(&mut self, codeDesc: String) {
        self.codeDesc = codeDesc;
    }
    pub fn set_head(&mut self, head: HashMap<String, String>) {
        self.head = head;
    }
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }
}