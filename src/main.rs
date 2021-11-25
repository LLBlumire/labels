use serde::Deserialize;
use web_sys::HtmlTextAreaElement;
use yew::{function_component, html, use_node_ref, use_state, Callback};

#[derive(Deserialize)]
struct AddressRecord {
    name: Option<String>,
    address_1: Option<String>,
    address_2: Option<String>,
    city: Option<String>,
    state: Option<String>,
    postal: Option<String>,
    country: Option<String>,
    _country_code: Option<String>,
    _phone_number: Option<String>,
    _delivery_notes: Option<String>,
}

#[function_component(App)]
fn app() -> Html {
    let input = use_node_ref();
    let output = use_state(Vec::<AddressRecord>::new);

    let inputchange = {
        let input = input.clone();
        let output = output.clone();
        Callback::from(move |_| {
            let input = input.cast::<HtmlTextAreaElement>().unwrap().value();
            let mut reader = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(input.as_bytes());
            output.set(
                reader
                    .deserialize()
                    .into_iter()
                    .map(|r| r.unwrap())
                    .collect(),
            )
        })
    };

    let output_display = output.chunks(4).map(|records| {
        let articles = records.iter().map(|record| {
            html! {
                <article>
                    { record.name.as_ref().map(|field| html! { <div>{field}</div> }).unwrap_or_default() }
                    { record.address_1.as_ref().map(|field| html! { <div>{field}</div> }).unwrap_or_default() }
                    { record.address_2.as_ref().map(|field| html! { <div>{field}</div> }).unwrap_or_default() }
                    { record.city.as_ref().map(|field| html! { <div>{field}</div> }).unwrap_or_default() }
                    { record.state.as_ref().map(|field| html! { <div>{field}</div> }).unwrap_or_default() }
                    { record.postal.as_ref().map(|field| html! { <div>{field}</div> }).unwrap_or_default() }
                    { record.country.as_ref().map(|field| html! { <div>{field}</div> }).unwrap_or_default() }
                </article>
            }
        });
        html! {
            <div class="page">
                { for articles }
            </div>
        }
    });

    html! {
        <>
            <div id="input">
                <textarea ref={input} onchange={inputchange}></textarea>
            </div>
            { for output_display }
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
