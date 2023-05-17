use yew::{function_component, html, Html, Properties, UseStateHandle};

#[derive(Properties, PartialEq)]
pub struct ErrorDisplayProps {
    pub error: UseStateHandle<Option<String>>,
}

#[function_component(ErrorDisplay)]
pub fn error_display_component(props: &ErrorDisplayProps) -> Html {
    let error = (*(props.error.clone())).clone();
    html! {
        if error.is_some() {
           <p class="text-red-600 pb-2">{error.unwrap()}</p>
        }
    }
}
