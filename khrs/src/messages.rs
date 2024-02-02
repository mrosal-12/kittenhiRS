use serde_json;

pub type Output<T> = (Option<T>, Message);          //This is what the handler outputs
pub type Command<T> = Box<dyn Fn(serde_json::Value, &T) -> Output<T>>;      //This what creates output

//Message enum (if there is a message, what does it say)
pub enum Message {
    None,
    Command(String, serde_json::Value),
}

//Hanlde struct (stores both cmd and the arguments)
pub struct Handle<T> {
    cmd: Command<T>,
    args: serde_json::Value,
}

impl Message {
    //try to find the handle based on the module's available functions
    pub fn find_handle<T>(self, mapping: Box<dyn Fn(&str) -> Command<T>>) -> Handle<T> {
        match self {
            Message::Command(cmd, args) => Handle::<T> {
                cmd: mapping(cmd),
                args: args,
            },

            //if Message is none, essentially return an empty handle
            Message::None => Handle::<T> {cmd: Box::new(|_val, _env| (Option::None, Message::None)), args: serde_json::Value::Null}
        }
    }

    //create message from string tuple
    pub fn from(cmd: &str, args: &str) -> Message {
        if cmd == "" {Message::None}    //if no cmd, no message
        else {
            Message::Command(
                String::from(cmd),
                serde_json::from_str(args),
            )
        }
    }
}

impl<T> Handle<T> {
    //run Handle with environment, return Message and whether env changes
    pub fn run(self, env: &T) -> (Option<T>, Message) {
        (self.cmd)(self.args, env)
    }
}