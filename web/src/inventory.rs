use common::Product;
use ybc::{ModalCloseMsg, ModalCloser};
use yew::{agent::Dispatcher, prelude::*};

#[derive(Debug)]
pub struct Inventory {
    _link: ComponentLink<Self>,
}

impl Component for Inventory {
    type Message = ();

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { _link: link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Editor product=None/>
        }
    }
}

#[derive(Debug)]
struct Editor {
    link: ComponentLink<Self>,
    props: EditorProps,
    bridge: Dispatcher<ModalCloser>,
    inner: Product,
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct EditorProps {
    product: Option<Product>,
}

#[derive(Debug, Clone)]
enum EditorMsg {
    Save,
    Abort,
    Delete,
    Update(Field),
}

#[derive(Debug, Clone)]
enum Field {
    Name(String),
    Price(String),
    Active(bool),
}

impl Component for Editor {
    type Message = EditorMsg;

    type Properties = EditorProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let bridge = ModalCloser::dispatcher();
        Self {
            link,
            inner: props.product.clone().unwrap_or_default(),
            props,
            bridge,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            EditorMsg::Save => {
                //TODO: save to backend
                self.bridge.send(ModalCloseMsg("editor".into()));
                false
            }
            EditorMsg::Abort => {
                self.bridge.send(ModalCloseMsg("editor".into()));
                false
            }
            EditorMsg::Delete => {
                //TODO: delete from backend
                self.bridge.send(ModalCloseMsg("editor".into()));
                false
            }
            EditorMsg::Update(field) => {
                match field {
                    Field::Name(v) => self.inner.name = v,
                    Field::Price(v) => self.inner.price = v.parse().unwrap(),
                    Field::Active(v) => self.inner.active = v,
                }
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let trigger = html! {<button>{"Open"}</button>};
        let title = if self.props.product.is_some() {
            "Edit Product"
        } else {
            "New Product"
        };
        let body = html! {
            <ybc::Field>
                <label for={"name"} class=classes!("label")>{"Name:"}</label>
                <ybc::Input
                    name={"name"}
                    value=self.inner.name.clone()
                    update=self.link.callback(|v| EditorMsg::Update(Field::Name(v)))
                />
                <label for={"price"} class=classes!("label")>{"Price in cents:"}</label>
                <input
                    name={"Price"}
                    type={"number"}
                    class=classes!("input")
                    value=self.inner.price.to_string()
                    min={"0"}
                    step={"any"}
                    oninput=self.link.callback(|v: InputData| EditorMsg::Update(Field::Price(v.value)))
                />
                <ybc::Checkbox
                    name={"active"}
                    checked=self.inner.active
                    update=self.link.callback(|v| EditorMsg::Update(Field::Active(v))) >
                {"Active"}
                </ybc::Checkbox>
            </ybc::Field>
        };

        let cb = |msg: EditorMsg| self.link.callback(move |_| msg.clone());
        let footer = html! {
            <ybc::Buttons>
              <ybc::Button onclick=cb(EditorMsg::Save) classes=classes!("is-success")>{"Save"}</ybc::Button>
              <ybc::Button onclick=cb(EditorMsg::Abort) classes=classes!("is-warning")>{"Abort"}</ybc::Button>
              <ybc::Button onclick=cb(EditorMsg::Delete) classes=classes!("is-danger")>{"Delete"}</ybc::Button>
            </ybc::Buttons>
        };

        html! {
            <ybc::ModalCard
                id="editor"
                title=title
                body=body
                footer=footer
                trigger=trigger
             />
        }
    }
}
