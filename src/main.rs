use leptos::*;
use web_sys::{EventTarget, MouseEvent};

#[component]
fn drag_area(cx: Scope, children: Children) -> impl IntoView {
    let dragging = create_rw_signal(cx, false);
    let x_position = create_rw_signal(cx, 0);
    let y_position = create_rw_signal(cx, 0);

    let x_drag_offset = create_rw_signal(cx, 0);
    let y_drag_offset = create_rw_signal(cx, 0);

    let node_ref = create_node_ref(cx);

    let on_mouse_down = move |event: MouseEvent| {
        if let Some(target) = event.target() {
            if let Some(element) = node_ref.get() {
                if &element as &EventTarget == &target {
                    dragging.set(true);
                    x_drag_offset.set(x_position.get() - event.page_x());
                    y_drag_offset.set(y_position.get() - event.page_y());
                };
            }
        }
    };

    let on_mouse_up = move |_| {
        dragging.set(false);
    };

    let on_mouse_move = move |event: MouseEvent| {
        if dragging.get() {
            x_position.set(event.page_x() + x_drag_offset.get());
            y_position.set(event.page_y() + y_drag_offset.get());
        }
    };

    let style = move || {
        format!("background-color:red; width:100pt; height: 100pt; position: absolute; left: {}px; top: {}px;", x_position.get(), y_position.get())
    };

    view! { cx,
        <div
            _ref=node_ref
            style=style
            on:mousemove=on_mouse_move
            on:mouseup=on_mouse_up
            on:mousedown=on_mouse_down
            on:mouseleave=on_mouse_up
        >
            <div>
                "Drag Area"
            </div>
            { children(cx) }
        </div>
    }
}

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <DragArea>
                "hi"
            </DragArea>
        }
    })
}
