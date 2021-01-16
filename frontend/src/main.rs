#![feature(once_cell)]
#![warn(rust_2018_idioms, unreachable_pub)]
// #![deny(missing_docs, broken_intra_doc_links)]
#![recursion_limit = "1024"]

use seed::{*, prelude::*};

mod space_invaders;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc<'_> = wee_alloc::WeeAlloc::INIT;

struct Model {
    page: Page,
    space_invaders: crate::space_invaders::Model,
}

enum GMsg {
    SpaceInvaders(crate::space_invaders::Msg)
}

enum Page {
    Home,
    SpaceInvaders,
}

impl Model {
    fn init(_url: Url, orders: &mut impl Orders<GMsg>) -> Self {
        Self {
            // todo
            page: Page::SpaceInvaders,
            space_invaders: crate::space_invaders::Model::new(orders),
        }
    }

    fn update(msg: GMsg, model: &mut Self, orders: &mut impl Orders<GMsg>) {
        match msg {
            GMsg::SpaceInvaders(msg) => {
                model.space_invaders.update(msg, orders);
            }
        }
    }

    fn view(&self) -> Vec<Node<GMsg>> {
        nodes![
            Self::view_header(),
            self.view_main(),
            Self::view_footer(),
        ]
    }

    fn view_header() -> Node<GMsg> {
        header![
            nav![
                C!["navbar", "navbar-dark", "bg-dark", "navbar-fixed-top"],
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

    fn view_main(&self) -> Node<GMsg> {
        main![
            C!["container-fluid", "bg-secondary", "p-0"],
            style! { St::Height => "calc(100vh - 2 * 56px)" },
            
            div![
                C!["row", "m-0", "w-100", "h-100", "d-flex", "align-items-center", "justify-content-evenly"],
                (1..=1).map(|i| self.view_team_card(i))
            ],
        ]
    }

    fn view_team_card(&self, team: u8) -> Node<GMsg> {
        div![
            C![
                "col", "card", "p-0", "mx-3", "bg-light", "border", "border-light", "border-3", "rounded-3",
                "shadow-lg", "text-center"
            ],
            style! { St::MaxWidth => "60vh" }, 
             
            div![
                C!["card-header", "bg-light"],
                "Team ",
                team.to_string()
            ],
            div![
                C!["card-body", "position-relative"],
                style! {
                    St::Background => "#000",
                    St::Height => "0",
                    St::Overflow => "hidden",
                    St::PaddingTop => "calc(4 / 3 * 100%)" 
                },
                self.space_invaders
                    .view()
                    .map_msg(GMsg::SpaceInvaders)
            ],
            div![
                C!["card-footer", "bg-light"],
                "footer"
            ]
        ]
    }

    fn view_footer() -> Node<GMsg> {
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
