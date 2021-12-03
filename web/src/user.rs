use anyhow::Error;
use common::User;
use gloo_events::EventListener;
use ybc::{TileCtx, TileSize};
use yew::{prelude::*, services::fetch::FetchTask, web_sys::HtmlElement};

use crate::request::fetch_all_users;

pub struct UserGrid {
    link: ComponentLink<Self>,
    props: Props,
    _fetch_task: Option<FetchTask>,
    users: Vec<User>,
}

pub enum Msg {
    FetchedUsers(Result<Vec<User>, Error>),
    Select(User),
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub on_selected: Callback<User>,
}

impl Component for UserGrid {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let task = fetch_all_users(link.callback(Msg::FetchedUsers)).expect("unable to build task");
        Self {
            link,
            props,
            users: Vec::new(),
            _fetch_task: Some(task),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchedUsers(Ok(users)) => {
                self.users = users;
                true
            }
            Msg::FetchedUsers(Err(e)) => {
                log::error!("{}", e);
                false
            }
            Msg::Select(user) => {
                self.props.on_selected.emit(user);
                false
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
        let cb = self.link.callback(Msg::Select);
        html! {
            <>
                <ybc::Tile vertical=true>
                { for self.users.as_slice().chunks(3).map(|c| { html! {
                    <ybc::Tile>
                    {for c.iter().map(|p| html!{
                        <ybc::Tile ctx=TileCtx::Parent size=TileSize::Four>
                            <UserCard item=p.clone() onclick=cb.clone() />
                        </ybc::Tile>
                    })}
                    </ybc::Tile>
                }})}
                </ybc::Tile>
            </>
        }
    }
}

pub struct UserCard {
    link: ComponentLink<Self>,
    props: UserCardProps,
    node: NodeRef,
    onclick_listener: Option<EventListener>,
}

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct UserCardProps {
    item: User,
    onclick: Callback<User>,
}

impl Component for UserCard {
    type Message = ();

    type Properties = UserCardProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            node: NodeRef::default(),
            onclick_listener: None,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        self.props.onclick.emit(self.props.item.clone());
        false
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
        html! {
            <ybc::Tile
              ref={self.node.clone()}
              ctx=TileCtx::Child
              classes=classes!("box", "is-clickable", "is-unselectable")>
                <ybc::Title>{self.props.item.name.clone()}</ybc::Title>
            </ybc::Tile>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if !first_render {
            return;
        }

        if let Some(element) = self.node.cast::<HtmlElement>() {
            let cb = self.link.callback(move |_| ());
            let listener = EventListener::new(&element, "click", move |e| cb.emit(e.clone()));
            self.onclick_listener = Some(listener);
        }
    }
}
