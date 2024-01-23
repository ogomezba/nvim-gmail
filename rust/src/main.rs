use std::sync::{Arc, Mutex};

mod command;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use command::Command;
use nvim_rs::rpc::IntoVal;
use nvim_rs::Value;
use rs_gmail::inbox::{EmailHeader, GmailInbox, Uid};
use tokio::io::Stdout;

use nvim_rs::{compat::tokio::Compat, create::tokio as create, Handler, Neovim};

#[derive(Clone)]
struct NeovimHandler {
    gmail: Arc<Mutex<Option<GmailInbox>>>,
}

impl NeovimHandler {
    async fn start(&self, username: String, pass: String) -> Result<Value> {
        let gmail = GmailInbox::new(username, pass);
        self.gmail
            .lock()
            .map_err(|_| anyhow!("Unexpected error"))?
            .replace(gmail);
        Ok(Value::Nil)
    }

    async fn get_last_emails(&self) -> Result<Value> {
        let (headers, _) = self
            .gmail
            .lock()
            .map_err(|_| anyhow!("Unexpected error"))?
            .as_mut()
            .ok_or(anyhow!(
                "start() command needs to be invoked before using any other command"
            ))?
            .get_last_emails()?;

        let headers = parse_headers(headers);

        Ok(headers.into_val())
    }


    async fn get_email_body(&self, uid: Uid) -> Result<Value> {
        let body = self
            .gmail
            .lock()
            .map_err(|_| anyhow!("Unexpected error"))?
            .as_mut()
            .ok_or(anyhow!(
                "start() command needs to be invoked before using any other command"
            ))?
            .get_email_info(uid)?;

        Ok(Value::from(body))
    }

    async fn send_email(&self, to: String, subject: String, body: String) -> Result<Value> {
        self.gmail
            .lock()
            .map_err(|_| anyhow!("Unexpected error"))?
            .as_mut()
            .ok_or(anyhow!(
                "start() command needs to be invoked before using any other command"
            ))?
            .send_email(&to, &subject, body)?;

        Ok(Value::Nil)
    }
}

#[async_trait]
impl Handler for NeovimHandler {
    type Writer = Compat<Stdout>;

    async fn handle_request(
        &self,
        name: String,
        args: Vec<Value>,
        _: Neovim<Compat<Stdout>>,
    ) -> Result<Value, Value> {
        let command = Command::try_from_request(name, args).map_err(err_to_val)?;

        Ok(match command {
            Command::Start { username, pass } => {
                self.start(username, pass).await.map_err(err_to_val)?
            }
            Command::GetEmails => self.get_last_emails().await.map_err(err_to_val)?,
            Command::GetEmailBody(uid) => self.get_email_body(uid).await.map_err(err_to_val)?,
            Command::SendEmail { to, body, subject } => self
                .send_email(to, subject, body)
                .await
                .map_err(err_to_val)?,
            Command::Unknown => todo!(),
        })
    }
}

fn err_to_val(e: anyhow::Error) -> Value {
    Value::from(e.to_string())
}

fn parse_headers(headers: Vec<EmailHeader>) -> Vec<Value> {
    headers
        .iter()
        .map(|header| {
            Value::Map(vec![
                ("subject".into_val(), header.subject.clone().into_val()),
                ("from".into_val(), header.from.clone().into_val()),
                ("uid".into_val(), (header.uid as i64).into_val()),
                ("date".into_val(), (header.date.clone()).into_val()),
            ])
        })
        .collect()
}

#[tokio::main]
async fn main() {
    let handler: NeovimHandler = NeovimHandler {
        gmail: Arc::new(Mutex::new(None)),
    };
    let (_, io_handler) = create::new_parent(handler).await;

    let _ = io_handler.await;
}
