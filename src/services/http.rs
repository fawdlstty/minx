use actix::{Actor, ActorContext, Addr, Recipient, StreamHandler, clock::Instant};
use actix::prelude::*;
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, Responder, get};
use actix_web::web;
use actix_web_actors::ws;
use async_std::sync::Mutex;
use async_trait::async_trait;
use std::{sync::atomic::AtomicI64, collections::HashMap, collections::HashSet, sync::Arc};

use crate::ServiceModule;

pub struct Http {
	m_port: i32,
	m_server: ChatServer::new ().start (),
}

#[async_trait]
impl ServiceModule for Http {
	fn get_name (&self) -> &'static str { "http" }
	async fn async_entry (&self) {
		match HttpServer::new (|| {
			App::new ()
				.service (minx_hello)
				.service (web::resource ("/minx_ws/").to (minx_ws))
				//.route ("/hey", web::get ().to (manual_hello))
		}).bind ("127.0.0.1:8080") {
		    Ok (mut _server) => { _server.run ().await; () },
		    Err (_e) => println! ("http listen failed: {}", _e.to_string ()),
		};
	}
	async fn async_send (&self, _msg: String) -> bool {
		false
	}
}

impl Http {
	pub fn new (_param: &HashMap<String, String>) -> Http {
		let _port = match _param ["port"].to_string ().parse::<i32> () {
		    Ok(_port) => _port,
		    Err(_) => 80,
		};
		Http {
			m_port: _port
		}
	}
}



#[get ("/minx_hello")]
async fn minx_hello () -> impl Responder {
    HttpResponse::Ok ().body ("Hello Minx!")
}



#[derive (Message)]
#[rtype (result = "()")]
pub struct WsMessage (pub String);

struct ChatServer {
	m_inc: Arc<AtomicI64>,
	m_sessions: HashMap<i64, Recipient<WsMessage>>,
}
impl ChatServer {
	pub fn new () -> ChatServer {
		ChatServer {
			m_inc: Arc::new (AtomicI64::new (0)),
			m_sessions: HashMap::new (),
		}
	}
}

impl Actor for ChatServer {
   type Context = ws::WebsocketContext<Self>;
}
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatServer {
	fn started (&mut self, _ctx: &mut Self::Context) {
		// let mut _map: HashMap<i64, &Self::Context> = HashMap::new ();
		// let mut _xx: &Self::Context = _map.get (&2333).unwrap ();
		// _xx.text ("aaas");
	}

    fn handle (&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping (msg)) => ctx.pong (&msg),
            Ok(ws::Message::Text (text)) => ctx.text (text),
            Ok(ws::Message::Binary (bin)) => ctx.binary (bin),
            _ => (),
        }
	}

	fn finished (&mut self, ctx: &mut Self::Context) {
        ctx.stop ()
    }
}

// https://github.com/actix/examples/blob/master/websocket-chat/src/main.rs
// https://github.com/actix/examples/blob/master/websocket-chat/src/server.rs
async fn minx_ws (req: HttpRequest, stream: web::Payload, _srv: web::Data<Addr<ChatServer>>) -> Result<HttpResponse, Error> {
    ws::start (WsChatSession {
		id: 0,
		hb: Instant::now (),
		name: None,
		addr: _srv.get_ref ().clone (),
	}, &req, stream)
}

struct WsChatSession {
    id: i64,
    hb: Instant,
    name: Option<String>,
    addr: Addr<ChatServer>,
}
impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        self.hb(ctx);

        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        let addr = ctx.address();
        self.addr
            .send(server::Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat server
        self.addr.do_send(server::Disconnect { id: self.id });
        Running::Stop
    }
}

// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct Message(pub String);



// pub struct ChatServer {
//     sessions: HashMap<i64, Recipient<Message>>,
// }

// impl ChatServer {
//     pub fn new () -> ChatServer {
//         ChatServer {
//             sessions: HashMap::new (),
//         }
//     }
// }

// impl ChatServer {
//     fn send_string(&self, _msg: String, dest_id: i64) -> bool {
// 		match self.sessions.get (&dest_id) {
// 		    Some(_addr) => match _addr.do_send (Message (_msg)) {
// 			    Ok(_) => true,
// 			    Err(_) => false
// 			},
// 		    None => false,
// 		}
//     }
// }

// impl Actor for ChatServer {
//     type Context = Context<Self>;
// }

// #[derive(Message)]
// #[rtype(i64)]
// pub struct Connect {
//     pub addr: Recipient<Message>,
// }
// impl Handler<Connect> for ChatServer {
//     type Result = i64;

//     fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
//         let id = self.rng.gen::<usize>();
//         self.sessions.insert (id, msg.addr);
//         id
//     }
// }

// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct ClientMessage {
//     pub id: i64,
//     pub msg: String,
// }
// impl Handler<ClientMessage> for ChatServer {
//     type Result = ();

//     fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
//         //self.send_message(&msg.room, msg.msg.as_str(), msg.id);
//     }
// }
