use js_sys::Date;
use yew::{html, Component, Context, Html};
use chrono::prelude::{DateTime, NaiveDateTime, Utc};
use gloo::timers::callback::Interval;
use gloo::net::http::Request;
use gloo::net::Error;
use gloo::console;
use wasm_bindgen_futures::spawn_local;

use gloo::net::eventsource::futures::EventSource;
use futures::{stream, StreamExt};
use serde::{Serialize, Deserialize};

use serde_json;


#[derive(Serialize, Deserialize)]
pub struct TimeStamp {
    id: u64, 
    last_stop_ts: i64
}

pub enum Msg {
    Stop,
    SetClock(i64),
    UpdateClockMsg
}

pub struct App{
    last_stop_ts: Option<i64>,
    clock_str: String,
    _standalone: Interval,
    _eventlistener: EventSource,
}


impl App{
    fn update_clock_msg(last_stop_ts: Option<i64>) -> String {
        let last_stop_ts = match last_stop_ts {Some(ts) => ts,
                                               None => return String::from("NaN")
                                               };
        let now = Utc::now();
        let last_stop = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(last_stop_ts, 0).unwrap(), Utc);

        let diff = now.signed_duration_since(last_stop);

        let days = diff.num_days();
        let years = days / 365;
        let hours = diff.num_hours() % 24;
        let remaining_days = days % 365;
        let months = remaining_days / 31;
        let days = remaining_days % 31;
        let minutes = diff.num_minutes() % 60;
        let seconds = diff.num_seconds() % 60;

        let mut result = String::new();
        let mut more_than_seconds = false;

        if years > 0 {
            result += &format!("{} years ", years);
            more_than_seconds = true;
        }

        if months > 0 {
            result += &format!("{} months ", months);
            more_than_seconds = true;
        }

        if days > 0 {
            result += &format!("{} days ", days);
            more_than_seconds = true;
        }

        if hours > 0 {
            result += &format!("{} hours ", hours);
            more_than_seconds = true;
        }

        if minutes > 0 {
            result += &format!("{} minutes ", minutes);
            more_than_seconds = true;
        }

        if more_than_seconds {
            result += "and ";
        }
        result += &format!("{} seconds ", seconds);

        result
    }

    async fn stop_clock() -> Result<i64, Error> {
        let stop_ts = Utc::now().timestamp();
        let _resp_future = Request::get(&format!("stop/{}", stop_ts))
                                   .send()
                                   .await?;
        Ok(stop_ts)
    }


    fn set_event_listener(ctx: &Context<Self>) -> EventSource {
        let mut es = EventSource::new("stop_events").unwrap();
        let stream_msg = es.subscribe("message").unwrap();
        let stream_open = es.subscribe("open").unwrap();
        let stream_err = es.subscribe("error").unwrap();
        let _stop_event_handler = {
            let link = ctx.link().clone();
            spawn_local(async move {
                let mut all_streams = stream::select_all([stream_msg, stream_open, stream_err]);
                while let Some(Ok((event_type, msg))) = all_streams.next().await {
                    match &*event_type {
                        "open" => console::debug!("EventStream init complete"),
                        "message" => {let ts : TimeStamp = serde_json::from_str(&msg.data().as_string().unwrap()).unwrap();
                                      console::debug!("Received msg event with ts: ", ts.last_stop_ts);
                                      link.send_message(Msg::SetClock(ts.last_stop_ts));
                                      },
                        "error" => console::debug!("Error from eventstream! {:?}", msg),
                        &_ => console::debug!("unknown event type received!")
                    }
                }
                console::debug!("EventSource Closed");
            })
        };

        es
    }
}

impl Component for App{
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {

        let clock_handle = {
            let link = ctx.link().clone();
            Interval::new(100, move || link.send_message(Msg::UpdateClockMsg))
        }; 

        Self { last_stop_ts: None,
               _standalone: clock_handle,
               _eventlistener: App::set_event_listener(ctx),
               clock_str: String::new()}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetClock(ts) => {
                self.last_stop_ts = Some(ts);
                true
            }
            Msg::Stop=> {
                spawn_local(async {
                    match App::stop_clock().await {
                        Ok(ts) => console::debug!(format!("Successfully sent {}", ts)),
                        Err(_) => console::error!("Could not send stop/<ts>")
                    }
                });
                true
            }
            Msg::UpdateClockMsg=> {
                self.clock_str = App::update_clock_msg(self.last_stop_ts);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
          <main>
            <h1 class="time">{"DON'T STOP THE CLOCK!"}<br/>
              <span onclick={ctx.link().callback(|_| Msg::Stop)} id="time">{&self.clock_str}</span>
              <br/>{"since it was stopped"}
            </h1>
                // Display the current date and time the page was rendered
             <p class="footer">
                { "Rendered: " }
                { String::from(Date::new_0().to_string()) }
            </p>
          </main>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
