use anyhow::Error;
use gloo_events::EventListener;
use ybc::TileCtx::{Ancestor, Child, Parent};
use ybc::TileSize::Four;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::FetchService;
use yew::web_sys::HtmlElement;

use common::Product;

#[derive(Debug)]
pub struct Checkout {
    link: ComponentLink<Self>,
    products: Vec<Product>,
    selected: Option<Product>,
    fetch_task: Option<FetchTask>,
}

#[derive(Debug)]
pub enum CheckoutMsg {
    Select(Product),
    Response(Result<Vec<Product>, Error>),
}

impl Component for Checkout {
    type Message = CheckoutMsg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let request = Request::get("/api/v3/products")
            .body(Nothing)
            .expect("Could build request");

        let task = FetchService::fetch(
            request,
            link.callback(|resp: Response<Json<Result<Vec<Product>, Error>>>| {
                let Json(data) = resp.into_body();
                CheckoutMsg::Response(data)
            }),
        )
        .expect("unable to build fetch task");

        Self {
            link,
            products: Vec::new(),
            selected: None,
            fetch_task: Some(task),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            CheckoutMsg::Select(product) => {
                self.selected = Some(product);
                false
            }
            CheckoutMsg::Response(Ok(products)) => {
                self.products = products;
                true
            }
            CheckoutMsg::Response(Err(e)) => {
                log::error!("{}", e);
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let cb = self.link.callback(CheckoutMsg::Select);
        let pl_active = if self.products.is_empty() {
            "is-active"
        } else {
            ""
        };
        html! {
        <>
          <div class=classes!("pageloader", pl_active)>
            <ybc::Title>{"Loading"}</ybc::Title>
          </div>
          <ybc::Container fluid=true>
            <ybc::Tile ctx=Ancestor>
              <ybc::Tile ctx=Parent vertical=true size=Four>
                { for self.products.iter().map(|p| html!{ <ProductCard item=p.clone() onclick=cb.clone() /> }) }
              </ybc::Tile>
            </ybc::Tile>
          </ybc::Container>
        </>
          }
    }
}

struct ProductCard {
    link: ComponentLink<Self>,
    props: ProductCardProps,
    node: NodeRef,
    onclick_listener: Option<EventListener>,
}

#[derive(Debug, Clone, Properties, PartialEq)]
struct ProductCardProps {
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
            <ybc::Tile ref={self.node.clone()} ctx=Child classes=classes!("box", "is-clickable")>
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
