use clap::Parser;
use reqwest::{Method};
use rurl::http;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(required = true)]
    url: String,

    #[arg(short = 'X', long = "method")]
    method: Option<Method>,

    #[arg(short = 'H', long = "header")]
    header: Option<Vec<String>>,

    #[arg(short = 'd', long = "data")]
    data: Option<String>,

    #[arg(short = 'F', long = "form")]
    form: Option<Vec<String>>,

    #[arg(short = 'O', long = "remote-name")]
    remote_name: Option<String>,
}


#[tokio::main]
async fn main(){
    let args = Args::parse();
    http::run(
        args.url,
        args.method,
        args.header,
        args.data,
        args.form,
        args.remote_name
    ).await;
}
