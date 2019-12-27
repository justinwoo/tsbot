use actix::prelude::*;

pub type MyError = String;

pub type MyResult<A> = Result<A, MyError>;

#[derive(Debug)]
pub enum Msg {
    Timer,
    User,
    Error(String),
}

#[derive(Debug)]
pub struct RunResult {
    pub from_user: bool,
    pub text: String,
}

impl Message for Msg {
    type Result = ();
}

impl Message for RunResult {
    type Result = ();
}
