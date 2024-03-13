use crate::{components::label, types::BeanInfo};
use maud::{html, Markup};

pub fn label_form(bean_info: &BeanInfo) -> Markup {
    let label_htmx_vals = r"{
        name: htmx.find('#name').innerHTML,
        country: htmx.find('#country').innerHTML,
        roaster: htmx.find('#roaster').innerHTML,
        varietals: htmx.find('#varietals').innerHTML,
        region: htmx.find('#region').innerHTML,
        farm: htmx.find('#farm').innerHTML,
        elevation: htmx.find('#elevation').innerHTML,
        dose_weight: htmx.find('#dose_weight').innerHTML,
        roasting_date: htmx.find('#roasting_date').innerHTML,
        processing: htmx.find('#processing').innerHTML,
        aromatics: htmx.find('#aromatics').innerHTML
      }";

    html! {
        (label(&bean_info))
        div
          .flex .flex-col .space-y-4 .items-center
        {
            form
              hx-post="/api/load_from_bq"
              hx-target="#label"
              hx-include="[name='dose_weight']"
              _="on htmx:afterOnLoad remove @hidden from #print_button then add @hidden to #validate_button"
              .flex .flex-row .space-x-4 .items-end .border .border-gray-600 .p-3 .w-full .justify-center
            {
                div .flex .flex-col .grow {
                    label for="bq_url" { "Beanconquerer Share Link"}
                    input #bq_url
                      name="url"
                      onfocus="this.value=''"
                      value="https://beanconqueror.com?shareUserBean0=ChVNdXN0YWbDoSBFc3RhdGUgRGVjYWYSABoYMjAyNC0wMS0zMVQyMzowMDowMC4wMDBaIgAqBEFNT0MyADgAQABIAFIAWhZzd2VldCwgdmFuaWxsYSwgdG9mZmVlYEhoAHADggEAiAEBkgEsaHR0cHM6Ly9hbWF0dGVyb2Zjb25jcmV0ZS5jb20vcHJvZHVjdC9kZWNhZi+aAQCgAQCqAVYKCENvbG9tYmlhEglSaXNhcmFsZGEaD011c3RhZsOhIEVzdGF0ZSIAKgZ+MTYwMG0yBDIwMjM6CENhc3RpbGxvQhJFQSBzdWdhcmNhbmUgZGVjYWZKALABAroBFggAEAAaACAAKAAwADoAQABIAFAAWADCAQDIAQDQAQDaARYIABAAGAAgACgAMAA4&shareUserBean1=AEAASABQAFgA4gECCgA="
                      .bg-gray-700
                    ;
                }
                button type="submit"
                  .w-fit .h-fit .bg-gray-600 .border boder-gray-300 .p-1
                { "Load" }
            }
            div .flex .flex-row .space-x-4 .items-end .border .border-gray-600 .p-3 .justify-stretch .w-full {
                div .flex .flex-col {
                    label for="dose_weight_input" {"dose weight"}
                    div {
                        input
                          _="on change put me.value + 'g' into #dose_weight"
                          #dose_weight_input name="dose_weight" type="number" value="12" step="0.5"
                          .bg-gray-700 .w-20
                        ;"(g)"
                    }
                }
                div .flex .flex-row .border-l .border-gray-600 .space-x-2 .items-end .justify-between .ps-4 .grow {
                    div .flex .flex-col {
                        label for="no_pages" {"copies"}
                        input
                          #no_pages name="no_pages" type="number" value="1" min="1" max="255"
                          .bg-gray-700 .w-12
                        ;
                    }
                    button
                      #print_button
                      hx-post="/api/print_label" hx-ext="json-enc" hx-target="#label"
                      hx-vals={"js:{bean_info:"(label_htmx_vals)", no_pages: Number(htmx.find('#no_pages').value)}"}
                      hx-confirm="Do you really want to print?"
                      .w-fit .h-fit .text-md .border .bg-gray-600 .border-gray-300 .p-1
                      _="on input from #label add @hidden"
                    { "Print" }
                    button hidden
                      #validate_button
                      hx-post="/api/update_label"
                      hx-ext="json-enc" hx-target="#label" hx-vals={"js:"(label_htmx_vals)}
                      .w-fit .h-fit .text-md .border .bg-gray-600 .border-gray-300 .p-1
                      _="on input from #label remove @hidden
                         on htmx:afterOnLoad remove @hidden from #print_button then add @hidden"
                    { "Validate" }
                }
            }
        }
    }
}
