use std::fs::{create_dir_all, File, metadata};
use std::io::{Write};
use std::path::Path;
use reqwest::Response;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub async fn handle_response(mut res: Response, remote_name: Option<String>) {
    let stdout = StandardStream::stdout(ColorChoice::Auto);
    let mut stdout = stdout.lock();
    let mut buf = Vec::new();

    // Print status
    buf.clear();
    write!(buf, "[Status]: {}", res.status()).unwrap();
    let status_color = if res.status().is_success() {
        Color::Green
    } else {
        Color::Red
    };
    stdout.set_color(ColorSpec::new().set_fg(Some(status_color))).unwrap();
    stdout.write_all(&buf).unwrap();
    writeln!(&mut stdout).unwrap();

    // Print headers
    buf.clear();
    write!(buf, "[Headers]: {:#?}", res.headers()).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).unwrap();
    stdout.write_all(&buf).unwrap();
    writeln!(&mut stdout).unwrap();

    // Print content length
    if let Some(content_length) = res.content_length() {
        buf.clear();
        write!(buf, "[Content Length]: {:#?}", content_length).unwrap();
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan))).unwrap();
        stdout.write_all(&buf).unwrap();
        writeln!(&mut stdout).unwrap();
    }

    // Print remote address
    buf.clear();
    write!(buf, "[Remote Address]: {:#?}", res.remote_addr()).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta))).unwrap();
    stdout.write_all(&buf).unwrap();
    writeln!(&mut stdout).unwrap();

    // Print body or save to file
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
        buf.clear();
        write!(buf, "[Body]:").unwrap();
        while let Some(chunk) = res.chunk().await.expect("响应失败") {
            if let Ok(utf8_string) = String::from_utf8(Vec::from(chunk.clone())) {
                write!(buf, "{}", utf8_string).unwrap();
            } else {
                write!(buf, "{}", String::from_utf8_lossy(&chunk)).unwrap();
            }
        }
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::White))).unwrap();
        stdout.write_all(&buf).unwrap();
    }
}
