use std::error;

use netlify_lambda_http::{
    handler,
    http::Method,
    lambda::{self, lambda, Context},
    IntoResponse, Request, RequestExt,
};

type Error = Box<dyn error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    netlify_lambda::run(handler(hello)).await?;
    Ok(())
}

async fn hello(request: Request, context: Context) -> Result<impl IntoResponse, Error> {
    if request.method() != Method::POST {
        return Ok("Not allowed".to_string());
    }

    Ok(format!(
        "hello {}",
        request
            .query_string_parameters()
            .get("name")
            .unwrap_or_else(|| "stranger")
    ))
}
