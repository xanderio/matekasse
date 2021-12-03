use anyhow::Result;
use common::{Product, User};
use gloo_events::EventListener;
use ybc::{TileCtx, TileSize};
use yew::{prelude::*, services::fetch::FetchTask, web_sys::HtmlElement};

use crate::request::{buy_product, fetch_all_products};

pub struct ProductGrid {
    link: ComponentLink<Self>,
    props: Props,
    products: Vec<Product>,
    fetch_task: Option<FetchTask>,
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let task = fetch_all_products(link.callback(Msg::FetchedProducts)).unwrap();
        Self {
            link,
            props,
            products: Vec::new(),
            fetch_task: Some(task),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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
                self.props.on_change.emit(user);
                false
            }
            Msg::BuyProduct(Err(e)) => {
                log::error!("{}", e);
                false
            }
            Msg::ClickProduct(product) => {
                log::info!("{:?}", &product);
                self.fetch_task = Some(
                    buy_product(
                        &self.props.user,
                        &product,
                        self.link.callback(Msg::BuyProduct),
                    )
                    .unwrap(),
                );
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
        let cb = self.link.callback(Msg::ClickProduct);
        html! {
            <>
                <ybc::Tile vertical=true>
                { for self.products.as_slice().chunks(3).map(|c| { html! {
                    <ybc::Tile>
                    {for c.iter().map(|p| html!{
                        <ybc::Tile ctx=TileCtx::Parent size=TileSize::Four>
                            <ProductCard item=p.clone() onclick=cb.clone() />
                        </ybc::Tile>
                    })}
                    </ybc::Tile>
                }})}
                </ybc::Tile>
            </>
        }
    }
}

pub struct ProductCard {
    link: ComponentLink<Self>,
    props: ProductCardProps,
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
                <ybc::Subtitle>{self.format_price()}</ybc::Subtitle>
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

impl ProductCard {
    fn format_price(&self) -> String {
        format!("{0:.2}€", self.props.item.price as f64 / 100.0)
    }
}
