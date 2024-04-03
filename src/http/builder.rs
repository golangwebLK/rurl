use std::borrow::Cow;
use std::fs::read;
use std::path::Path;
use reqwest::{Client, Method, multipart};
use reqwest::multipart::Part;

pub struct RequestBuilder {
    client: Client,
    method: Method,
    url: String,
    data: Option<String>,
    form: Option<Vec<String>>,
    header: Option<Vec<String>>,
}

impl RequestBuilder {
    pub fn new(url: String) -> Self {
        Self {
            client: Client::new(),
            method: Method::GET,
            url,
            data: None,
            form: None,
            header: None,
        }
    }

    pub fn method(mut self, method: Option<Method>) -> Self {
        if let Some(m) = method{
            self.method = m;
        }
        self
    }

    pub fn data(mut self, data: Option<String>) -> Self {
        self.data = data;
        self
    }

    pub fn form(mut self, form: Option<Vec<String>>) -> Self {
        self.form = form;
        self
    }

    pub fn header(mut self, header: Option<Vec<String>>) -> Self {
        self.header = header;
        self
    }
    pub fn build(self) -> reqwest::RequestBuilder {
        let mut request_builder = self.client.request(self.method.clone(), &self.url);

        if let Some(data) = &self.data {
            request_builder = request_builder
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(data.clone());
        }

        if let Some(form) = &self.form {
            request_builder = request_builder.header("Content-Type", "multipart/form-data");
            let form_data = parse_form_data(form);
            let forms = form_data.into_iter().fold(multipart::Form::new(), |data, field| {
                match field {
                    FormData::Text(key, value) => data.text(key, value),
                    FormData::File(key, file_path) => {
                        let file = read_file(&file_path);
                        let filename = Path::new(&file_path)
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string();
                        data.part(key, Part::bytes(Cow::from(file)).file_name(filename))
                    }
                }
            });
            request_builder = request_builder.multipart(forms);
        }

        if let Some(header) = &self.header {
            for item in header {
                let parts: Vec<&str> = item.split(':').collect();
                if parts.len() == 2 {
                    request_builder = request_builder.header(parts[0].trim(), parts[1].trim());
                } else {
                    panic!("header格式错误: {}", item);
                }
            }
        }

        request_builder
    }
}





enum FormData {
    Text(String, String),
    File(String, String),
}

fn parse_form_data(form: &[String]) -> Vec<FormData> {
    form.iter()
        .flat_map(|s| s.split('&'))
        .map(|s| {
            let parts: Vec<_> = s.splitn(2, '=').collect();
            if parts.len() == 2 {
                if let Some(file_path) = parts[1].strip_prefix('@') {
                    FormData::File(parts[0].to_owned(), file_path.trim().to_owned())
                } else {
                    FormData::Text(parts[0].to_owned(), parts[1].to_owned())
                }
            } else {
                panic!("表单字段格式错误: {}", s);
            }
        })
        .collect()
}


fn read_file(file_path: &str) -> Vec<u8> {
    let path = Path::new(file_path);
    if path.exists() {
        read(path).expect("文件读取失败")
    } else {
        panic!("文件路径错误: {}", file_path);
    }
}