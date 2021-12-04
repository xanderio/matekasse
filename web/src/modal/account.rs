use common::User;
use yew::{agent::Dispatcher, prelude::*};

use crate::modal::{ModalCard, ModalCloseMsg, ModalCloser};

#[derive(Debug)]
pub struct AccountEditor {
    link: ComponentLink<Self>,
    props: Props,
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let bridge = ModalCloser::dispatcher();
        let user = props.user.clone().unwrap_or_default();

        Self {
            link,
            props,
            bridge,
            user,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let id = "AccountEditor".to_string();
        let title = if self.props.user.is_some() {
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
                        name={"name"}
                        type="text"
                        class="input"
                        value=self.user.name.clone()
                        oninput=self.link.callback(|e: InputData| Msg::Name(e.value))/>
                </div>
            </div>
            <div class="field">
                <label class="label">{"E-Mail"}</label>
                <div class="control">
                    <input
                        name={"email"}
                        type="text"
                        class="input"
                        value=self.user.email.clone()
                        oninput=self.link.callback(|e: InputData| Msg::Email(e.value))/>
                </div>
            </div>
            <div class="field">
                <label class="label">
                    <input
                        name={"active"}
                        type="checkbox"
                        checked=active
                        oninput=self.link.callback(move |_| Msg::Active(!active))
                        />
                    {"Active"}
                </label>
            </div>
            <div class="field">
                <label class="label">
                    <input
                        name={"audit"}
                        type="checkbox"
                        checked=audit
                        oninput=self.link.callback(move |_| Msg::Audit(!audit))
                        />
                    {"Audit"}
                </label>
            </div>
            </>
        };
        let footer = html! {
            <div class="buttons">
                <button onclick=self.link.callback(|_| Msg::Save) class="button is-primary">
                    {"Speichern"}
                </button>
                <button onclick= self.link.callback(|_| Msg::Abort) class="button is-warning">
                    {"Abrechnen"}
                </button>
            </div>
        };
        html! {
            <ModalCard id=id title=title footer=footer body=body trigger=self.props.trigger.clone()/>
        }
    }
}
