use yew::prelude::*;

use crate::{modal::account::AccountEditor, Mode};

pub struct Menu {
    link: ComponentLink<Self>,
    props: Props,
}

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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Action(action) => {
                self.props.on_action.emit(action);
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let btn_classes = classes!("button", "is-fullwidth", "is-medium", "is-primary", "mb-2");
        let account_chance = self.link.callback(|_| Msg::Action(Action::ChangeAccount));

        let user = if let Mode::Product(user) = &self.props.mode {
            html! {
                <div class=classes!("media")>
                    <div class=classes!("media-left")>
                        <figure class=classes!("image", "is-64x64")>
                            <img class=classes!("is-rounded") src={"https://chaos.social/system/accounts/avatars/000/015/422/original/AD8QQFNGKKJK.png"}/>
                        </figure>
                    </div>
                    <div class=classes!("media-content")>
                        <div class=classes!("container")>
                            <h3 class=classes!("title")>
                                {user.name.clone()}
                            </h3>
                            <h3 class=classes!("subtitle", "is-4")>
                                {self.format_balance()}
                            </h3>
                        </div>
                    </div>
                </div>
            }
        } else {
            html! {
                <h3 class=classes!("title")>{"Account wählen"}</h3>
            }
        };

        let buttons = if self.props.mode == Mode::User {
            let trigger = html! {
                    <button class=btn_classes>{"Neuer Account"}</button>
            };
            html! {
                <AccountEditor user=None trigger=trigger/>
            }
        } else {
            html! {
                <>
                    <button class=btn_classes.clone() onclick=account_chance>{"Account wechseln"}</button>
                    <button class=btn_classes.clone() >{"Einzahlen"}</button>
                    <button class=btn_classes.clone() >{"Account bearbeiten"}</button>
                    <button class=btn_classes.clone() >{"Produkte bearbeiten"}</button>
                    <button class=btn_classes >{"Neues Produkt"}</button>
                </>
            }
        };

        html! {
            <div class=classes!("tile", "is-parent", "is-vertical")>
                <div class=classes!("card", "is-child")>
                    <header class=classes!("card-header")>
                        <h3 class=classes!("title", "card-header-title")>{self.props.mode.to_string()}</h3>
                    </header>
                    <div class=classes!("card-content")>
                        {user}
                        {buttons}
                    </div>
                </div>
            </div>
        }
    }
}

impl Menu {
    fn format_balance(&self) -> String {
        if let Mode::Product(user) = &self.props.mode {
            format!("{0:.2}€", user.balance as f64 / 100.0)
        } else {
            "".to_string()
        }
    }
}
