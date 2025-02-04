mod model;

use hirola::prelude::*;
use model::Users;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

async fn fetcher() -> Result<Users, JsValue> {
    let window = web_sys::window().unwrap();

    let mut opts = RequestInit::new();
    opts.method("GET");
    let url = format!("https://jsonplaceholder.typicode.com/users");
    let request = Request::new_with_str_and_init(&url, &opts)?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    let json = resp.json()?;
    let json = JsFuture::from(json).await?;
    let users: Users = json.into_serde().unwrap();
    Ok(users)
}

fn fetch_users(_app: &HirolaApp) -> Dom {
    let users: AsyncResult<Users> = use_async(fetcher());

    html! {
            <div class="grid h-screen place-items-center">
                {if users.get().is_none() {
                    html!{
                        <div class="h-10 w-32">"Loading..."</div>
                    }
                }  else {
                    let users = &*users.get();
                    let users = users.clone().unwrap();

                    html! {
                        <div class="grid h-screen place-items-center">
                                {for user in users.unwrap() {
                                    html! {
                                        <div>
                                            {user.name.clone()}
                                        </div>
                                    }
                                }}
                        </div>
                    }
                }}
           </div>
    }
}

fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let app = HirolaApp::new();
    app.mount(&body, fetch_users);
}
