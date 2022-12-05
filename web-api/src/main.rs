mod schemes;
mod middlewares;

use kv_log_macro::info;
use tide::utils::After;
use tide::{Response, Result};

#[async_std::main]
async fn main() -> Result<()> {
    femme::start();
    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());

    app.with(After(|res: Response| async move {
        info!("ok");
        Ok(res)
    }));
    app.at("/")
        .get(|_| async move { Ok(format!("Hello {}, this was request number {}!", 0, 0)) });

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
