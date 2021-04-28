use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[macro_use]
extern crate lazy_static;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CompanyResponse {
    code: u16,
    status: String,
    total: u32,
    data: Vec<Company>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Company {
    pub name: String,
    pub email: String,
    pub vat: String,
    pub phone: String,
    pub country: String,
    pub website: String,
    pub image: String,
    pub contact: Contact,
    pub addresses: Vec<Address>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    #[serde(alias = "firstname")]
    pub first_name: String,
    #[serde(alias = "lastname")]
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub birthday: String,
    pub gender: String,
    pub address: Address,
    pub website: String,
    pub image: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    #[serde(rename = "streetName")]
    pub street_name: String,
    #[serde(rename = "buildingNumber")]
    pub building_number: String,
    pub city: String,
    pub zipcode: String,
    pub country: String,
    pub county_code: String,
    pub latitude: f32,
    pub longitude: f32,
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

// lazy_static! {
//     static ref company_res: Mutex<Vec<CompanyResponse>> = Mutex::new(Vec::new());
// }

#[wasm_bindgen]
pub async fn get_json(quantity: i32) -> Result<JsValue, JsValue> {
    init_panic_hook();
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!(
        "https://fakerapi.it/api/v1/companies?_quantity={}",
        quantity
    );

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

    // Use serde to parse the JSON into a struct.

    // very slow, https://github.com/rustwasm/wasm-bindgen/issues/1205#issuecomment-457648506
    let company_res: CompanyResponse = json.into_serde().unwrap();
    // Send the `Branch` struct back to JS as an `Object`.
    Ok(JsValue::from_serde(&company_res).unwrap())

    // company_res.lock().unwrap().push(json.into_serde().unwrap());
    // Ok(JsValue::from_serde(&company_res.lock().unwrap()[0]).unwrap())
}

#[wasm_bindgen]
pub async fn get_str(quantity: i32) -> Result<JsValue, JsValue> {
    init_panic_hook();
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!(
        "https://fakerapi.it/api/v1/companies?_quantity={}",
        quantity
    );

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
    let text = JsFuture::from(resp.text()?).await?;

    Ok(text)
}
