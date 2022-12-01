use std::fs::File;
use std::io::Write;
use std::time::Duration;

use diesel::pg::PgConnection;
use job_scheduler::{Job, JobScheduler};
use lettre::message::{header, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use log::{error, info};
use tera::Context;

use crate::config::{Config, SmtpCredentials};
use crate::util::TEMPLATES;
use crate::{api, config};

pub(crate) fn send_email_notification(
    smtp_creds: &SmtpCredentials,
    to: String,
    subject: String,
    content: String,
) {
    info!("Preparing to send email notification to {}.", to);
    let email = Message::builder()
        .from("no-reply@ouafi.net".parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .multipart(
            MultiPart::alternative()
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(String::from(&content)),
                )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(content),
                ),
        )
        .unwrap();

    let creds = Credentials::new(smtp_creds.username.clone(), smtp_creds.password.clone());

    let mailer = SmtpTransport::relay(&smtp_creds.server)
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => info!("Email sent successfully!"),
        Err(e) => error!("Could not send email: {:?}", e),
    }
}

pub(crate) fn store_new_links_job(conn: &mut PgConnection, schedule: &str, mut cfg: &mut Config) {
    let mut scheduler = JobScheduler::new();

    scheduler.add(Job::new(schedule.parse().unwrap(), || {
        api::code_is_valid(&mut cfg);

        if cfg.code.is_none() || !cfg.code_valid.unwrap() {
            match api::obtain_request_code(&mut cfg) {
                Err(err) => error!("Cannot obtain code: {:?}", err),
                Ok(()) => match cfg.code {
                    Some(_) => {
                        api::authorize_app(&mut cfg);
                        match api::obtain_request_token(&mut cfg) {
                            Ok(_) => info!("Received request token."),
                            Err(e) => error!("{}", e),
                        }
                        let config_content = serde_json::to_string_pretty(&cfg).unwrap();
                        match File::create(config::CONFIG_FILE_PATH) {
                            Ok(mut f) => match f.write_all(&config_content.as_bytes()) {
                                Ok(_) => {
                                    info!("Updated config written to disk")
                                }
                                Err(e) => {
                                    error!("Cannot write to file: {:?}", e);
                                }
                            },
                            _ => {}
                        };
                    }
                    None => error!("{}", ""),
                },
            }
        } else {
            info!("{}", "Code is ok");
            match api::obtain_links(conn, &mut cfg) {
                Ok(links) => match &cfg.smtp_credentials {
                    Some(cred) => {
                        let mut context = Context::new();
                        context.insert("links", &links);
                        let body = TEMPLATES.render("links.html", &context);
                        send_email_notification(
                            cred,
                            "mail@ouafi.net".to_string(),
                            "Link Archiver: New links were archived".to_string(),
                            body.unwrap(),
                        )
                    }
                    _ => info!("No SMTP configuration set. Skipping notifications..."),
                },
                Err(e) => error!("{}", e.details),
            }
        }
    }));

    loop {
        scheduler.tick();

        std::thread::sleep(Duration::from_millis(500));
    }
}
