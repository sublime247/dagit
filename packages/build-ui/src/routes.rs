use bdk::prelude::*;

#[allow(unused)]
use crate::pages::*;
#[derive(Clone, Routable)]
#[rustfmt::skip]
#[derive(PartialEq)]
pub enum Route {
    #[nest("/:lang")]
        #[layout(HeaderLayout)]
            #[route("/")]
            RootPage { lang: Language},
        #[end_layout]
        #[nest("/agits/:agit_id")]
            #[layout(NavigationLayout)]
                #[route("/")]
                    HomePage { lang: Language, agit_id: i64 },
                #[nest("/orders")]
                    #[route("/sales-request")]
                        SalesRequest { lang: Language, agit_id: i64 },
                    #[route("/shipping-label")]
                        ShippingLabel { lang: Language, agit_id: i64 },
                #[end_nest]

                #[nest("/management")]
                    #[route("/artworks")]
                        Artworks { lang: Language, agit_id: i64 },
                    #[route("/collections")]
                        Collections { lang: Language, agit_id: i64 },
                    #[route("/artist")]
                        Artist { lang: Language, agit_id: i64 },
                    #[route("/collectors")]   
                        Collectors { lang: Language, agit_id: i64 },
                #[end_nest]

                #[nest("/hub")]
                    #[route("/dao")]   
                        Dao { lang: Language, agit_id: i64 },
                    #[route("/oracle")]   
                        Oracle { lang: Language, agit_id: i64 },
                    #[route("/faq")]
                    Faq { lang: Language, agit_id: i64 },
                #[end_nest]

                #[nest("/analytics")]
                    #[route("/traffic")]
                        Traffic { lang: Language, agit_id: i64 },
                    #[route("/report")]  
                        Report { lang: Language, agit_id: i64 },
                #[end_nest]

                #[route("/design")]  
                    Design { lang: Language, agit_id: i64 },
                #[route("/extension-tool")]
                    ExtensionTool { lang: Language, agit_id: i64 },
              
              
            #[end_layout]
        #[end_nest]
    #[end_nest]
    #[redirect("/", || Route::RootPage { lang: Language::En })]
    #[route("/:..route")]
    NotFoundPage {
        route: Vec<String>,
    },
}
