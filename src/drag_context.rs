use std::{any::Any, rc::Rc};

use leptos::*;

#[derive(Default, Debug, Clone, Copy)]
pub struct MousePosition(pub i32, pub i32);

impl MousePosition {
    pub fn x(&self) -> i32 {
        self.0
    }
    pub fn y(&self) -> i32 {
        self.1
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DragContextData {
    drag_data: RwSignal<Option<Rc<dyn Any>>>,
    on_remove: RwSignal<Option<Rc<dyn Fn()>>>,
}

impl DragContextData {
    pub fn new(cx: Scope) -> Self {
        DragContextData {
            drag_data: create_rw_signal(cx, None),
            on_remove: create_rw_signal(cx, None),
        }
    }

    pub fn install(cx: Scope) -> Self {
        provide_context(cx, Self::new(cx));
        use_context(cx).unwrap()
    }

    pub fn summon(cx: Scope) -> Option<Self> {
        use_context(cx)
    }

    pub fn set_drag_data<A: Any + Clone, F: Fn() + 'static>(&self, data: A, new_on_remove: F) {
        self.on_remove.update(|on_remove| {
            if let Some(on_remove) = on_remove {
                on_remove();
            }
            *on_remove = Some(Rc::new(new_on_remove))
        });
        self.drag_data.set(Some(Rc::new(data)));
    }

    pub fn clear(&self) {
        self.on_remove.set(None);
        self.drag_data.set(None);
    }

    pub fn can_steal<A: 'static>(&self) -> bool {
        if let Some(drag_data) = self.drag_data.get() {
            drag_data.as_ref().is::<A>()
        } else {
            false
        }
    }

    pub fn steal<A: Clone + 'static>(&self) -> Option<A> {
        log!("Stealing...");
        if let Some(drag_data) = self.drag_data.get() {
            log!("Data found...");
            if let Some(result) = drag_data.downcast_ref::<A>() {
                log!("Correct type...");
                if let Some(on_remove) = self.on_remove.get() {
                    on_remove();
                };
                self.clear();
                Some(result.clone())
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[component]
pub fn drag_context(cx: Scope, children: Children) -> impl IntoView {
    let mouse_position = create_rw_signal(cx, MousePosition::default());
    provide_context(cx, mouse_position);

    let drag_context = DragContextData::install(cx);

    create_effect(cx, move |_| {
        log!(
            "drag data changed in context {}",
            drag_context.drag_data.get().is_some()
        );
    });

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

            {move || format!("{:?}", mouse_position.get())}
            {children(cx)}
        </div>
    }
}
