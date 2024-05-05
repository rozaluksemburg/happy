use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// просто комментарий 1 2 3 4 5
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! { 
        <Stylesheet id="leptos" href="/css/style.css" />
        <Title text="Welcome to Leptos"/>
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}


// Определение компонента таблицы
#[component]
fn TableComponent() -> impl IntoView {
    view! {
        <table class="bg-red-500 w-full" text-white border="1">
            <tr>
                <th>{"Заголовок 1"}</th>
                <th>{"Заголовок 2"}</th>
            </tr>
            <tr>
                <td>{"Данные 1 хм"}</td>
                <td>{"Данные 2"}</td>
            </tr>
        </table>
    }
}

#[component]
fn HeaderComponent() -> impl IntoView {
    view! {
        <header class="bg-blue-500 text-white p-4">
            <div class="container mx-auto flex justify-between">
                <h1 class="text-lg font-bold">{"Leptos Market"}</h1>
                <nav>
                    <ul class="flex">
                        <li class="px-4"><a href="/" class="nav-link">{"Home"}</a></li>
                        <li class="px-4"><a href="/products" class="nav-link">{"Products"}</a></li>
                        <li class="px-4"><a href="/about" class="nav-link">{"About Us"}</a></li>
                        <li class="px-4"><a href="/contact" class="nav-link">{"Contact"}</a></li>
                    </ul>
                </nav>
            </div>
        </header>
    }
}


#[component]
fn HomePage() -> impl IntoView {
    // Создание реактивной переменной для отслеживания количества нажатий
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <>
            <HeaderComponent/>
            <div class="container mx-auto mt-8">
                <h1 class="text-xl font-semibold">{"Welcome to Leptos Market!"}</h1>
                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" on:click=on_click>
                    {"Click Me: "}{count}
                </button>
                <TableComponent/>
            </div>
        </>
    }
}

