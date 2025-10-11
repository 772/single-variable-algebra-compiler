fn main() {
    output("TEST".to_string());
}

#[cfg(not(target_arch = "wasm32"))]
fn output(s: String) {
    println!("{s}");
}

#[cfg(target_arch = "wasm32")]
fn output(s: String) {
    use web_sys::wasm_bindgen::JsCast;
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let textarea = document
        .get_element_by_id("input")
        .unwrap()
        .dyn_into::<web_sys::HtmlTextAreaElement>()
        .unwrap();
    textarea.set_value(&s);
}
