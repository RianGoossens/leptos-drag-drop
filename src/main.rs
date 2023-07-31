use leptos::*;

pub mod draggable;

pub use draggable::Draggable;

#[component]
fn thingy(cx: Scope, color: [u8; 3]) -> impl IntoView {
    let style = move || {
        format!(
            "border-radius:10pt;background-color:rgb({},{},{});padding:10pt;margin:10pt;",
            color[0], color[1], color[2]
        )
    };
    view! { cx,
        <div style=style>
            "I'm a Thingy!"
        </div>
    }
}

#[component]
fn drop_context(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div>
            {children(cx)}
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
