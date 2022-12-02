use js_sys::Date;
use yew::{html, Component, Context, Html};
use chrono::prelude::{DateTime, NaiveDateTime, Utc};
use gloo::timers::callback::Interval;

pub enum Msg {
    Stop,
    UpdateClock
}

pub struct App{
    last_stop_ts: i64,
    clock_str: String,
    _standalone: Interval,
}

impl App{
    fn update_clock_msg(last_stop_ts: i64) -> String {
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

        if years > 0 {
            result += &format!("{} years ", years);
        }

        if months > 0 {
            result += &format!("{} months ", months);
        }

        if days > 0 {
            result += &format!("{} days ", days);
        }

        if hours > 0 {
            result += &format!("{} hours ", hours);
        }

        if minutes > 0 {
            result += &format!("{} minutes ", minutes);
        }

        result += &format!("and {} seconds", seconds);

        result
    }
}

impl Component for App{
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {

        let clock_handle = {
            let link = ctx.link().clone();
            Interval::new(1, move || link.send_message(Msg::UpdateClock))
        }; 

        let last = 1670004841;
        Self { last_stop_ts: last,
               _standalone: clock_handle,
               clock_str: App::update_clock_msg(last)}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Stop=> {
                self.last_stop_ts = Utc::now().timestamp();
                true
            }
            Msg::UpdateClock=> {
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
              {"since it was stopped"}<br/>
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
