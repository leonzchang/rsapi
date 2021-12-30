mod api;
mod app;
mod table;
mod utils;

fn main() {
    let opts = app::Opts {
        database_url: "postgres://postgres:test123@localhost:5432/domain?sslmode=disable"
            .to_string(),
    };
    app::run(opts);
}
