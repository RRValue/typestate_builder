use std::marker::PhantomData;

// date we wanto to build up
#[derive(Debug)]
struct HttpResponseData{
    status: String,
    header: Vec<String>,
    body: String,
}

// a module which makes the states private
// states from a state machine "HttpResponseBuilder"
#[doc(hidden)]
mod states {
    pub struct Start {}
    pub struct Status {}
    pub struct Headers {}
    pub struct Body {}
    pub struct Send {}
}

// a module which makes the traits private
#[doc(hidden)]
mod traits {
    use super::states::{Start, Status, Headers, Body, Send};

    // declare states for BuildState
    pub trait BuildState {}
    impl BuildState for Start {}
    impl BuildState for Status {}
    impl BuildState for Headers {}
    impl BuildState for Body {}
    impl BuildState for Send {}
}

// make staes and trait easy reachable
use states::{Start, Status, Headers, Body, Send};
use traits::BuildState;

// Builder structure. Has the data and phantom data, which stores the builder state
pub struct HttpResponseBuilder<S: BuildState> {
    data: HttpResponseData,
    #[doc(hidden)]
    state: PhantomData<S>,
}

impl HttpResponseBuilder<Start> {
    // new builder in state Start, return builder in state Status
    pub fn new() -> HttpResponseBuilder<Status> {
        HttpResponseBuilder::<Status>{
            data: HttpResponseData{
                status: String::new(), 
                header: Vec::new(), 
                body: String::new()
            }, 
            state: PhantomData
        }
    }
}

impl HttpResponseBuilder<Status> {
    // set status data, can only be called once. Return builder in state Header
    pub fn status(mut self, status: String) -> HttpResponseBuilder<Headers> {
        self.data.status = status;
        self.into()
    }
}

impl HttpResponseBuilder<Headers> {
    // set header data, can be called multpile times. Return builder in state Header
    pub fn add_header(mut self, header: String) -> HttpResponseBuilder<Headers> {
        self.data.header.push(header);
        self.into()
    }

    // Say header are done, can be called once. Return builder in stateBody
    pub fn header_done(self) -> HttpResponseBuilder<Body> {
        self.into()
    }
}

impl HttpResponseBuilder<Body> {
    // set body data, can only be called once. Return builder in state Send
    pub fn body(mut self, body: String) -> HttpResponseBuilder<Send> {
        self.data.body = body;
        self.into()
    }
}

impl HttpResponseBuilder<Send> {
    // Consumes the response and sends it. Returns ()
    pub fn send(self) {
        print!("Ab gehts")
    }
}

impl<S> HttpResponseBuilder<S>
    where S: BuildState
{
    // Print is possible in every state
    pub fn print(self) -> HttpResponseBuilder<S>{
        println!("{:?}", self.data);

        self
    }
}

#[doc(hidden)]
impl<S> HttpResponseBuilder<S>
    where S: BuildState
{
    // private helper function transform from one state to another
    fn into<T>(self) -> HttpResponseBuilder<T>
        where T: BuildState {
        HttpResponseBuilder{
            data: self.data, 
            state: PhantomData
        }
    }
}