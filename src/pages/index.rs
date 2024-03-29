use maud::{html, Markup, DOCTYPE};

use crate::{components, types::BeanInfo};

pub fn index() -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            title { "CoffeeLabeler" }
            link href="https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100;0,300;0,400;0,500;0,700;0,900;1,100;1,300;1,400;1,500;1,700;1,900&display=swap" rel="stylesheet";
            script src="https://unpkg.com/htmx.org@1.9.10" {}
            script src="https://unpkg.com/htmx.org/dist/ext/json-enc.js" {}
            script src="https://unpkg.com/hyperscript.org@0.9.12" {}
            script src="https://cdn.tailwindcss.com" {}
        }
        body .flex .flex-col .items-center .bg-gray-800 .text-gray-300 .text-base .space-y-8 .p-12 .min-w-0 {
            h1 .text-5xl { "CoffeeLabeler"}
            (components::label_form(&BeanInfo::default()))
        }
    }
}
