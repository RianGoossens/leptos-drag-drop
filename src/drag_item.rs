use std::any::Any;

use leptos::*;
use web_sys::HtmlDivElement;

use crate::{drop_zone::DropZoneData, MousePosition};

#[component]
pub fn drag_item<A: Any + Clone, F, IV>(
    cx: Scope,
    drag_data: RwSignal<A>,
    view_function: F,
) -> impl IntoView
where
    IV: IntoView,
    F: Fn(Scope, RwSignal<A>) -> IV,
{
    let mouse_position = use_context::<RwSignal<MousePosition>>(cx)
        .expect("Drag Item was not put into a DragContext");
    let drop_zone_data =
        DropZoneData::summon(cx).expect("Drag Item was not placed in a Drop Zone.");

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
        drop_zone_data.start_drag();
    };

    let drag_end = move || {
        drop_zone_data.stop_drag();
    };

    view! { cx,
        <div
            _ref=node_ref
            style:position=move || {
                if drop_zone_data.dragging().get() { "absolute" } else { "relative" }
            }
            style:left=move || {
                if drop_zone_data.dragging().get() {
                    format!("{}px", mouse_position.get().0 - offset.get().0)
                } else {
                    "0px".to_string()
                }
            }

            style:top=move || {
                if drop_zone_data.dragging().get() {
                    format!("{}px", mouse_position.get().1 - offset.get().1)
                } else {
                    "0px".to_string()
                }
            }

            style:width=move || {
                if drop_zone_data.dragging().get() {
                    format!("{}px", size.get().0)
                } else {
                    "auto".to_string()
                }
            }

            style:height=move || {
                if drop_zone_data.dragging().get() {
                    format!("{}px", size.get().1)
                } else {
                    "auto".to_string()
                }
            }

            on:mousedown=move |event| {
                drag_start(event.page_x(), event.page_y());
            }

            on:mouseup=move |_| {
                drag_end();
            }

            on:touchstart=move |event| {
                let touch = event
                    .target_touches()
                    .get(0)
                    .expect("There should always be at least 1 touch.");
                drag_start(touch.page_x(), touch.page_y());
            }

            on:touchend=move |_| {
                drag_end();
            }
        >

            {view_function(cx, drag_data)}
        </div>
    }
}
