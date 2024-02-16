mod utils;

mod plot;

// use crate::plot::plot_stock_quotes;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, stockr!");
}

#[wasm_bindgen]
pub async fn run(symbol: String) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!("http://127.0.0.1:8080/stock/{}", symbol);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    // Send the JSON response back to JS.
    Ok(json)
}


#[wasm_bindgen]
pub async fn run2(repo: String) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!("https://api.github.com/repos/{}/branches/master", repo);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    // Send the JSON response back to JS.
    Ok(json)
}
// #[wasm_bindgen]
// pub async fn analyze_and_plot_stock(symbol: String) -> Result<JsValue, JsValue> {
//     let analysis = // call function

//     let reg_quotes_to_plot = analysis.get_regular_quotes_for_plot();
//     let vol_quotes_to_plot = analysis.get_volatile_quotes_for_plot();

//     if let Err(e) = plot_stock_quotes(
//         &symbol,
//         &reg_quotes_to_plot,
//         &vol_quotes_to_plot,
//         analysis.start_date,
//         analysis.end_date,
//     ) {
//         eprintln!("Error generating plot: {}", e);
//     }

//     Ok(JsValue::from_str(&format!("Analysis and plot successful")))
// }