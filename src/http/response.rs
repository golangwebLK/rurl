use std::fs::{create_dir_all, File, metadata};
use std::io::Write;
use std::path::Path;

pub async fn handle_response(mut res: reqwest::Response, remote_name: Option<String>) {
    println!("status: {:#?}", res.status());
    println!("headers: {:#?}", res.headers());
    println!(
        "content_length: {:#?}",
        res.content_length().expect("文本长度获取失败")
    );
    println!(
        "remote_addr: {:#?}",
        res.remote_addr().expect("远程地址获取失败")
    );
    println!("body:");
    if let Some(remote_name) = remote_name {
        let file_path = Path::new(&remote_name);
        let directory = file_path.parent().unwrap();
        let dir_path = directory.to_str().unwrap();
        if metadata(dir_path).is_err() {
            create_dir_all(dir_path).expect("路径创建失败");
        }
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        let mut file = File::create(file_name).unwrap();
        while let Some(chunk) = res.chunk().await.expect("响应失败") {
            file.write_all(&chunk).unwrap();
        }
    } else {
        while let Some(chunk) = res.chunk().await.expect("响应失败") {
            if let Ok(utf8_string) = String::from_utf8(Vec::from(chunk.clone())) {
                println!("{:#?}", utf8_string);
            } else {
                println!("{:#?}", String::from_utf8_lossy(&chunk));
            }
        }
    }
}