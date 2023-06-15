use tl::{queryselector, Parser, ParserOptions, VDom};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let http_client = build_http_client()?;
    //we are searching for the word "access" here.
    let html = http_client
        .get("http://tratu.coviet.vn/hoc-tieng-anh/tu-dien/lac-viet/A-V/access.html")
        .send()
        .await?
        .text_with_charset("utf-8")
        .await?;

    let dom = get_dom(&html)?;
    let pronun = extract_pronunciation(&dom, "div.p5l.fl.cB");
    println!("{:?}", pronun);

    Ok(())
}

fn get_dom(html: &str) -> anyhow::Result<VDom> {
    let dom = tl::parse(&html, ParserOptions::default())?;
    Ok(dom)
}

fn extract_pronunciation(dom: &VDom, query_selector: &str) -> Option<String> {
    let query_result = dom.query_selector(query_selector);
    if query_result.is_none() {
        return None;
    }

    for node_handle in query_result.unwrap() {
        if let Some(node) = node_handle.get(dom.parser()) {
            return Some(node.inner_text(dom.parser()).to_string());
        }
    }

    return None;
}

fn build_http_client() -> reqwest::Result<reqwest::Client> {
    let client_builder = reqwest::ClientBuilder::new().gzip(true);
    let client = client_builder.build();
    client
}
