use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use rand::Rng;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <NavBar/>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="/other" view=|cx| view! { cx, <OtherPage/>} />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn NavBar(cx: Scope) -> impl IntoView {
    view! { cx,
    <nav>
        <A exact=true href="/">"Home"</A>
        <A href="/other">"Other"</A>
    </nav>}
}

#[component]
fn OtherPage(cx: Scope) -> impl IntoView {
    let mut rng = rand::thread_rng();
    let streak = create_rw_signal(cx, 1u32);
    let tails = create_rw_signal(cx, rng.gen_bool(0.5));
    let on_click = move |_| {
        tails.update(|old| {
            let new = rng.gen_bool(0.5);
            if new == *old {
                streak.update(|streak| *streak += 1);
            } else {
                streak.set(1);
            }
            *old = new;
        })
    };
    view! { cx,
    <button on:click=on_click>"Flip coin"</button>
    <h1>{move || if tails.get() {"It is tails"} else {"It is heads"}}</h1>
    <h2>{move || if streak.get() > 1 {format! {"You have a streak of: {}", streak.get()}} else {String::new()} }</h2>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let count = create_rw_signal(cx, 0);
    let on_click = move |_| {
        count.update(|count| *count += 1);
        if count.get() == 10 {
            count.set(0);
        }
    };

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
