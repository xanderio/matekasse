use std::{fmt::Display, panic};

use agents::{product::ProductStore, user::UserStore};
use common::User;
use ybc::{TileCtx, TileSize};
use yew::prelude::*;

mod agents;
mod inventory;
mod menu;
mod product;
mod user;

pub struct App {
    link: ComponentLink<Self>,
    mode: Mode,
    loading: bool,
    _product_store: Box<dyn Bridge<ProductStore>>,
    _user_store: Box<dyn Bridge<UserStore>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Product(User),
    User,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Product(_) => write!(f, "Einkaufen"),
            Mode::User => write!(f, "User"),
        }
    }
}

pub enum Msg {
    MenuAction(menu::Action),
    ProductStore(agents::product::Output),
    UserStore(agents::user::Output),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link: link.clone(),
            mode: Mode::User,
            loading: true,
            _user_store: UserStore::bridge(link.callback(Msg::UserStore)),
            _product_store: ProductStore::bridge(link.callback(Msg::ProductStore)),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MenuAction(menu::Action::ChangeAccount) => {
                self.mode = Mode::User;
                true
            }
            Msg::ProductStore(agents::product::Output::Update(products)) => {
                self.loading = products.is_empty();
                true
            }
            Msg::UserStore(agents::user::Output::Current(Some(user))) => {
                log::info!("{:?}", &user);
                self.mode = Mode::Product(user);
                true
            }
            Msg::UserStore(agents::user::Output::Current(None)) => {
                self.mode = Mode::User;
                true
            }
            Msg::UserStore(_) => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let pl_active = if self.loading { "is-active" } else { "" };
        let menu_cb = self.link.callback(Msg::MenuAction);
        html! {
            <>
                <div class=classes!("pageloader", "is-bottom-to-top", pl_active)>
                    <ybc::Title>{"Loading"}</ybc::Title>
                </div>
                <ybc::Section>
                    <ybc::Container>
                        <ybc::Tile ctx=TileCtx::Ancestor>
                            <menu::Menu mode=self.mode.clone() on_action=menu_cb/>
                            <ybc::Tile size=TileSize::Nine>
                            {match self.mode {
                                Mode::Product(_) => html!{<product::ProductGrid/>},
                                Mode::User => html!{<user::UserGrid/>}
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
