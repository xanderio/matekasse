use std::panic;

use agents::ProductStore;
use yew::prelude::*;

mod agents;
mod inventory;
mod product;

pub struct App {
    loading: bool,
    _store: Box<dyn Bridge<ProductStore>>,
}

pub enum Msg {
    Store(agents::Output),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            loading: true,
            _store: ProductStore::bridge(link.callback(Msg::Store)),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Store(agents::Output::Update(products)) => {
                self.loading = products.is_empty();
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let pl_active = if self.loading { "is-active" } else { "" };
        html! {
            <>
                <div class=classes!("pageloader", "is-bottom-to-top", pl_active)>
                    <ybc::Title>{"Loading"}</ybc::Title>
                </div>
                <ybc::Section>
                    <product::ProductGrid/>
                </ybc::Section>
            </>
        }
    }
}

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
