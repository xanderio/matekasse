use common::Product;
use gloo_events::EventListener;
use ybc::{TileCtx, TileSize};
use yew::{prelude::*, web_sys::HtmlElement};

use crate::agents::{self, Output, ProductStore};

pub struct ProductGrid {
    link: ComponentLink<Self>,
    _store: Box<dyn Bridge<ProductStore>>,
    products: Vec<Product>,
}

pub enum GridMsg {
    Store(agents::Output),
    Select(Product),
}

impl Component for ProductGrid {
    type Message = GridMsg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link: link.clone(),
            _store: ProductStore::bridge(link.callback(GridMsg::Store)),
            products: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            GridMsg::Store(Output::Update(products)) => self.products = products,
            GridMsg::Select(product) => log::info!("{:?}", product),
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
          <ybc::Container>
            <ybc::Tile ctx=TileCtx::Ancestor>
              <ybc::Tile ctx=TileCtx::Parent vertical=true size=TileSize::Four>
                { for self.products.iter().map(|p| html!{ <ProductCard item=p.clone() onclick=cb.clone() /> }) }
              </ybc::Tile>
            </ybc::Tile>
          </ybc::Container>
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
            <ybc::Tile ref={self.node.clone()} ctx=TileCtx::Child classes=classes!("box", "is-clickable")>
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
