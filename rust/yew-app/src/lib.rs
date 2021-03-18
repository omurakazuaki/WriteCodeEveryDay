#![recursion_limit="512"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
    num: i64
}

enum Msg {
    Add,
    Minus,
    SetNum(String)
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            num: 1,
            value: 0
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => self.value += self.num,
            Msg::Minus => self.value -= self.num,
            Msg::SetNum(v) => self.num = v.parse().unwrap()
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <div class="m-3">
                    <input type="text", class="form-control", oninput=self.link.callback(|i: InputData| Msg::SetNum(i.value)), value={&self.num},/>
                </div>
                <div class="btn-group m-3">
                    <button type="button" class="btn btn-primary btn-lg", onclick=self.link.callback(|_| Msg::Add)>{ "+" }</button>
                    <button type="button" class="btn btn-danger btn-lg", onclick=self.link.callback(|_| Msg::Minus)>{ "-" }</button>
                </div>
                <div class="m-3 text-center">
                    <p class="fs-1">{ self.value }</p>
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
