use crate::fetch;
// yew_icons = {version = "0.8", features = ["LucideMinimize2", "LucideMaximize2", "FontAwesomeSolidWindowMinimize", "FontAwesomeSolidSkullCrossbones"]}
pub struct TitleBar {
    full_screen: bool,
    icon_style: String,
}

pub enum Msg {
    Minimize,
    Maximize,
    Close,
}

impl TitleBar {
    fn maximin_icon(&self) -> yew::prelude::Html {
        if self.full_screen {
            yew::prelude::html! {
                <yew_icons::Icon icon_id={yew_icons::IconId::LucideMinimize2} style={self.icon_style.clone()}/>
            }
        } else {
            yew::prelude::html! {
                <yew_icons::Icon icon_id={yew_icons::IconId::LucideMaximize2}  style={self.icon_style.clone()}/>
            }
        }
    }
}

impl yew::prelude::Component for TitleBar {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
        Self {
            full_screen: false,
            icon_style: "height: 12px; width: 12px;".to_owned(),
        }
    }

    fn update(&mut self, _ctx: &yew::prelude::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Minimize => fetch::execute("minimize_window".to_owned()),
            Msg::Maximize => {
                self.full_screen = !self.full_screen;
                fetch::execute("maximize_window".to_owned());
            }
            Msg::Close => fetch::execute("close_window".to_owned()),
        }
        true
    }

    fn view(&self, ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        yew::prelude::html! {
            <div data-tauri-drag-region="" id="title-bar">
                <button onclick={ctx.link().callback(|_| Msg::Minimize)} class="tbar-other-btns"><yew_icons::Icon icon_id={yew_icons::IconId::FontAwesomeSolidWindowMinimize} style={self.icon_style.clone()} /></button>
                <button onclick={ctx.link().callback(|_| Msg::Maximize)} class="tbar-other-btns">{self.maximin_icon()}</button>
                <button onclick={ctx.link().callback(|_| Msg::Close)} id="close"><yew_icons::Icon icon_id={yew_icons::IconId::FontAwesomeSolidSkullCrossbones} style={self.icon_style.clone()} /></button>
            </div>
        }
    }

    fn changed(
        &mut self,
        _ctx: &yew::prelude::Context<Self>,
        _old_props: &Self::Properties,
    ) -> bool {
        true
    }

    fn rendered(&mut self, _ctx: &yew::prelude::Context<Self>, _first_render: bool) {}

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, _ctx: &yew::prelude::Context<Self>) {}
}
