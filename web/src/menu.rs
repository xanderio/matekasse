use ybc::{ImageSize, TileCtx};
use yew::prelude::*;

use crate::Mode;

pub struct Menu {
    _link: ComponentLink<Self>,
    props: Props,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub mode: Mode,
}

impl Component for Menu {
    type Message = ();

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { _link: link, props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <ybc::Tile ctx=TileCtx::Parent vertical=true>
                <ybc::Card classes=classes!("is-child")>
                    <ybc::CardHeader>
                        <ybc::Title classes=classes!("card-header-title")>{self.props.mode.to_string()}</ybc::Title>
                    </ybc::CardHeader>
                    <ybc::CardContent>
                        <ybc::Media>
                            <ybc::MediaLeft>
                                <ybc::Image size=ImageSize::Is64x64>
                                    <img class=classes!("is-rounded") src={"https://chaos.social/system/accounts/avatars/000/015/422/original/AD8QQFNGKKJK.png"}/>
                                </ybc::Image>
                            </ybc::MediaLeft>
                            <ybc::MediaContent>
                                <ybc::Container>
                                    <ybc::Title>
                                        {"xanderio"}
                                    </ybc::Title>
                                    <ybc::Subtitle classes=classes!("is-4")>
                                        {"20,00€"}
                                    </ybc::Subtitle>
                                </ybc::Container>
                            </ybc::MediaContent>
                        </ybc::Media>
                        <ybc::Button classes=classes!("is-fullwidth", "is-medium", "is-primary", "mb-2")>{"Einzahlen"}</ybc::Button>
                        <ybc::Button classes=classes!("is-fullwidth", "is-medium", "is-primary", "mb-2")>{"Account bearbeiten"}</ybc::Button>
                        <ybc::Button classes=classes!("is-fullwidth", "is-medium", "is-primary", "mb-2")>{"Produkte bearbeiten"}</ybc::Button>
                        <ybc::Button classes=classes!("is-fullwidth", "is-medium", "is-primary", "mb-2")>{"Neues Produkt"}</ybc::Button>
                    </ybc::CardContent>
                </ybc::Card>
            </ybc::Tile>
        }
    }
}
