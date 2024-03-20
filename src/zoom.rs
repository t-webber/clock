use yew::prelude::*;

pub struct App {
    pub zoom_level: f64, // Represents the current zoom level
}

pub enum Msg {
    ZoomIn,
    ZoomOut,
}

#[allow(clippy::float_arithmetic)]
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { zoom_level: 1.0 }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="zoom-bar">
                <button onclick={ctx.link().callback(move |_| Msg::ZoomIn)}>{ "+ Zoom In" }</button>
                <button onclick={ctx.link().callback(move |_| Msg::ZoomOut)}>{ "- Zoom Out" }</button>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        //     // Handle zoom functionality here if needed
        false
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {}

    // fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    //     App { zoom_level: 1.0 }
    // }

    // fn update(&mut self, _: Self::Message) -> ShouldRender {
    //     // Handle zoom functionality here if needed
    //     false
    // }

    // fn view(&self) -> Html {
    //     html! {
    //         <div>
    //             <button onclick=|_| Msg::ZoomIn>{ "+ Zoom In" }</button>
    //             <button onclick=|_| Msg::ZoomOut>{ "- Zoom Out" }</button>
    //             <div style=format!("font-size: {}%", self.zoom_level * 100)>
    //                 { "Text content goes here" }
    //             </div>
    //         </div>
    //     }
    // }
}
