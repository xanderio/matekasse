use std::{fmt::Display, panic};

use common::User;
use yew::prelude::*;

mod menu;
mod modal;
mod product;
mod request;
mod settings;
mod user;

pub struct App {
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

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            mode: Mode::User,
            //TODO: agent to keep this state?
            loading: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        let pl_active = if self.loading { "is-active" } else { "" };

        let menu_cb = ctx.link().callback(Msg::MenuAction);
        let user_cb = ctx.link().callback(Msg::UserSelected);
        let product_cb = ctx.link().callback(Msg::UserSelected);

        html! {
            <>
                <div class={classes!("pageloader", "is-bottom-to-top", pl_active)}>
                    <h3 class="titel">{"Loading"}</h3>
                </div>
                <section class="section">
                    <div class="container">
                        <div class="columns">
                            <div class="column">
                                <menu::Menu mode={self.mode.clone()} on_action={menu_cb}/>
                            </div>
                            <div class="column tile is-ancestor is-9">
                            {match self.mode.clone() {
                                Mode::Product(user) => html!{<product::ProductGrid {user} on_change={product_cb}/>},
                                Mode::User => html!{<user::UserGrid on_selected={user_cb} />}
                            }}
                            </div>
                        </div>
                    </div>
                </section>
            </>
        }
    }
}

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());

    log::info!("local settings {:?}", settings::get_all());

    yew::start_app::<App>();
}
