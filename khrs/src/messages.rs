use serde_json;

pub type Output<'a, T> = (Option<T>, Message<'a>);
pub type Command<'a, T> = Box<dyn Fn(serde_json::Value, &'a T) -> Output<T>>;

pub enum Message<'a> {
    None,
    Command(&'a str, serde_json::Value),
}

pub struct Handle<'a, T> {
    cmd: Command<'a, T>,
    args: serde_json::Value,
}

impl<'a> Message<'a> {
    pub fn find_handle<T>(self, mapping: Box<dyn Fn(&str) -> Command<T>>) -> Handle<'a, T> {
        match self {
            Message::Command(cmd, args) => Handle::<T> {
                cmd: mapping(cmd),
                args: args,
            },
            Message::None => Handle::<T> {cmd: Box::new(|_val, _env| (Option::None, Message::None)), args: serde_json::Value::Null}
        }
    }
}

impl<'a, T> Handle<'a, T> {
    pub fn run(self, env: &'a T) -> (Option<T>, Message) {
        (self.cmd)(self.args, env)
    }
}