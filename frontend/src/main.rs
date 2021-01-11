#![warn(rust_2018_idioms, unreachable_pub)]
// #![deny(missing_docs, broken_intra_doc_links)]
#![recursion_limit = "1024"]
#![no_std]

extern crate alloc;

use alloc::string::ToString;
use alloc::vec::Vec;

use seed::{*, prelude::*};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc<'_> = wee_alloc::WeeAlloc::INIT;

struct Model {}

enum Msg {}

impl Model {
    fn init(url: Url, orders: &mut impl Orders<Msg>) -> Self {
        Self {}
    }

    fn update(msg: Msg, model: &mut Self, orders: &mut impl Orders<Msg>) {}

    fn view(&self) -> Vec<Node<Msg>> {
        nodes![
            Self::view_header(),
            Self::view_main(),
            Self::view_footer(),  
        ]
    }

    fn view_header() -> Node<Msg> {
        header![
            nav![
                C!["navbar", "navbar-dark bg-dark", "navbar-fixed-top"],
                div![
                    C!["container-fluid"],
                    a![
                        C!["navbar-brand"],
                        attrs! { At::Href => "#" },
                        "Competitive AI",
                        
                    ]
                ]
            ]
        ]
    }

    fn view_main() -> Node<Msg> {
        main![
            C!["container-fluid", "bg-secondary"],
            style! { St::Height => "calc(100vh - 2 * 56px)" },
            
            div![
                C!["container-xxl", "d-flex", "align-items-center", "h-100"],
                div![
                    C!["row", "mx-auto", "mx-md-n5", "h-75", "w-100"],
                    (1..=2).map(Self::view_team_card)
                        
                ],
            ]
            
        ]
    }

    fn view_team_card(team: u8) -> Node<Msg> {
        div![
            C![
                "col", "mx-2", "p-0", "bg-light", "border", "border-light", 
                "border-3", "shadow-lg", "card", "rounded-3", "text-center"
            ], 
             
            div![
                C!["card-header", "bg-light"],
                "Team ",
                team.to_string()
            ],
            div![
                C!["card-body", "p-0"],
                style! { St::Background => "#000" },
                "todo"
            ],
            div![
                C!["card-footer", "bg-light"],
                "footer"
            ]
        ]
    }

    fn view_footer() -> Node<Msg> {
        footer![
            nav![
                C!["navbar", "navbar-dark", "bg-dark", "navbar-fixed-bottom"],
                div![
                    C!["container-fluid"],
                    
                    span![
                        C!["navbar-text", "mx-auto"],
                    
                        "This page was developed by ",
                        a![
                            attrs! { At::Href => "https://github.com/WenwerLars" },
                            "Lars Wenwer"
                        ],
                        " and ",
                        a![
                            attrs! { At::Href => "https://github.com/DzenanJupic" },
                            "Dzenan Jupic"
                        ],
                        a![
                            C!["ms-3", "text-white-50"],
                            attrs! { At::Href => "https://github.com/DzenanJupic/competetive-ai" },
                            "[Source]"
                        ],
                    ]
                ]
            ]
        ]
    }
}


fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    seed::App::start(
        "app",
        Model::init,
        Model::update,
        Model::view,
    );
}
