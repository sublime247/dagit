use bdk::prelude::{by_components::icons::{arrows, validations}, *};

use crate::{pages::agits::_id::management::artists::{controllers::Controller, i18n::ArtistTranslate}, routes::Route};
#[component]
pub fn NewArtistPage(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    let _tr: ArtistTranslate = translate(&lang);
    let mut ctrl = Controller::new(lang, agit_id)?;
    let _profile_picture = use_signal(|| None::<String>);

    // Handle form submission
    let handle_save = move |_| {
       // todo: similate api this
    };

    rsx! {
        div { class: "w-full min-h-screen bg-background h-full flex text-white items-center",
            div { class: "flex flex-col w-full h-full max-w-4xl p-6",
                // Header with title and back button
                div { class: "flex items-center mb-6",
                    Link {
                        to: Route::ArtistPage { lang, agit_id: agit_id() },
                        class: "text-gray-400 hover:text-white mr-4",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            class: "h-6 w-6",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M15 19l-7-7 7-7"
                            }
                        }
                    }

                    h1 { class: "text-2xl font-bold", "Add Artist" }
                }

                p { class: "text-gray-400 mb-8",
                    "Register a new artist on the platform. Showcase their work, share exhibition history, and connect with curators and galleries."
                }

                // Artist Info Section
                div { class: "mb-8",
                div { class: "flex items-center mb-4",
                h2 { class: "text-xl font-semibold", "Artist Info" }
                arrows::ChevronDown { class: "ml-2 [&>path]:stroke-white", height: 20, width: 20 }
            }

            // Form fields
            div { class: "space-y-4",
                div { class: "flex items-center",
                    label { class: "text-sm w-40",
                        span { class: "text-red-500 mr-1", "*" }
                        "Display Name or Email"
                    }
                    input {
                        class: "bg-border-background border border-border-primary text-white p-2.5 flex-1",
                        placeholder: "Input",
                        r#type: "text",
                        value:"{ctrl.artist_input_field().display_name.clone()}",
                        oninput: move |evt| ctrl.update_artist_field("display_name".to_string(), evt.value().clone())
                    }
                }

                div { class: "flex items-center",
                    label { class: "text-sm w-40", "Social Media" }
                    input {
                        class: "bg-border-background border border-border-primary text-white p-2.5 flex-1",
                        placeholder: "Input",
                        r#type: "text",
                        value:"{ctrl.artist_input_field().social_media.clone()}",
                        oninput: move |evt| ctrl.update_artist_field("social_media".to_string(), evt.value().clone())
                    }
                }

                div { class: "flex items-center",
                    label { class: "text-sm w-40",
                        span { class: "text-red-500 mr-1", "*" }
                        "Medium"
                    }
                    input {
                        class: "bg-border-background border border-border-primary text-white p-2.5 flex-1",
                        placeholder: "Input",
                        r#type: "text",
                        value:"{ctrl.artist_input_field().medium.clone()}",
                        oninput: move |evt| ctrl.update_artist_field("medium".to_string(), evt.value().clone())
                    }
                }

                div { class: "flex items-center",
                    label { class: "text-sm w-40",
                        span { class: "text-red-500 mr-1", "*" }
                        "Theme"
                    }
                    input {
                        class: "bg-border-background border border-border-primary text-white p-2.5 flex-1",
                        placeholder: "Input",
                        r#type: "text",
                        value:"{ctrl.artist_input_field().theme.clone()}",
                        oninput: move |evt| ctrl.update_artist_field("theme".to_string(), evt.value().clone())
                  
                    }
                }

                div { class: "flex items-center",
                    label { class: "text-sm w-40",
                        span { class: "text-red-500 mr-1", "*" }
                        "Art Style"
                    }
                    input {
                        class: "bg-border-background border border-border-primary text-white p-2.5 flex-1",
                        placeholder: "Input",
                        r#type: "text",
                        value:"{ctrl.artist_input_field().art_style.clone()}",
                        oninput: move |evt| ctrl.update_artist_field("art_style".to_string(), evt.value().clone())
                    }
                }
            }


                        // Profile Picture Upload
                        div { class: "flex flex-col mt-8",
                            label { class: "mb-1 text-sm", "Profile Picture" }
                            div {
                                class: "border border-dashed border-green-500 flex flex-col items-center justify-center text-center text-center w-[340px] h-[340px]",

                                // You would implement file upload functionality here
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    class: "h-12 w-12 text-gray-400 mb-2",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    stroke: "currentColor",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "1",
                                        d: "M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                                    }
                                }
                                p { class: "text-sm text-gray-400",
                                    "Please drag and drop your file here."
                                }
                                p { class: "text-xs text-gray-500",
                                    "One or more files (JPG, PNG)"
                                }
                            }
                            p { class: "text-xs text-gray-500 mt-1",
                                "Maximum resolution: 3000px × 3000px (square format)"
                            }
                            p { class: "text-xs text-gray-500",
                                "Uploads are allowed under 10MB"
                            }
                        }
                }

                // Introduction Section
                div { class: "mb-8 mt-8",
                    div { class: "flex items-center mb-4",
                        h2 { class: "text-xl font-semibold", "Introduction" }
                        arrows::ChevronDown { class: "ml-2 [&>path]:stroke-white", height: 20, width: 20 }
                    }

                    textarea {
                        class: "bg-border-background border border-border-primary text-white p-4 w-full h-32",
                        placeholder: "Please feel free to describe your artistic work, style, and creative philosophy. It would be wonderful if you could share the message behind your art and the unique aspects of your work.",
                        value: "{ctrl.artist_input_field().introduction.clone()}",
                      
                        oninput: move |evt| {
                            ctrl.update_artist_field("introduction".to_string(), evt.value().clone())
                        }

                    }
                }

                // Biography Section
                div { class: "mt-8",
                    div { class: "flex items-center mb-4",
                        h2 { class: "text-xl font-semibold", "Biography" }
                        arrows::ChevronDown { class: "ml-2 [&>path]:stroke-white", height: 20, width: 20 }
                    }

                    textarea {
                        class: "bg-border-background border border-border-primary text-white p-4 w-full h-32",
                        placeholder: "Please provide a brief introduction about yourself, including your background, career, and areas of expertise. Sharing who you are and what experiences have shaped you would be greatly appreciated.",
                        value: "{ctrl.artist_input_field().biography.clone()}",
                        oninput: move |evt| {
                            ctrl.update_artist_field("biography".to_string(), evt.value().clone())
                        }
                    }
                }

                // Action buttons
                div { class: "flex justify-end space-x-4 mt-8",
                    button {
                        class: "border border-white text-white px-6 py-2 flex items-center justify-center",
                        onclick: handle_save,
                        "Save"
                    }

                    button {
                        class: "border border-white text-white px-6 py-2 flex items-center justify-center",
                        onclick: move |_| {
                                ctrl.create_artist();
                        },
                        validations::Add { class: "mr-2 [&>path]:stroke-white [&>circle]:stroke-white" }
                        "Add Artwork"
                    }
                }
            }
        }
    }
}
