use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;


#[component]
pub fn App() -> impl IntoView {
    // Обеспечивает контекст для стилей, заголовков и мета-тегов в приложении
    provide_meta_context();

    view! {
        // Подключение таблицы стилей
        <Stylesheet id="leptos" href="/public/style.css" />
        // Установка заголовка страницы
        <Title text="Welcome to Leptos"/>
        // Маршрутизатор с обработчиком ошибок для несуществующих маршрутов
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
        <table class="bg-red-500" text-white border="1">
            <tr>
                <th>{"Заголовок 1"}</th>
                <th>{"Заголовок 2"}</th>
            </tr>
            <tr>
                <td>{"Данные 1"}</td>
                <td>{"Данные 2"}</td>
            </tr>
        </table>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    // Создание реактивной переменной для отслеживания количества нажатий
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        // Использование компонента таблицы
        <TableComponent/>
    }
}
