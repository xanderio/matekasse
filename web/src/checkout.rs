use anyhow::Error;
use common::Product;
use ybc::TileCtx::{Ancestor, Parent};
use ybc::TileSize::Four;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::FetchService;

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
          <ybc::Container>
            <ybc::Tile ctx=Ancestor>
              <ybc::Tile ctx=Parent vertical=true size=Four>
                // { for self.products.iter().map(|p| html!{ <product::ProductCard item=p.clone() onclick=cb.clone() /> }) }
              </ybc::Tile>
            </ybc::Tile>
          </ybc::Container>
        </>
          }
    }
}
