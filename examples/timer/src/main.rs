use gloo::console::{self, Timer};
use gloo::timers::callback::{Interval, Timeout};
use yew::{html, Component, Context, Html};

pub enum Msg {
    StartNotificationTimeout,
    StartStopwatch,
    ResetStopwatch,
    StopStopwatch,
    NotificationTimeoutDone,
    StopwatchTick,
    UpdateTime,
}

pub struct App {
    time: String,
    message: Option<String>,
    _standalone: (Interval, Interval),
    stopwatch_interval: Option<Interval>,
    notification_timeout: Option<Timeout>,
    console_timer: Option<Timer<'static>>,
    stopwatch_count: u64,
}

impl App {
    fn get_current_time() -> String {
        let date = js_sys::Date::new_0();
        String::from(date.to_locale_time_string("en-US"))
    }

    fn get_stopwatch_string(&self) -> String {
        let seconds = self.stopwatch_count % 60;
        let minutes = (self.stopwatch_count / 60) % 60;
        let hours = (self.stopwatch_count / 3600) % 60;

        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let standalone_handle =
            Interval::new(10, || console::debug!("Example of a standalone callback."));

        let clock_handle = {
            let link = ctx.link().clone();
            Interval::new(1, move || link.send_message(Msg::UpdateTime))
        };

        Self {
            time: App::get_current_time(),
            message: None,
            _standalone: (standalone_handle, clock_handle),
            stopwatch_interval: None,
            notification_timeout: None,
            console_timer: None,
            stopwatch_count: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StartNotificationTimeout => {
                let handle = {
                    let link = ctx.link().clone();
                    Timeout::new(2000, move || {
                        link.send_message(Msg::NotificationTimeoutDone)
                    })
                };

                self.notification_timeout = Some(handle);

                self.console_timer = Some(Timer::new("Timer"));
                true
            }
            Msg::StartStopwatch => {
                let handle = {
                    let link = ctx.link().clone();
                    Interval::new(1000, move || link.send_message(Msg::StopwatchTick))
                };
                self.stopwatch_interval = Some(handle);

                self.message = Some(String::from("Started stopwatch!"));
                ctx.link()
                    .clone()
                    .send_message(Msg::StartNotificationTimeout);

                console::clear!();
                console::warn!("Started stopwatch!");
                true
            }
            Msg::ResetStopwatch => {
                self.stopwatch_count = 0;

                self.message = Some(String::from("Reset stopwatch!"));
                ctx.link()
                    .clone()
                    .send_message(Msg::StartNotificationTimeout);

                console::clear!();
                console::warn!("Reset stopwatch.");
                true
            }
            Msg::StopStopwatch => {
                self.stopwatch_interval = None;

                self.message = Some(String::from("Stopped stopwatch!"));
                ctx.link()
                    .clone()
                    .send_message(Msg::StartNotificationTimeout);

                console::clear!();
                console::warn!("Stopped stopwatch!");
                true
            }
            Msg::NotificationTimeoutDone => {
                // todo weblog
                // ConsoleService::group();
                console::info!("Notification timeout done!");
                if let Some(timer) = self.console_timer.take() {
                    drop(timer);
                }

                self.message = None;

                // todo weblog
                // ConsoleService::group_end();
                true
            }
            Msg::StopwatchTick => {
                //self.messages.push("Tick...");
                self.stopwatch_count += 1;

                // todo weblog
                // ConsoleService::count_named("Tick");
                true
            }
            Msg::UpdateTime => {
                self.time = App::get_current_time();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let stopwatch_running = self.stopwatch_interval.is_some();
        let message = if self.message.is_some() {
            self.message.clone().unwrap()
        } else {
            "".to_string()
        };

        html! {
            <>
                <div id="wrapper">
                    <div class="item-container" id="time">
                        <h3>{ "Clock" }</h3>
                        <h4>{ &self.time }</h4>
                        <div><button style="opacity: 0;">{"."}</button></div>
                    </div>
                    <div class="item-container">
                        <h3>{ "Stopwatch" }</h3>

                        <h4>{ &self.get_stopwatch_string() }</h4>
                        <div class="buttons-container">
                            <button type="button" disabled={stopwatch_running} onclick={ctx.link().callback(|_| Msg::StartStopwatch)}>{ "Start" }</button>
                            <button type="button" disabled={!stopwatch_running} onclick={ctx.link().callback(|_| Msg::StopStopwatch)}>{ "Stop" }</button>
                            <button type="button" disabled={stopwatch_running} onclick={ctx.link().callback(|_| Msg::ResetStopwatch)}>{ "Reset" }</button>
                        </div>
                    </div>
                    <div class="item-container">
                        <h3>{ "Notifications" }</h3>
                        <p id="notification">{ message }</p>
                        <div><button style="opacity: 0;">{"."}</button></div>
                    </div>
                </div>
            </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
