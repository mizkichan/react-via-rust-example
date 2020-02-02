use js_sys::{Function, Object, Reflect};
use wasm_bindgen::prelude::*;
use web_sys::{console, Node};

pub mod react {
    use super::*;

    #[wasm_bindgen(module = "react")]
    extern "C" {
        pub static Fragment: JsValue;

        #[wasm_bindgen(js_name = createElement, variadic)]
        pub fn create_element(
            type_: &JsValue,
            props: &JsValue,
            children: Box<[JsValue]>,
        ) -> JsValue;

        #[wasm_bindgen(js_name = useState)]
        pub fn use_state(initial_state: &JsValue) -> Box<[JsValue]>;
    }
}

pub mod react_dom {
    use super::*;

    #[wasm_bindgen(module = "react-dom")]
    extern "C" {
        pub fn render(element: &JsValue, container: &Node);
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_log("Hello, world!");

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let container = body.append_child(document.create_element("div")?.as_ref())?;

    let closure = Closure::wrap(Box::new(app) as Box<dyn Fn() -> _>);
    react_dom::render(
        &react::create_element(closure.as_ref(), &JsValue::NULL, Box::from(Vec::new())),
        &container,
    );
    closure.forget();

    Ok(())
}

fn app() -> JsValue {
    let mut x = Vec::from(react::use_state(&JsValue::from(0)));
    let set_state1 = Function::from(x.pop().unwrap());
    let set_state2 = set_state1.clone();
    let state = (x.pop().unwrap()).as_f64().unwrap();

    react::create_element(
        &react::Fragment,
        &JsValue::NULL,
        Box::from(vec![
            button("-", move || {
                set_state1
                    .call1(&JsValue::NULL, &JsValue::from(state - 1.0))
                    .unwrap();
            }),
            JsValue::from(state),
            button("+", move || {
                set_state2
                    .call1(&JsValue::NULL, &JsValue::from(state + 1.0))
                    .unwrap();
            }),
        ]),
    )
}

fn button<F>(value: &str, on_click: F) -> JsValue
where
    F: Fn() + 'static,
{
    let props = Object::new();
    let closure = Closure::once_into_js(Box::new(on_click) as Box<dyn Fn()>);
    Reflect::set(&props, &JsValue::from("onClick"), &closure).unwrap();

    react::create_element(
        &JsValue::from("button"),
        &props,
        Box::from(vec![JsValue::from(value)]),
    )
}

fn console_log<T>(s: T)
where
    T: Into<JsValue>,
{
    console::log_1(&s.into())
}
