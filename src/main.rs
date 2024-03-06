use pick_a_double::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run()?.await
}
