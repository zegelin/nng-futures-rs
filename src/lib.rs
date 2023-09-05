use std::{task::{Poll, Waker, ready}, sync::Arc, pin::Pin};
use futures::{Sink, Stream, executor::block_on, Future, lock::Mutex};
use nng::{Socket, Aio, AioResult, Message, options::{Options, protocol::pubsub::Subscribe, Opt}};


struct PubSinkState {
    poll: Poll<Result<(), nng::Error>>,
    waker: Option<Waker>
}

pub struct PubSink {
    socket: Socket,
    aio: Aio,
    state: Arc<futures::lock::Mutex<PubSinkState>>
}

impl PubSink {
    pub fn new(url: &str) -> Result<Self, nng::Error> {
        let socket = Socket::new(nng::Protocol::Pub0)?;

        socket.listen(url)?;

        let state = Arc::new(Mutex::new(PubSinkState { poll: Poll::Ready(Ok(())), waker: Default::default() }));

        let aio = Aio::new({
            let state = state.clone();

            move |_, res| {
                if let AioResult::Send(res) = res {
                    let mut state = block_on(state.lock());

                    state.poll = Poll::Ready(res.map_err(|(_, err)| err));

                    if let Some(waker) = state.waker.take() {
                        waker.wake();
                    }
                } else {
                    unreachable!("unexpected AioResult for a send-only socket")
                }
            }
        })?;

        Ok(Self { socket, aio, state })
    }
}

impl<Tx> Sink<Tx> for PubSink where
    Tx: Into<Message>
{
    type Error = nng::Error;

    fn poll_ready(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        let mut state = self.state.lock();
        let state = Pin::new(&mut state);
        let mut state = ready!(state.poll(cx));

        state.waker = Some(cx.waker().clone());

        let res = ready!(state.poll);

        state.poll = Poll::Pending;

        Poll::Ready(res)
    }

    fn start_send(self: std::pin::Pin<&mut Self>, item: Tx) -> Result<(), Self::Error> {
        self.socket.send_async(&self.aio, item).map_err(|(_, err)| err)
    }

    fn poll_flush(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        let mut state = self.state.lock();
        let state = Pin::new(&mut state);
        let mut state = ready!(state.poll(cx));

        state.waker = Some(cx.waker().clone());

        state.poll
    }

    fn poll_close(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        let mut state = self.state.lock();
        let state = Pin::new(&mut state);
        let mut state = ready!(state.poll(cx));

        state.waker = Some(cx.waker().clone());

        state.poll
    }
}


struct SubStreamState {
    poll: Poll<Result<nng::Message, nng::Error>>,
    waker: Waker
}

pub struct SubStream {
    socket: Socket,
    aio: Aio,
    state: Arc<Mutex<Option<SubStreamState>>>
}

impl SubStream {
    pub fn new(url: &str) -> Result<Self, nng::Error> {
        let socket = Socket::new(nng::Protocol::Sub0)?;

        socket.dial_async(url)?;

        let state: Arc<Mutex<Option<SubStreamState>>> = Arc::new(Mutex::new(None));

        let aio = Aio::new({
            let state = state.clone();

            move |_, res| {
                if let AioResult::Recv(res) = res {
                    let mut state = block_on(state.lock());

                    if let Some(state) = state.as_mut() {
                        state.poll = Poll::Ready(res);

                        state.waker.wake_by_ref();

                    } else {
                        panic!("got AioResult when nobody is waiting!")
                    }
                    
                } else {
                    unreachable!("unexpected AioResult for a recv-only socket")
                }
            }
        })?;

        Ok(Self { socket, aio, state })
    }

    pub fn subscribe(&self, val: <Subscribe as Opt>::OptType) -> Result<(), nng::Error> {
        self.socket.set_opt::<Subscribe>(val)
    }
}

impl Stream for SubStream {
    type Item = Result<nng::Message, nng::Error>;

    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        let mut state = self.state.lock();
        let state = Pin::new(&mut state);
        let mut state = ready!(state.poll(cx));

        match state.as_mut() {
            None => {
                *state = Some(SubStreamState { poll: Poll::Pending, waker: cx.waker().clone() });

                if let Err(err) = self.socket.recv_async(&self.aio) {
                    return Poll::Ready(Some(Err(err)))
                }

                Poll::Pending
            },
            Some(SubStreamState { poll: Poll::Pending, waker }) => {
                // update the waker
                *waker = cx.waker().clone();
                
                Poll::Pending
            },
            Some(SubStreamState { poll: Poll::Ready(_), waker: _ }) => {
                let state = state.take().unwrap(); // reset state to None

                state.poll.map(|res| Some(res))
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // i would love to write some tests, but don't have a clue where to start

}