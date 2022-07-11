use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlElement};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub title: String,
  pub songs: Vec<String>,
  pub curr_drag: String,
  pub answer_key: Vec<String>,
}

#[derive(Default, Clone)]
struct Model {
  pub songs: Vec<String>,
  pub answer_key: Vec<String>,
}

#[function_component(AlbumForm)]
pub fn album_form(props: &Props) -> Html {
  let state = use_state(|| Model {
    songs: props.songs.clone(),
    answer_key: props.answer_key.clone(),
  });
  let html_state = state.clone();
  let html_props = props.clone();
  let ondrop = Callback::from(move |event: DragEvent| {
    event.prevent_default();
    let target: Option<EventTarget> = event.target();
    let input = target.and_then(|t| t.dyn_into::<HtmlElement>().ok());
    if let Some(input) = input {
      if let Some(input_name) = input.get_attribute("name") {
        log::info!("Update: {:?}", input_name);

        if html_props.answer_key.contains(&html_props.curr_drag) {
          let mut new_songs = state.songs.clone();
          let index = new_songs
            .iter()
            .position(|x| *x == html_props.curr_drag)
            .unwrap();
          new_songs.remove(index);
          state.set(Model {
            answer_key: state.answer_key.to_vec(),
            songs: new_songs,
          })
        }
      }
      return;
    }

    state.set(Model {
      answer_key: state.answer_key.to_vec(),
      songs: state.songs.clone(),
    })
  });

  let ondragover = Callback::from(|event: DragEvent| {
    event.prevent_default();
  });

  let ondragenter = Callback::from(|event: DragEvent| {
    event.prevent_default();
  });

  let curr_answer_form = props.answer_key.iter().enumerate().map(|(x, song)| {
    if html_state.songs.contains(&song) {
        return html! {
            <div name={song.to_string()} class="answer-box" ondragenter={ondragenter.clone()} ondragover={ondragover.clone()} ondrop={ondrop.clone()}>{x + 1}{". "}</div>
        }
    };
    return html! {
        <div class="answer-box">{song}</div>
    };
}).collect::<Html>();

  html! {
    <div class="album-container">
      <h4>{props.title.clone()}</h4>
      <div class="answer-form-container">
        <form class="answer-form">
          {curr_answer_form.clone()}
        </form>
      </div>
  </div>
    }
}
