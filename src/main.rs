use leptos::*;

pub mod drag_context;
pub mod drag_item;
pub mod draggable;
pub mod drop_zone;

pub use drag_context::{DragContext, DragContextData, MousePosition};
pub use drag_item::DragItem;
pub use draggable::Draggable;
pub use drop_zone::DropZone;

#[component]
fn my_drop_zone(cx: Scope, color: Option<[u8; 3]>) -> impl IntoView {
    view! { cx,
        <DropZone
            initial_value=color
            view_function=|cx, children| {
                view! { cx,
                    <div
                        style:display="flex"
                        style:flex-direction="column"
                        style:background-color="gray"
                        style:padding="10pt"
                        style:margin="10pt"
                        style:max-width="100pt"
                        style:height="100pt"
                        style:border-radius="10pt"
                        style:flex=1
                    >
                        "I'm a Drop Zone!!"
                        {children(cx)}
                    </div>
                }
            }

            drag_item_function=|cx, data| view! { cx, <MyDragItem color=data/> }
        />
    }
}

#[component]
fn my_drag_item(cx: Scope, color: RwSignal<[u8; 3]>) -> impl IntoView {
    view! { cx,
        <DragItem
            drag_data=color
            view_function=|cx, color| {
                let color = color.get();
                view! { cx,
                    <div
                        style:border-radius="10pt"
                        style:padding="10pt"
                        style:background-color=move || {
                            format!("rgb({},{},{})", color[0], color[1], color[2])
                        }
                    >

                        "I'm a Thingy!"
                    </div>
                }
            }
        />
    }
}

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <div style="height:200pt;background-color:green;">
                <Draggable>
                    <div>"Drag Area" <br/> "hi"</div>
                </Draggable>
            </div>
            <div style="height:200pt;background-color:orange;margin:10pt;display:flex;flex-direction:column;">
                "Hello World!" <DragContext>
                    <div style="display:flex;flex-direction:row;">
                        <MyDropZone color=Some([0, 255, 0])/>
                        <MyDropZone color=Some([255, 0, 255])/>
                        <MyDropZone color=None/>
                    </div>
                </DragContext>
            </div>
        }
    })
}
