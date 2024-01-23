use anyhow::{anyhow, Result};
use nvim_rs::Value;
use rs_gmail::inbox::Uid;

pub enum Command {
    Start {
        username: String,
        pass: String,
    },
    GetEmails,
    GetEmailBody(Uid),
    SendEmail {
        to: String,
        body: String,
        subject: String,
    },
    Unknown,
}

impl Command {
    pub fn try_from_request(name: String, args: Vec<Value>) -> Result<Self> {
        Ok(match name.as_ref() {
            "start" => {
                let username = get_str_param(&args, 0, "Expected username")?;
                let pass = get_str_param(&args, 1, "Expected password")?;
                Self::Start { username, pass }
            }
            "get_last_emails" => Self::GetEmails,
            "get_email_body" => {
                let uid = get_nbr_param(&args, 0, "Expected Uid")?;
                Self::GetEmailBody(uid)
            }
            "send_email" => {
                let to: String = get_str_param(&args, 0, "Expected To")?;
                let subject: String = get_str_param(&args, 1, "Expected Subject")?;
                let body: String = get_str_param(&args, 2, "Expected Body")?;

                Self::SendEmail { to, body, subject }
            }
            _ => Self::Unknown,
        })
    }
}

fn get_nbr_param(args: &Vec<Value>, idx: usize, err: &str) -> Result<u32> {
    Ok(args
        .get(idx)
        .ok_or(anyhow!(err.to_owned()))?
        .as_u64()
        .ok_or(anyhow!("Invalid type. Expected Number"))? as u32)
}

fn get_str_param(args: &Vec<Value>, idx: usize, err: &str) -> Result<String> {
    Ok(args
        .get(idx)
        .ok_or(anyhow!(err.to_owned()))?
        .as_str()
        .ok_or(anyhow!("Invalid type. Expected String"))?
        .to_owned())
}
