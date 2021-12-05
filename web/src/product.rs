use anyhow::Result;
use common::{Product, User};
use gloo_events::EventListener;
use web_sys::HtmlElement;
use yew::prelude::*;

use crate::request::{buy_product, fetch_all_products};

pub struct ProductGrid {
    products: Vec<Product>,
}

pub enum Msg {
    FetchedProducts(Result<Vec<Product>>),
    ClickProduct(Product),
    BuyProduct(Result<User>),
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub user: User,
    pub on_change: Callback<User>,
}

impl Component for ProductGrid {
    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link()
            .send_future(async { Msg::FetchedProducts(fetch_all_products().await) });
        Self {
            products: Vec::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchedProducts(Ok(products)) => {
                self.products = products;
                true
            }
            Msg::FetchedProducts(Err(e)) => {
                log::error!("{}", e);
                false
            }
            Msg::BuyProduct(Ok(user)) => {
                ctx.props().on_change.emit(user);
                false
            }
            Msg::BuyProduct(Err(e)) => {
                log::error!("{}", e);
                false
            }
            Msg::ClickProduct(product) => {
                log::info!("{:?}", &product);
                let user = ctx.props().user.clone();
                ctx.link().send_future(async move {
                    Msg::BuyProduct(buy_product(&user, &product).await)
                });
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cb = ctx.link().callback(Msg::ClickProduct);
        html! {
            <div class="tile is-vertical">
            { for self.products.as_slice().chunks(3).map(|c| { html! {
                <div class="tile">
                {for c.iter().map(|p| html!{
                    <div class="tile is-parent is-4">
                        <ProductCard item={p.clone()} onclick={cb.clone()} />
                    </div>
                })}
                </div>
            }})}
            </div>
        }
    }
}

pub struct ProductCard {
    node: NodeRef,
    onclick_listener: Option<EventListener>,
}

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct ProductCardProps {
    item: Product,
    onclick: Callback<Product>,
}

impl Component for ProductCard {
    type Message = ();

    type Properties = ProductCardProps;

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
                <div class="card-image">
                    <figure class="image is-4by3">
                        <img src="https://bulma.io/images/placeholders/1280x960.png" alt="Placeholder image"/>
                    </figure>
                </div>
                <div class="card-header">
                    <div class="card-header-title is-centered">
                        <p class="is-size-3">{ctx.props().item.name.clone()}</p>
                    </div>
                </div>
                <div class="card-content">
                    <div class="content">
                        <p class="is-size-4 has-text-centered">{Self::format_price(ctx.props().item.price)}</p>
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

impl ProductCard {
    fn format_price(price: i32) -> String {
        format!("{0:.2}€", price as f64 / 100.0)
    }
}
