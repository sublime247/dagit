use bdk::prelude::by_components::icons::{arrows, edit, settings, validations};
use bdk::prelude::*;
use crate::routes::Route;

use super::i18n::ArtistTranslate;

use super::controllers::Controller;
#[component]
#[allow(unused_variables)]
pub fn ArtistPage(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    let ctrl = Controller::new(lang, agit_id)?;
    let tr: ArtistTranslate= translate(&lang);
    let view_mode = use_signal(|| "table");
    let  search_query = use_signal(String::new);
    let artists = ctrl.artist();
  rsx!{
    div { 
      class: "w-full min-h-screen bg-background h-full flex text-white justify-center items-center",
     div{ class:"flex flex-col w-full h-full",
      div { 
          class: "flex flex-col mb-6",
          h1 { 
              class: "text-2xl font-bold",
              {tr.artists}
          }
          p { 
              class: "text-sm text-gray-400",
              "1,201 Total Artists"
          }
      }

      div {
        class: "flex flex-col md:flex-row justify-between mb-6 gap-4",

        // View mode buttons
        div {
            class: "flex space-x-2",



                 // Filter dropdown
                 button {
                    class: "p-2 border border-border-primary text-white w-full sm:w-auto",
                    // onclick: move |_| show_filters.toggle(),
                    settings::Sliders { class: "[&>path]:stroke-white" }
                }
            // All dropdown
            div { class: "relative",
            div { class: "absolute inset-y-0 right-0 flex items-center pr-3 pointer-events-none",
                arrows::ChevronDown{ class: "[&>path]:stroke-white", height: 20, width: 20 }
            }
            input {
                class: "bg-border-background border border-border-primary text-white text-sm rounded-none block w-full pl-3 p-2.5",
                placeholder: "All",
                r#type: "text",
            }
        }

        }

        // Search
        div {
            class: "relative flex-1 mr-4",
            div { class: "absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none",
            edit::Search { class: "[&>path]:stroke-white [&>circle]:stroke-white" }
        }
        input {
            class: "bg-border-background border border-border-primary text-white text-sm rounded-none block w-full pl-10 p-2.5",
            placeholder: "Search",
            r#type: "text",
        }
        }
        button {
          class: "bg-border-background border border-border-primary text-white px-4 py-2 flex items-center justify-center w-full sm:w-auto",
          onclick: move |_| {
              ctrl.open_new_artist_form();
          },
          validations::Add { class: "mr-3 [&>path]:stroke-white [&>circle]:stroke-white" }
          {tr.new_artist}
      }
    }


      div { 
          class: " flex-1 overflow-x-auto",
          table { 
              class: "w-full text-sm text-left border-collapse min-w-[800px]",
              thead { class: "text-xs text-white capitalize bg-table-background border-b border-border-primary",
                  tr { 

                      th { 
                          class:"px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                         div{  class: "flex items-center",
                          span{{tr.artist}}
                          arrows::UpDown { class: "[&>path]:stroke-white [&>circle]:stroke-white", height:18, width:18 }
                         }
                      }
                      th { 
                          class:"px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                          div{  class: "flex items-center",
                           span{{tr.revenue}}
                           arrows::UpDown { class: "[&>path]:stroke-white [&>circle]:stroke-white",  height:18, width:18 }
                          }
                      }
                      th { 
                          class:"px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                          div{  class: "flex items-center",
                           span{{tr.artworks}}
                           arrows::UpDown { class: "[&>path]:stroke-white [&>circle]:stroke-white",  height:18, width:18 }
                          }
                      }
                      th { 
                          class:"px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                          div{  class: "flex items-center",
                           span{{tr.featured_work}}
                           arrows::UpDown { class: "[&>path]:stroke-white [&>circle]:stroke-white",  height:18, width:18 }
                          }
                      }
                      th { 
                        class:"px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                        "Attributes"
                    }
                      th { 
                          class:"px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                          div{  class: "flex items-center",
                           span{{tr.status}}
                           arrows::UpDown { class: "[&>path]:stroke-white [&>circle]:stroke-white",  height:18, width:18 }
                          }
                      }
                      th { 
                          class:"px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                          div{  class: "flex items-center",
                           span{{tr.social_media}}
                           arrows::UpDown { class: "[&>path]:stroke-white [&>circle]:stroke-white",  height:18, width:18 }
                          }
                      }
                  
                      th { 
                          class:"px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                          div{  class: "flex items-end",
                           span{""}
                           validations::Extra { class: "[&>circle]:stroke-white", height:18 }
                          }

                      }
                  }
              }
              tbody { 
               {   artists.into_iter().enumerate().map(|(_index, artist)| {
                      rsx! {
                        
                        
                          tr { 
                              key: "owned-{artist.id}",
                              class: "hover:bg-gray-900 cursor-pointer",
                          onclick: move |_| {
                            let artist_id = artist.id.clone();
                              use_navigator().push(Route::ArtistDetailPage { 
                                  lang: lang, 
                                  agit_id: agit_id(), 
                                  artist_id: artist_id.parse::<i64>().unwrap()
                              });
                          },
                             
                              td { 
                                  class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                  div { 
                                      class: "flex items-center",
                                      div { class: "w-8 h-8 bg-white rounded-full mr-2 flex items-center justify-center"}
                                      div { 
                                          class: "flex items-center",
                                          div {  class:"flex flex-col",
                                          span { {artist.name.clone()} }
                                          span { {artist.mail.clone()} }

                                          }
                                      }
                                  }
                              }
                              td { 
                                  class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                  div { 
                                      class: "flex flex-col",
                                      span { "{artist.revenue} ETH" }
                                      span { class: "text-xs text-gray-500", "${artist.revenue_usd}" }
                                  }
                              }
                              td { 
                                class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                {artist.artwork.clone()}
                            }
                              td { 
                                  class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                  {artist.featured_work.to_string()}
                              }
                              td { 
                                  class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                  div { 
                                      class: "flex space-x-1",
                                  {artist.attributes.iter().map(|attribute| {
                                          rsx! {
                                              span { 
                                                  class: "text-xs px-2 py-1 rounded border border-white",
                                                  "{attribute}"
                                              }
                                          }
                                      })
                                   } }
                              }
                            
                              td { 
                                  class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                  {artist.status.clone()}
                              }
                              td { 
                                  class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                  {artist.social_media.clone()}
                              }
                              td { 
                                  class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                                  button { 
                                      class: "text-gray-400 hover:text-white",
                                      validations::Extra { class: "[&>circle]:stroke-white", height:18 }

                                  }
                              }
                          }
                           }
                 })
              }
          }
          }
      }
  }
}
  }
}
