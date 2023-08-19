use std::any::Any;

use leptos::*;

use crate::DragContextData;

#[derive(Debug, Clone, Copy)]
pub struct DropZoneData {
    dragging: RwSignal<bool>,
}

impl DropZoneData {
    pub fn new(cx: Scope) -> Self {
        DropZoneData {
            dragging: create_rw_signal(cx, false),
        }
    }

    pub fn install(cx: Scope) -> Self {
        provide_context(cx, Self::new(cx));
        use_context(cx).unwrap()
    }

    pub fn summon(cx: Scope) -> Option<Self> {
        use_context(cx)
    }

    pub fn start_drag(&self) {
        self.dragging.set(true);
    }

    pub fn stop_drag(&self) {
        self.dragging.set(false);
    }

    pub fn dragging(&self) -> ReadSignal<bool> {
        self.dragging.read_only()
    }
}

#[component]
pub fn drop_zone<A, VF, DF, IV0, IV1>(
    cx: Scope,
    initial_value: Option<A>,
    view_function: VF,
    drag_item_function: DF,
) -> impl IntoView
where
    A: Any + Clone,
    VF: Fn(Scope, Children) -> IV0,
    DF: Fn(Scope, RwSignal<A>) -> IV1 + 'static,
    IV0: IntoView,
    IV1: IntoView,
{
    let drag_context =
        DragContextData::summon(cx).expect("Drop Zone was not inside of a Drag Context.");

    let drop_zone_data = DropZoneData::install(cx);

    let drop_data = create_rw_signal(cx, initial_value.map(|x| create_rw_signal(cx, x)));

    create_effect(cx, move |_| {
        if drop_zone_data.dragging().get() {
            if let Some(data) = drop_data.get() {
                log!("Drop data changed");
                drag_context.set_drag_data(data, move || {
                    //drop_data.set(None);
                })
            }
        }
    });

    let on_release = move || {
        log!("RELEASE");
        if drop_data.get().is_none() {
            log!("ISNONE");
            if let Some(new_data) = drag_context.steal::<A>() {
                log!("STEALING");
                drop_data.set(Some(create_rw_signal(cx, new_data)));
            }
        }
    };

    view! { cx,
        <div on:mouseup=move |_| on_release() on:touchend=move |_| on_release()>
            {view_function(
                cx,
                Box::new(move |cx| Fragment::new(
                    vec![
                        if let Some(drop_data) = drop_data.get() {
                            log!("REDRAW");
                            drag_item_function(cx, drop_data).into_view(cx)
                        } else {
                            view! { cx, {} } .into_view(cx)
                        }
                    ],
                )),
            )}

        </div>
    }
}
