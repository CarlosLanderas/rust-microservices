#[macro_use]
extern crate nickel;
extern crate lettre;

use failure::{Error, format_err};
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use lettre::{SendableEmail, EmailAddress, Envelope};
use nickel::{TemplateCache, ReloadPolicy, Nickel, HttpRouter, Response, MiddlewareResult, Request, FormBody};
use nickel::hyper::status::StatusCode;
use lettre_email::error::Error::Envelope;
use std::collections::HashMap;

struct Data {
    sender: Mutex<Sender<SendableEmail>>,
    cache: TemplateCache,
}

fn main() {

    let tx = spawn_sender();

    let data = Data {
        sender: Mutex::new(tx),
        cache: TemplateCache::with_policy(ReloadPolicy::Always),
    };

    let mut server = Nickel::with_data(data);
    server.get("/", middleware!("Mailer Microservice"));
    server.post("/send", send);
    server.listen("127.0.0.1:8002").unwrap();
}

fn send<'mw>(req: &mut Request<Data>, res: Response<'mw, Data>) -> MiddlewareResult<'mw, Data> {
    try_with!(res, send_impl(req).map_err(|_| StatusCode::BadRequest));
    res.send("true")
}

fn send_impl(req: &mut Request<Data>) -> Result<(), Error> {
    let (to, code) = {
        let params = req.form_body().map_err(|_| format_err!(""))?;
        let to = params.get("to").ok_or(format_err!("to field not set"))?.to_owned();
        let code = params.get("code").ok_or(format_err!("code field not set"))?.to_owned();
        (to, code)
    };

    let data = req.server_data();
    let to = EmailAddress::new(to)?;
    let envelope = Envelope::new(None, vec![to])?;
    let mut params: HashMap<&str, &str> = HashMap::new();
    params.insert("code", &code);
    let mut body : Vec<u8> = Vec::new();

    data.cache.render("templates/confirm.tpl", &mut body, &params)?;
    let email = SendableEmail::new(envelope, "Confirm email".to_string(), Vec::new());
    let sender = data.sender.lock().unwrap().clone();
    sender.send(email).map_err(|_| format_err!("can't send email"))?;
    Ok(())
}
