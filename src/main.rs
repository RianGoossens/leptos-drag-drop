use leptos::*;

pub mod draggable;

pub use draggable::Draggable;
use web_sys::HtmlDivElement;

#[component]
fn thingy(cx: Scope, color: [u8; 3]) -> impl IntoView {
    let mouse_position = use_context::<RwSignal<MousePosition>>(cx)
        .expect("DropZone was not put into a DropContext");
    let dragging = create_rw_signal(cx, false);
    let offset = create_rw_signal(cx, (0, 0));
    let size = create_rw_signal(cx, (0., 0.));

    let node_ref = create_node_ref(cx);

    let drag_start = move |x, y| {
        mouse_position.set(MousePosition(x, y));
        let mouse_position = mouse_position.get();

        let element = &node_ref.get().expect("Node ref not set!") as &HtmlDivElement;
        let rect = element.get_bounding_client_rect();
        size.set((rect.width(), rect.height()));
        offset.set((
            mouse_position.x() - rect.x() as i32,
            mouse_position.y() - rect.y() as i32,
        ));
        dragging.set(true);
    };

    let drag_end = move || {
        dragging.set(false);
    };

    view! { cx,
        <div
            _ref=node_ref
            style:position=move || if dragging.get() { "absolute" } else { "relative" }
            style:left=move || if dragging.get() { format!("{}px", mouse_position.get().0 - offset.get().0) } else { "0px".to_string() }
            style:top=move || if dragging.get() { format!("{}px", mouse_position.get().1 - offset.get().1) } else { "0px".to_string() }
            style:width=move || if dragging.get() { format!("{}px", size.get().0) } else { "auto".to_string() }
            style:height=move || if dragging.get() { format!("{}px", size.get().1) } else { "auto".to_string() }

            on:mousedown=move |event| {
                drag_start(event.page_x(), event.page_y());
            }
            on:mouseup=move |_| {
                drag_end();
            }
            on:touchstart=move |event| {
                let touch = event.target_touches().get(0).expect("There should always be at least 1 touch.");
                drag_start(touch.page_x(), touch.page_y());
            }
            on:touchend=move |event| {
                drag_end();
            }
        >
            <div
                style:border-radius="10pt"
                style:padding="10pt"
                style:background-color=move || format!("rgb({},{},{})", color[0], color[1], color[2])
            >
                "I'm a Thingy!"
            </div>
        </div>
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct MousePosition(i32, i32);

impl MousePosition {
    pub fn x(&self) -> i32 {
        self.0
    }
    pub fn y(&self) -> i32 {
        self.1
    }
}

#[component]
fn drop_context(cx: Scope, children: Children) -> impl IntoView {
    let mouse_position = create_rw_signal(cx, MousePosition::default());

    provide_context(cx, mouse_position);

    view! { cx,
        <div
            style:width="100%"
            style:height="100%"
            on:mousemove=move |event| {
                mouse_position.set(MousePosition(event.page_x(), event.page_y()));
            }
            on:touchmove=move |event| {
                let touch = event.target_touches().get(0).expect("There should always be 1 Touch");
                mouse_position.set(MousePosition(touch.page_x(), touch.page_y()));
            }
        >
            { move || format!("{:?}", mouse_position.get())}
            { children(cx) }
        </div>
    }
}

#[component]
fn drop_zone(cx: Scope, size_weight: u32, children: Children) -> impl IntoView {
    let style = move || {
        format!("display:flex;flex-direction:column;background-color:gray;padding:10pt;margin:10pt;max-width:100pt;height:100pt;border-radius:10pt;flex:{size_weight}")
    };
    view! { cx,
        <div style=style>
            "I'm a Drop Zone!!"
            { children(cx) }
        </div>
    }
}

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <div style="height:200pt;background-color:green;">
                <Draggable>
                    <div>
                        "Drag Area" <br/>
                        "hi"
                    </div>
                </Draggable>
            </div>
            <div style="height:200pt;background-color:orange;margin:10pt;display:flex;flex-direction:row;">
                "Hello World!"
                <DropContext>
                    <DropZone size_weight=1>
                        <Thingy color=[0,255,0]/>
                    </DropZone>
                </DropContext>
            </div>
        }
    })
}
