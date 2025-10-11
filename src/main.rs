use web_sys::console;

fn main() {
    output();
}

#[cfg(not(target_arch = "wasm32"))]
fn output() {
    println!("{s}");
}

#[cfg(target_arch = "wasm32")]
fn output() {
	console::log_1(&"Hello, world!".into());
}
	
/*    use web_sys::wasm_bindgen::JsCast;
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let textarea = document
        .get_element_by_id("input")
        .unwrap()
        .dyn_into::<web_sys::HtmlTextAreaElement>()
        .unwrap();
    textarea.set_value(&s);
*/
