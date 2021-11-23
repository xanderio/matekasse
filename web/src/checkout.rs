use gloo_events::EventListener;
use ybc::TileCtx::{Ancestor, Child, Parent};
use ybc::TileSize::Four;
use yew::prelude::*;
use yew::web_sys::HtmlElement;

#[derive(Debug, Clone, PartialEq)]
pub struct Product {
    name: String,
    price: f32,
}

#[derive(Debug)]
pub struct Checkout {
    link: ComponentLink<Self>,
    products: Vec<Product>,
    selected: Option<Product>,
}

#[derive(Debug)]
pub enum CheckoutMsg {
    Select(Product),
}

impl Component for Checkout {
    type Message = CheckoutMsg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let products = vec![
            Product {
                name: "Mio Mio Mate".to_string(),
                price: 1.50,
            },
            Product {
                name: "Flora Mate".to_string(),
                price: 2.00,
            },
            Product {
                name: "Mate Mate".to_string(),
                price: 1.50,
            },
            Product {
                name: "Kaffe".to_string(),
                price: 0.50,
            },
            Product {
                name: "Spezi".to_string(),
                price: 1.50,
            },
        ];
        Self {
            link,
            products,
            selected: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::info!("{:?}", msg);
        match msg {
            CheckoutMsg::Select(product) => self.selected = Some(product),
        };
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let cb = self.link.callback(CheckoutMsg::Select);
        html! {
        <ybc::Container fluid=true>
          <ybc::Tile ctx=Ancestor>
            <ybc::Tile ctx=Parent vertical=true size=Four>
                { for self.products.iter().map(|p| html!{ <ProductCard item=p.clone() onclick=cb.clone() /> }) }
            </ybc::Tile>
          </ybc::Tile>
        </ybc::Container>
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
                 <ybc::Subtitle>{format!("{0:.2}€", self.props.item.price)}</ybc::Subtitle>
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
