use crate::types::BeanInfo;
use maud::{html, Markup};

fn editable_div(content: &str, id: &str) -> Markup {
    html! {
        div .flex .grow .items-center .justify-center .max-w-full {
            #(id)
            div .px-1 contenteditable .grow .max-w-full .overflow-hidden .text-center .text-ellipsis .break-normal {(content)}
        }
    }
}

pub fn label(bean_info: &BeanInfo) -> Markup {
    html! {
        script src="https://cdn.tailwindcss.com" {}
        div #label
          .flex .flex-col .justify-between ."w-[696px]" ."h-[401px]" ."text-3xl"
          .text-black .bg-white .border .border-black
        {
            div .flex .flex-row .max-w-full .border-b-4 .border-black {
                div .leading-none .flex .items-center .justify-between .bg-red-600 .text-white .font-bold ."text-[50px]" .border-r .border-black {
                    (editable_div(&bean_info.country, "country"))
                }
                div .flex .grow .max-w-full .items-center .justify-between .border-r .border-black {
                    (editable_div(&bean_info.name, "name"))
                }
                div .flex .grow .max-w-full .items-center .justify-center {
                    (editable_div(&bean_info.roaster, "roaster"))
                }
            }
            div .flex .flex-row .grow .max-w-full .border-b .border-black {
                (editable_div(&bean_info.varietals, "varietals"))
            }
            div .flex .flex-row .grow .max-w-full .border-b .border-black {
                div .flex .grow .items-center .justify-center .border-r .border-black {
                    (editable_div(&bean_info.region, "region"))
                }
                div .flex .grow .items-center .justify-center {
                    (editable_div(&bean_info.farm, "farm"))
                }
            }
            div .flex .flex-row .grow .max-w-full {
                div .flex .flex-col ."w-[22%]" .grow .border-r .border-black {
                    div .flex .grow .items-center .justify-center .border-b .border-black {
                        (editable_div(&bean_info.elevation, "elevation"))
                    }
                    div .flex .grow .items-center .justify-center .border-b .border-black {
                        (editable_div(&bean_info.dose_weight, "dose_weight"))
                    }
                    div .flex .grow .items-center .justify-center {
                        (editable_div(&bean_info.roasting_date, "roasting_date"))
                    }
                }
                div .flex .flex-col ."w-[78%]" .grow {
                    div .flex .grow .items-center .justify-center .border-b .border-black {
                        (editable_div(&bean_info.processing, "processing"))
                    }
                    div .flex .grow .items-center .justify-center {
                        (editable_div(&bean_info.aromatics, "aromatics"))
                    }
                }
            }
        }
    }
}
