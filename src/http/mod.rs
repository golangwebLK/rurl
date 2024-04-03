mod response;
mod builder;

pub async fn run(
    url: String,
    method: Option<reqwest::Method>,
    header: Option<Vec<String>>,
    data: Option<String>,
    form: Option<Vec<String>>,
    remote_name: Option<String>,
) {
    let res = builder::RequestBuilder::new(url)
        .method(method)
        .header(header)
        .form(form)
        .data(data)
        .build()
        .send()
        .await
        .expect("请求错误");
    response::handle_response(res, remote_name).await;
}










