use courtlistener_worker::worker;
use worker::*;

#[event(fetch, respond_with_errors)]
pub async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    worker::main(req, env, ctx).await
}




