use std::{fmt::Display, panic};

use agents::ProductStore;
use ybc::{TileCtx, TileSize};
use yew::prelude::*;

mod agents;
mod inventory;
mod menu;
mod product;

pub struct App {
    mode: Mode,
    loading: bool,
    _store: Box<dyn Bridge<ProductStore>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Product,
    User,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Product => write!(f, "Einkaufen"),
            Mode::User => write!(f, "User"),
        }
    }
}

pub enum Msg {
    Store(agents::Output),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            mode: Mode::Product,
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
                    <ybc::Container>
                        <ybc::Tile ctx=TileCtx::Ancestor>
                            <menu::Menu mode=self.mode/>
                            <ybc::Tile size=TileSize::Nine>
                            {match self.mode {
                                Mode::Product => html!{<product::ProductGrid/>},
                                Mode::User => html!{<product::ProductGrid/>}
                            }}
                            </ybc::Tile>
                        </ybc::Tile>
                    </ybc::Container>
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
