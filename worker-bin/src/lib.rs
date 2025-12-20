use courtlistener_worker::worker;
use worker::*;

#[event(fetch, respond_with_errors)]
pub async fn main(req: Request, env: Env, ctx: Context) -> worker::Result<Response> {
    courtlistener_worker::worker::main(req, env, ctx).await
}

