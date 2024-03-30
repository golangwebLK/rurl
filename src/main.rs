use clap::Parser;
use reqwest::{Method};

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
}


#[tokio::main]
async fn main(){
    let args = Args::parse();
    run(args.url, args.method, args.header, args.data).await;
}

async fn run(
    url: String,
    method: Method,
    header: Option<Vec<String>>,
    data: Option<String>
){
    let client = reqwest::Client::new();
    let mut request_builder = client
        .request(method, url);
    if let Some(data) = data{
        request_builder = request_builder
            .header("Content-Type","application/x-www-form-urlencoded");
        request_builder = request_builder.body(data);
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