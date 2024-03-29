use js_sys::Boolean;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use yew::prelude::*;
use web_sys::{HtmlVideoElement, MediaDevices, MediaStream, MediaStreamConstraints, Navigator, window};

#[function_component(Producer)]
fn producer() -> Html {
    wasm_bindgen_futures::spawn_local(async move {
        let navigator: Navigator = window().unwrap().navigator();
        let media_devices: MediaDevices = navigator.media_devices().unwrap();
        let video_element = window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("webcam")
            .unwrap()
            .unchecked_into::<HtmlVideoElement>();
        let mut constraints = MediaStreamConstraints::new();
        constraints.video(&Boolean::from(true));
        let devices_query = media_devices
            .get_user_media_with_constraints(&constraints)
            .unwrap();
        let device = JsFuture::from(devices_query)
            .await
            .unwrap()
            .unchecked_into::<MediaStream>();
        video_element.set_src_object(Some(&device))
    });
    html!(
        <div class="producer">
            <h3>{"Producer"}</h3>
            <video id="webcam"></video>
        </div>
    )
}

#[function_component(Consumer)]
fn consumer() -> Html {
    html!(
        <div class="consumer">
            <h3>{"Consumer"}</h3>
        </div>
    )
}


#[function_component(App)]
fn app() -> Html {
    html!(
        <div class="grid">
            <Producer />
            <Consumer />
        </div>
    )
}

fn main() {
    yew::start_app::<App>();
}