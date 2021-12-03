use common::User;
use gloo_events::EventListener;
use ybc::{TileCtx, TileSize};
use yew::{prelude::*, web_sys::HtmlElement};

use crate::agents::user::{Output, UserStore};

pub struct UserGrid {
    link: ComponentLink<Self>,
    _store: Box<dyn Bridge<UserStore>>,
    users: Vec<User>,
}

pub enum GridMsg {
    Store(Output),
    Select(User),
}

impl Component for UserGrid {
    type Message = GridMsg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link: link.clone(),
            _store: UserStore::bridge(link.callback(GridMsg::Store)),
            users: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            GridMsg::Store(Output::Update(users)) => self.users = users,
            GridMsg::Select(users) => log::info!("{:?}", users),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let cb = self.link.callback(GridMsg::Select);
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
