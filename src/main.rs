use std::borrow::Cow;
use std::fs::{read};
use std::path::Path;
use clap::Parser;
use reqwest::{Method, multipart};
use reqwest::multipart::Part;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(required = true)]
    url: String,

    #[arg(short = 'X', long = "method")]
    method: Method,

    #[arg(short = 'H', long = "header")]
    header: Option<Vec<String>>,

    #[arg(short = 'd', long = "data")]
    data: Option<String>,

    #[arg(short = 'F', long = "form")]
    form: Option<Vec<String>>,
}


#[tokio::main]
async fn main(){
    let args = Args::parse();
    run(args.url, args.method, args.header, args.data,args.form).await;
}

async fn run(
    url: String,
    method: Method,
    header: Option<Vec<String>>,
    data: Option<String>,
    form: Option<Vec<String>>

){
    let client = reqwest::Client::new();
    let mut request_builder = client
        .request(method, url);
    if let Some(data) = data{
        request_builder = request_builder
            .header("Content-Type","application/x-www-form-urlencoded");
        request_builder = request_builder.body(data);
    }
    if let Some(form) = form {
        request_builder = request_builder
            .header("Content-Type","multipart/form-data");
        let form_data: Vec<_> = form.iter().flat_map(|s| s.split('&')).map(|s|s.to_owned()).collect();
        let data = multipart::Form::new();

        let forms = form_data.into_iter().fold(data, |data, field| {
            let parts: Vec<_>  = field.splitn(2, '=').map(|s|s.to_owned()).collect();
            if parts.len() == 2 {
                if let Some(file_path) = parts[1].strip_prefix('@') {
                    let file_path = file_path.trim();
                    if Path::new(file_path).exists() {
                        let file = read(file_path).expect("文件读取失败");
                        let filename = Path::new(file_path).file_name()
                            .expect("无法获取文件名")
                            .to_string_lossy()
                            .to_string();
                        let part = Part::bytes(Cow::from(file.clone())).file_name(filename);
                        data.part(parts[0].clone(), part)
                    } else {
                        panic!("文件路径错误: {}", file_path);
                    }
                } else {
                    data.text(parts[0].clone(), parts[1].clone())
                }
            }else {
                panic!("表单字段格式错误: {}", field);
            }
        });
        request_builder = request_builder.multipart(forms);
    }

    if let Some(header) = header {
        request_builder = header.iter().fold(request_builder, |builder, item| {
            let parts: Vec<&str> = item.split(':').collect();
            if parts.len() == 2 {
                builder.header(parts[0].trim().to_string(), parts[1].trim().to_string())
            } else {
                panic!("header格式错误: {}", item);
            }
        });
    }

    let mut res = request_builder.send().await.expect("请求错误");
    println!("status: {:#?}",res.status());
    println!("headers: {:#?}",res.headers());
    println!("content_length: {:#?}",res.content_length().expect("文本长度获取失败"));
    println!("remote_addr: {:#?}",res.remote_addr().expect("远程地址获取失败"));
    println!("body:");
    while let Some(chunk) = res.chunk().await.expect("响应失败") {
        if let Ok(utf8_string) = String::from_utf8(Vec::from(chunk.clone())) {
            println!("{:#?}", utf8_string);
        } else {
            println!("{:#?}", String::from_utf8_lossy(&chunk));
        }
    }
}