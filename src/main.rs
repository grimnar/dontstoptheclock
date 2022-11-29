#[cfg(test)] mod tests;

pub mod models;

use chrono::prelude::Utc;
use rocket::{State, Shutdown};
use rocket::fs::{FileServer, relative};

use rocket::serde::json::Json;
use rocket::response::stream::{EventStream, Event};

use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};

use rocket_dyn_templates::{Template, context};

use crate::models::{TimeStamp, RingBuffer, TimeStamps, TimeStampList};


#[rocket::get("/")]
async fn root(list: TimeStamps<'_>) -> Template {
    let list = list.lock().await;
    Template::render("index", context! {timestamps: list.buf.clone()})
}


#[rocket::get("/last")]
async fn last(list: TimeStamps<'_>) -> Option<Json<TimeStamp>> {
    let list = list.lock().await;
    Some(Json(list.last_item()))
}


#[rocket::get("/stop_events")]
async fn events(queue: &State<Sender<TimeStamp>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}

#[rocket::get("/stop/<ts>")]
async fn stop(ts: u64, queue: &State<Sender<TimeStamp>>, list: TimeStamps<'_>) {
    let mut list = list.lock().await;
    list.push(ts);
    let _res = queue.send(list.last_item());
}



#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("templates/static")))
        .mount("/", rocket::routes![root, last, stop, events])
        .manage(channel::<TimeStamp>(1024).0)
        .manage(TimeStampList::new(RingBuffer::new(5, 1662921288)))
        .attach(Template::fairing())
}
