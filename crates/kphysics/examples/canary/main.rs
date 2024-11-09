use app::App;

mod app;

pub fn main() {
    dbg!("hey");
    let mut app = App::new();

    app.setup();

    while app.is_running() {
        app.input();
        app.update();
        app.render();
    }

    dbg!(app.is_running());
}
