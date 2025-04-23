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
                        SalesRequestPage { lang: Language, agit_id: i64 },
                    #[route("/shipping-label")]
                        ShippingLabelPage { lang: Language, agit_id: i64 },
                #[end_nest]

                #[nest("/management")]
                    #[route("/artworks")]
                        ArtworkPage { lang: Language, agit_id: i64 },
                    #[route("/collections")]
                        CollectionPage { lang: Language, agit_id: i64 },
                    #[route("/collections/:collection_id")]
                        CollectionDetailPage { lang: Language, agit_id: i64, collection_id:i64 },
                    #[route("/artists")]
                        ArtistPage { lang: Language, agit_id: i64 },
                    #[route("/artists/:artist_id")]
                        ArtistDetailPage { lang: Language, agit_id: i64, artist_id: i64 },
                    #[route("/artists/new_artist")]
                        NewArtistPage { lang: Language, agit_id: i64},
                    #[route("/collectors")]  
                        CollectorsPage { lang: Language, agit_id: i64 },
                    #[route("/collectors/:collector_id")]
                        CollectorDetailPage { lang: Language, agit_id: i64, collector_id: i64 },
                #[end_nest]

                #[nest("/hub")]
                    #[route("/dao")]   
                        DaoPage { lang: Language, agit_id: i64 },
                    #[route("/oracle")]   
                        OraclePage { lang: Language, agit_id: i64 },
                    #[route("/faq")]
                    FaqPage { lang: Language, agit_id: i64 },
                #[end_nest]

                #[nest("/analytics")]
                    #[route("/traffic")]
                        TrafficPage { lang: Language, agit_id: i64 },
                    #[route("/report")]  
                        ReportPage { lang: Language, agit_id: i64 },
                #[end_nest]

                #[route("/design")]  
                    DesignPage { lang: Language, agit_id: i64 },
                #[route("/extension-tool")]
                    ExtensionToolPage { lang: Language, agit_id: i64 },
            #[end_layout]
        #[end_nest]
    #[end_nest]
    #[redirect("/", || Route::RootPage { lang: Language::En })]
    #[route("/:..route")]
    NotFoundPage {
        route: Vec<String>,
    },
}
