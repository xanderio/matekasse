use std::panic;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum AppRouter {
    #[to = "/user"]
    Users,
    #[to = "/inventory"]
    Inventory,
    #[to = "/"]
    Checkout,
}

mod checkout;
mod inventory;

#[derive(Debug)]
pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <Router<AppRouter>
              render = Router::render(|router: AppRouter| {
                let page = match router {
                    AppRouter::Users => html!{<p>{"Users"}</p>},
                    AppRouter::Inventory => html!{<inventory::Inventory/>},
                    AppRouter::Checkout => html!{<checkout::Checkout/>},
                };
                page
            })
          />
        }
    }
}

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
