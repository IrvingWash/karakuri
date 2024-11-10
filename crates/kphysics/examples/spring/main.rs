use app::App;

mod app;

pub fn main() {
    let mut app = App::new();

    app.setup();

    while app.is_running() {
        app.input();
        app.update();
        app.render();
    }
}
