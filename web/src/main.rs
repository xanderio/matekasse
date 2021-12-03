use std::{fmt::Display, panic};

use common::User;
use ybc::{TileCtx, TileSize};
use yew::prelude::*;

mod agents;
mod inventory;
mod menu;
mod product;
mod request;
mod user;

pub struct App {
    link: ComponentLink<Self>,
    mode: Mode,
    loading: bool,
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
    UserSelected(User),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            mode: Mode::User,
            //TODO: agent to keep this state?
            loading: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MenuAction(menu::Action::ChangeAccount) => {
                self.mode = Mode::User;
                true
            }
            Msg::UserSelected(user) => {
                self.mode = Mode::Product(user);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let pl_active = if self.loading { "is-active" } else { "" };

        let menu_cb = self.link.callback(Msg::MenuAction);
        let user_cb = self.link.callback(Msg::UserSelected);
        let product_cb = self.link.callback(Msg::UserSelected);

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
                            {match self.mode.clone() {
                                Mode::Product(user) => html!{<product::ProductGrid user=user on_change=product_cb/>},
                                Mode::User => html!{<user::UserGrid on_selected=user_cb />}
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
