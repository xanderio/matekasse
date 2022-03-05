use anyhow::Result;
use common::User;
use gloo_events::EventListener;
use web_sys::HtmlElement;
use yew::prelude::*;

use crate::request::fetch_all_users;

pub struct UserGrid {
    users: Vec<User>,
}

pub enum Msg {
    FetchedUsers(Result<Vec<User>>),
    Select(User),
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub on_selected: Callback<User>,
}

impl Component for UserGrid {
    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link()
            .send_future(async { Msg::FetchedUsers(fetch_all_users().await) });
        Self { users: Vec::new() }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
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
                ctx.props().on_selected.emit(user);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="tile is-vertical">
            { for self.users.as_slice().chunks(3).map(|c| { html! {
                <div class="tile">
                {for c.iter().map(|p| html!{
                    <div class="tile is-parent is-4">
                        <UserCard item={p.clone()} onclick={ctx.link().callback(Msg::Select)} />
                    </div>
                })}
                </div>
            }})}
            </div>
        }
    }
}

pub struct UserCard {
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

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node: NodeRef::default(),
            onclick_listener: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, _msg: Self::Message) -> bool {
        ctx.props().onclick.emit(ctx.props().item.clone());
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div
              ref={self.node.clone()}
              class="tile is-child card is-clickable is-unselectable">
                <div class="card-content">
                    <div class="media">
                        <figure class="media-left">
                            <p class="image is-64x64">
                                <img class="is-rounded" alt={format!("Avatar von {}", ctx.props().item.name.clone())}
                                    src="https://chaos.social/system/accounts/avatars/000/015/422/original/AD8QQFNGKKJK.png"/>
                            </p>
                        </figure>
                        <p class="is-size-2 has-text-centered">
                            {ctx.props().item.name.clone()}
                        </p>
                    </div>
                </div>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        if let Some(element) = self.node.cast::<HtmlElement>() {
            let cb = ctx.link().callback(move |_| ());
            let listener = EventListener::new(&element, "click", move |e| cb.emit(e.clone()));
            self.onclick_listener = Some(listener);
        }
    }
}
