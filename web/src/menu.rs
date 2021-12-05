use yew::prelude::*;

use crate::{modal::account::AccountEditor, Mode};

pub struct Menu;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub mode: Mode,
    pub on_action: Callback<Action>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    Action(Action),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    ChangeAccount,
}

impl Component for Menu {
    type Message = Msg;

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Action(action) => {
                ctx.props().on_action.emit(action);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let btn_classes = classes!("button", "is-fullwidth", "is-medium", "is-primary", "mb-2");
        let account_chance = ctx.link().callback(|_| Msg::Action(Action::ChangeAccount));

        let user = if let Mode::Product(user) = &ctx.props().mode {
            html! {
                <div class="media">
                    <div class="media-left">
                        <figure class="image is-64x64">
                            <img class="is-rounded" src="https://chaos.social/system/accounts/avatars/000/015/422/original/AD8QQFNGKKJK.png"/>
                        </figure>
                    </div>
                    <div class="media-content">
                        <div class="container">
                            <h3 class="title">
                                {user.name.clone()}
                            </h3>
                            <h3 class="subtitle is-4">
                                {Self::format_balance(user.balance)}
                            </h3>
                        </div>
                    </div>
                </div>
            }
        } else {
            html! {
                <h3 class="title">{"Account wählen"}</h3>
            }
        };

        let buttons = if ctx.props().mode == Mode::User {
            let trigger = html! {
                    <button class={btn_classes}>{"Neuer Account"}</button>
            };
            html! {
                <AccountEditor user={None} {trigger}/>
            }
        } else {
            html! {
                <>
                    <button class={btn_classes.clone()} onclick={account_chance}>{"Account wechseln"}</button>
                    <button class={btn_classes.clone()}>{"Einzahlen"}</button>
                    <button class={btn_classes.clone()}>{"Account bearbeiten"}</button>
                    <button class={btn_classes.clone()}>{"Produkte bearbeiten"}</button>
                    <button class={btn_classes}>{"Neues Produkt"}</button>
                </>
            }
        };

        html! {
            <div class="card">
                <header class="card-header">
                    <h3 class="title card-header-title">{ctx.props().mode.to_string()}</h3>
                </header>
                <div class="card-content">
                    {user}
                    {buttons}
                </div>
            </div>
        }
    }
}

impl Menu {
    fn format_balance(balance: i32) -> String {
        format!("{0:.2}€", balance as f64 / 100.0)
    }
}
