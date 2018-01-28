use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;

use super::js;

type Message = String;

pub struct WebSocket {
    handle: js::SocketId,
    open: Rc<RefCell<bool>>,
    incoming: Rc<RefCell<Vec<Message>>>,

    onopen_handle: Box<Any>,
    onmessage_handle: Box<Any>,
}

impl WebSocket {
    pub fn connect(url: &str) -> Result<WebSocket, ()> {
        let handle = js::websocket_create(url);

        let open = Rc::new(RefCell::new(false));
        let open_cb = Rc::clone(&open);
        let onopen_handle = unsafe {
            js::websocket_onopen(handle, move || {
                *open_cb.borrow_mut() = true;
            })
        };

        let incoming = Rc::new(RefCell::new(Vec::new()));
        let incoming_cb = Rc::clone(&incoming);
        let onmessage_handle = unsafe {
            js::websocket_onmessage(handle, move |msg| {
                incoming_cb.borrow_mut().push(String::from(msg))
            })
        };

        Ok(WebSocket {
            handle: handle,
            open: open,
            incoming: incoming,

            onopen_handle: Box::new(onopen_handle),
            onmessage_handle: Box::new(onmessage_handle),
        })
    }

    pub fn open(&self) -> bool {
        *self.open.borrow()
    }

    pub fn send(&self, msg: &str) -> Result<(), ()> {
        js::websocket_send(self.handle, msg);
        Ok(())
    }

    pub fn next<'a>(&'a mut self) -> Option<Message> {
        let mut incoming = self.incoming.borrow_mut();
        let v = incoming.drain(0..).next();
        v
    }
}

impl Drop for WebSocket {
    fn drop(&mut self) {
        js::websocket_close(self.handle, 1000, "WebSocket dropped");
    }
}
