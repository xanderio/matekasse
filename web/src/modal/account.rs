use common::User;
use yew::prelude::*;
use yew_agent::{Dispatched, Dispatcher};

use crate::modal::{ModalCard, ModalCloseMsg, ModalCloser};

#[derive(Debug)]
pub struct AccountEditor {
    bridge: Dispatcher<ModalCloser>,
    user: User,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub user: Option<User>,
    pub trigger: Html,
}

pub enum Msg {
    Save,
    Abort,
    Name(String),
    Email(String),
    Active(bool),
    Audit(bool),
}

impl Component for AccountEditor {
    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let bridge = ModalCloser::dispatcher();
        let user = ctx.props().user.clone().unwrap_or_default();

        Self { bridge, user }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Save => false,
            Msg::Abort => {
                self.bridge.send(ModalCloseMsg("AccountEditor".into()));
                false
            }
            Msg::Name(name) => {
                self.user.name = name;
                true
            }
            Msg::Email(email) => {
                self.user.email = if !email.is_empty() { Some(email) } else { None };
                true
            }
            Msg::Active(active) => {
                self.user.active = active;
                true
            }
            Msg::Audit(audit) => {
                self.user.audit = audit;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let id = "AccountEditor".to_string();
        let title = if ctx.props().user.is_some() {
            "Account bearbeiten"
        } else {
            "Neuen Account erstellen"
        };
        let active = self.user.active;
        let audit = self.user.audit;
        let body = html! {
            <>
            <div class="field">
                <label class="label">{"Name"}</label>
                <div class="control">
                    <input
                        name="name"
                        type="text"
                        class="input"
                        value={self.user.name.clone()}
                        oninput={ctx.link().callback(|e: InputEvent| Msg::Name(e.data().unwrap_or_default()))}/>
                </div>
            </div>
            <div class="field">
                <label class="label">{"E-Mail"}</label>
                <div class="control">
                    <input
                        name="email"
                        type="text"
                        class="input"
                        value={self.user.email.clone()}
                        oninput={ctx.link().callback(|e: InputEvent| Msg::Email(e.data().unwrap_or_default()))}/>
                </div>
            </div>
            <div class="field">
                <label class="label">
                    <input
                        name="active"
                        type="checkbox"
                        checked={active}
                        oninput={ctx.link().callback(move |_| Msg::Active(!active))}
                        />
                    {"Active"}
                </label>
            </div>
            <div class="field">
                <label class="label">
                    <input
                        name="audit"
                        type="checkbox"
                        checked={audit}
                        oninput={ctx.link().callback(move |_| Msg::Audit(!audit))}
                        />
                    {"Audit"}
                </label>
            </div>
            </>
        };
        let footer = html! {
            <div class="buttons">
                <button onclick={ctx.link().callback(|_| Msg::Save)} class="button is-primary">
                    {"Speichern"}
                </button>
                <button onclick={ctx.link().callback(|_| Msg::Abort)} class="button is-warning">
                    {"Abrechnen"}
                </button>
            </div>
        };
        html! {
            <ModalCard {id} {title} {footer} {body} trigger={ctx.props().trigger.clone()}/>
        }
    }
}
