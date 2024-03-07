use crate::{components::label, types::BeanInfo};
use maud::{html, Markup};

pub fn label_form(bean_info: &BeanInfo) -> Markup {
    html! {
        form
          hx-post="/api/submit_label_form"
          #label_form
          hx-target="#label"
          hx-vals=r"js:{
            country: htmx.find('#country').innerHTML,
            name: htmx.find('#name').innerHTML,
            roaster: htmx.find('#roaster').innerHTML,
            varietals: htmx.find('#varietals').innerHTML,
            region: htmx.find('#region').innerHTML,
            farm: htmx.find('#farm').innerHTML,
            elevation: htmx.find('#elevation').innerHTML,
            dose_weight: htmx.find('#dose_weight').innerHTML,
            roasting_date: htmx.find('#roasting_date').innerHTML,
            processing: htmx.find('#processing').innerHTML,
            aromatics: htmx.find('#aromatics').innerHTML
          }"
          .flex .flex-col .space-y-4 .items-center
        {
            (label(&bean_info))
        }

        div
          .flex .flex-col .space-y-4 .items-center
        {
            form
              #bq_form
              hx-post="/api/submit_bq_url"
              hx-target="#label"
              .flex .flex-row .space-x-4
            {
                div .flex .flex-col {
                    label for="bq_url" { "Beanconquerer URL"}
                    input
                      #bq_url name="url" value="https://beanconqueror.com?shareUserBean0=ChVNdXN0YWbDoSBFc3RhdGUgRGVjYWYSABoYMjAyNC0wMS0zMVQyMzowMDowMC4wMDBaIgAqBEFNT0MyADgAQABIAFIAWhZzd2VldCwgdmFuaWxsYSwgdG9mZmVlYEhoAHADggEAiAEBkgEsaHR0cHM6Ly9hbWF0dGVyb2Zjb25jcmV0ZS5jb20vcHJvZHVjdC9kZWNhZi+aAQCgAQCqAVYKCENvbG9tYmlhEglSaXNhcmFsZGEaD011c3RhZsOhIEVzdGF0ZSIAKgZ+MTYwMG0yBDIwMjM6CENhc3RpbGxvQhJFQSBzdWdhcmNhbmUgZGVjYWZKALABAroBFggAEAAaACAAKAAwADoAQABIAFAAWADCAQDIAQDQAQDaARYIABAAGAAgACgAMAA4&shareUserBean1=AEAASABQAFgA4gECCgA="
                      .bg-gray-700
                    ;
                }
                div .flex .flex-col .w-fit .max-w-full {
                    label for="bq_dose_weight" {"dose weight (g)"}
                    input
                      #bq_dose_weight name="dose_weight" .w-32 type="number" value="12"
                      .bg-gray-700
                    ;
                }
            }
            div .flex .flex-row .space-x-4 {
                button type="submit" form="bq_form" .w-fit .bg-gray-600 .border boder-gray-300 .p-2 { "Load from Beanconquerer URL" }
                button type="submit" form="label_form" .w-fit .text-md .border .bg-gray-600 .border-gray-300 .p-2 {"Print"}
            }
        }

    }
}
