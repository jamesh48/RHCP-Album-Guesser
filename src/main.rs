mod album_form;
use crate::album_form::AlbumForm;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlElement};
use yew::prelude::*;

#[derive(Default, Clone)]
struct Model {
    pub songs: Vec<String>,
    pub curr_drag: String,
    pub answer_key: Vec<String>,
}

struct Album {
    pub title: String,
    pub songs: Vec<String>,
}

#[function_component(App)]
fn app() -> Html {
    let albums = vec![
        Album {
            title: "Blood Sex Sugar Magik".to_string(),
            songs: [
                "The Power of Equality".to_string(),
                "If you have to Ask".to_string(),
                "Breaking the Girl".to_string(),
                "Funky Monks".to_string(),
                "Suck my Kiss".to_string(),
                "I could have Lied".to_string(),
                "Mellowship slinky in B Major".to_string(),
                "The Righteous & the Wicked".to_string(),
                "Give it away".to_string(),
                "Blood sex sugar magik".to_string(),
                "Under the Bridge".to_string(),
                "Naked in the Rain".to_string(),
                "Apache Rose Peacock".to_string(),
                "The Greeting Song".to_string(),
                "My Lovely Man".to_string(),
                "Sir Psycho Sexy".to_string(),
                "They're Red Hot".to_string(),
            ]
            .to_vec(),
        },
        Album {
            title: "Californication".to_string(),
            songs: [
                "Around the World".to_string(),
                "Parallel Universe".to_string(),
                "Scar Tissue".to_string(),
                "Otherside".to_string(),
                "Get On Top".to_string(),
                "Californication".to_string(),
                "Easily".to_string(),
                "Porcelain".to_string(),
                "Emit Remmus".to_string(),
                "I Like Dirt".to_string(),
                "This Velvet Glove".to_string(),
                "Savior".to_string(),
                "Purple Stain".to_string(),
                "Right on Time".to_string(),
                "Road Trippin'".to_string(),
            ]
            .to_vec(),
        },
        Album {
            title: "By the way".to_string(),
            songs: [
                "By the way".to_string(),
                "Universally Speaking".to_string(),
                "This is the place".to_string(),
                "Dosed".to_string(),
                "Dont forget me".to_string(),
                "The Zephyr song".to_string(),
                "Can't stop".to_string(),
                "I could die for you".to_string(),
                "Midnight".to_string(),
                "Throw away your television".to_string(),
                "Cabron".to_string(),
                "Tear".to_string(),
                "On Mercury".to_string(),
                "Minor Thing".to_string(),
                "Warm Tape".to_string(),
                "Venice Queen".to_string(),
            ]
            .to_vec(),
        },
    ];
    let state = use_state(|| Model {
        songs: [
            "Around the World".to_string(),
            "Parallel Universe".to_string(),
            "Scar Tissue".to_string(),
            "Otherside".to_string(),
            "Get On Top".to_string(),
            "Californication".to_string(),
            "Easily".to_string(),
            "Porcelain".to_string(),
            "Emit Remmus".to_string(),
            "I Like Dirt".to_string(),
            "This Velvet Glove".to_string(),
            "Savior".to_string(),
            "Purple Stain".to_string(),
            "Right on Time".to_string(),
            "Road Trippin'".to_string(),
        ]
        .to_vec(),
        answer_key: [
            "Around the World".to_string(),
            "Parallel Universe".to_string(),
            "Scar Tissue".to_string(),
            "Otherside".to_string(),
            "Get On Top".to_string(),
            "Californication".to_string(),
            "Easily".to_string(),
            "Porcelain".to_string(),
            "Emit Remmus".to_string(),
            "I Like Dirt".to_string(),
            "This Velvet Glove".to_string(),
            "Savior".to_string(),
            "Purple Stain".to_string(),
            "Right on Time".to_string(),
            "Road Trippin'".to_string(),
        ]
        .to_vec(),
        curr_drag: "".to_string(),
    });
    let html_state = state.clone();
    let cloned_state = state.clone();

    let ondragstart = Callback::from(move |event: DragEvent| {
        let target: Option<EventTarget> = event.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlElement>().ok());
        if let Some(input) = input {
            if let Some(input_name) = input.get_attribute("name") {
                let state = cloned_state.clone();
                state.set(Model {
                    curr_drag: input_name.to_string(),
                    answer_key: state.answer_key.to_vec(),
                    songs: cloned_state.songs.clone(),
                })
            }
        }
    });

    let html_albums = albums
        .iter()
        .map(|album| {
            html! {
                <AlbumForm title={album.title.clone()} curr_drag={html_state.curr_drag.clone()} songs={album.songs.clone()} answer_key={album.songs.clone()} />
            }
        })
        .collect::<Html>();

    let songs_only = [
        albums[0].songs.clone(),
        albums[1].songs.clone(),
        albums[2].songs.clone(),
    ]
    .concat();

    let curr_answer_pool = songs_only.to_vec().iter().map(|song| {
        html! {
            <div draggable="true" class="answer-candidate" ondragstart={ondragstart.clone()} name={song.to_string()}>{song}</div>
        }
    }).collect::<Html>();

    html! {
        <div class="parent-container">
        <h4>{"RHCP Album Guesser"}</h4>
        <div class="main">
            <div class="albums-container">
                {html_albums}
            </div>
            <div class="answer-pool-container">
                {curr_answer_pool}
            </div>
        </div>

        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
