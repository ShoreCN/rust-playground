use futures::executor::block_on;

// 通过async异步地进行网页下载请求
async fn parse_html(html: String) -> String {
    println!("parse html: {}", html);
    format!("parse result of {}", html)
}

async fn download_web(url: &str) -> String{
    println!("begin to download web: {}", url);
    let content = format!("content of {}", url);
    let r = parse_html(content).await;
    println!("download task result: {}", r);
    r
}

async fn download_task() {
    let d1 = download_web("https://www.baidu.com");
    let d2 = download_web("https://www.google.com");

    futures::join!(d1, d2);
}

pub fn async_code() {
    block_on(download_task());
}