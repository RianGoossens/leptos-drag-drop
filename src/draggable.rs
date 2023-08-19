use leptos::*;
use web_sys::EventTarget;

#[component]
pub fn draggable(cx: Scope, children: Children) -> impl IntoView {
    let dragging = create_rw_signal(cx, false);
    let x_position = create_rw_signal(cx, 0);
    let y_position = create_rw_signal(cx, 0);

    let x_drag_offset = create_rw_signal(cx, 0);
    let y_drag_offset = create_rw_signal(cx, 0);

    let node_ref = create_node_ref(cx);

    let drag_start = move |event_target: Option<EventTarget>, x, y| {
        if let Some(target) = event_target {
            if let Some(element) = node_ref.get() {
                if &element as &EventTarget == &target {
                    log!("OK!");
                    dragging.set(true);
                    x_drag_offset.set(x_position.get() - x);
                    y_drag_offset.set(y_position.get() - y);
                };
            }
        }
    };

    let drag_end = move || {
        dragging.set(false);
    };

    let drag_move = move |x, y| {
        if dragging.get() {
            x_position.set(x + x_drag_offset.get());
            y_position.set(y + y_drag_offset.get());
        }
    };

    let style = move || {
        let z_index = if dragging.get() { 1 } else { 0 };
        let box_shadow = if dragging.get() {
            "5pt 5pt 5pt rgb(0,0,0,0.5)"
        } else {
            "none"
        };
        let transform = if dragging.get() {
            "translate(-5pt, -5pt);"
        } else {
            "none"
        };
        format!(
            "background-color:red; 
            width:100pt; 
            height: 100pt; 
            position: absolute; 
            left: {}px; 
            top: {}px; 
            z-index:{};
            transition-property: box-shadow, transform;
            transition-duration: 0.2s;
            transition-timing-function: ease-in-out;
            box-shadow:{};
            transform:{}",
            x_position.get(),
            y_position.get(),
            z_index,
            box_shadow,
            transform
        )
    };

    view! { cx,
        <div
            _ref=node_ref
            style=style
            on:mousedown=move |event| drag_start(event.target(), event.page_x(), event.page_y())
            on:mouseup=move |_| drag_end()
            on:mouseleave=move |_| drag_end()
            on:mousemove=move |event| drag_move(event.page_x(), event.page_y())

            on:touchstart=move |event| {
                event.prevent_default();
                event.stop_propagation();
                if let Some(target_touch) = event.target_touches().get(0) {
                    drag_start(event.target(), target_touch.page_x(), target_touch.page_y());
                }
            }

            on:touchend=move |event| {
                event.prevent_default();
                event.stop_propagation();
                drag_end()
            }

            on:touchmove=move |event| {
                event.prevent_default();
                event.stop_propagation();
                if let Some(target_touch) = event.target_touches().get(0) {
                    drag_move(target_touch.page_x(), target_touch.page_y());
                }
            }
        >

            {children(cx)}
        </div>
    }
}
