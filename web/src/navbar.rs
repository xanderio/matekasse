use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum AppRouter {
    #[to = "/user"]
    Users,
    #[to = "/inventory"]
    Inventory,
    #[to = "/"]
    Checkout,
}

#[derive(Debug)]
pub struct Navbar {
    router: AppRouter,
}

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct NavbarProps {
    pub router: AppRouter,
}

impl Component for Navbar {
    type Message = ();

    type Properties = NavbarProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            router: props.router,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.router != props.router {
            self.router = props.router;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let navlink = |route: AppRouter, name: &str| -> Html {
            let mut classes = classes!("navbar-item");
            if self.router == route {
                classes.push("is-active")
            }

            html! {
                <RouterAnchor<AppRouter> route=route classes=classes.to_string()>
                    {name}
                </RouterAnchor<AppRouter>>
            }
        };
        let navstart = html! {
            <>
                {navlink(AppRouter::Checkout, "Kasse")}
                {navlink(AppRouter::Users, "Nutzer")}
                {navlink(AppRouter::Inventory, "Inventar")}
            </>
        };

        html! {
            <ybc::Navbar fixed=ybc::NavbarFixed::Top navstart=navstart />
        }
    }
}
