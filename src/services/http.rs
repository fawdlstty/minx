use actix::{Actor, ActorContext, Addr, clock::Instant, prelude::*, Recipient, StreamHandler};
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, Responder, get, web, web::Bytes};
use actix_web_actors::ws;
use async_trait::async_trait;
use std::{collections::HashMap, sync::atomic::AtomicI64, time::Duration, sync::atomic};

use crate::ServiceModule;

pub struct Http {
	m_port: i32,
	m_server: WsChatServer,
}

#[async_trait]
impl ServiceModule for Http {
	fn get_name (&self) -> &'static str { "http" }
	async fn async_entry (&self) {
        let _bind_str = format! ("0.0.0.0:{}", self.m_port);
		match HttpServer::new (|| {
			App::new ()
				.service (minx_hello)
				.service (web::resource ("/minx_ws/").to (minx_ws))
				//.route ("/hey", web::get ().to (manual_hello))
		}).bind (&_bind_str [..]) {
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
		    Ok (_port) => _port,
		    Err (_) => 80,
		};
		Http {
            m_port: _port,
            m_server: WsChatServer::new (),
		}
	}
}



#[get ("/minx_hello")]
async fn minx_hello () -> impl Responder {
    HttpResponse::Ok ().body ("Hello Minx!")
}



pub enum WsPayloadData {
    Text (String),
    Binary (Bytes),
}

#[derive (Message)]
#[rtype (result = "()")]
pub struct WsMessage (pub WsPayloadData);

struct WsChatServer {
	m_inc: AtomicI64,
	m_sessions: HashMap<i64, Recipient<WsMessage>>,
}
impl WsChatServer {
	pub fn new () -> WsChatServer {
		WsChatServer {
			m_inc: AtomicI64::new (0),
			m_sessions: HashMap::new (),
		}
	}
    fn send_string (&self, _msg: String, dest_id: i64) -> bool {
        match self.m_sessions.get (&dest_id) {
            Some (_addr) => match _addr.do_send (WsMessage (WsPayloadData::Text (_msg))) {
                Ok (_) => true,
                Err (_) => false
            },
            None => false,
        }
    }
}
impl Actor for WsChatServer {
   type Context = Context<Self>;
}
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
	//fn started (&mut self, _ctx: &mut Self::Context) {}
	//fn finished (&mut self, _ctx: &mut Self::Context) { _ctx.stop () }
    fn handle (&mut self, _msg: Result<ws::Message, ws::ProtocolError>, _ctx: &mut Self::Context) {
        let _msg = match _msg {
            Ok (_msg) => _msg,
            Err (_) => {
                _ctx.stop ();
                return;
            },
        };
        self.hb = Instant::now ();
        match _msg {
            ws::Message::Ping (_tmp) => _ctx.pong (&_tmp),
            ws::Message::Pong (_) => (),
            ws::Message::Text (_text) => self.addr.do_send (WsStringMessage { id: self.id, msg: _text, }),
            ws::Message::Binary (_bin) => self.addr.do_send (WsBinaryMessage { id: self.id, msg: _bin, }),
            ws::Message::Close (_reason) => { _ctx.close (_reason); _ctx.stop (); },
            ws::Message::Continuation (_) => _ctx.stop (),
            _ => (),
        };
	}
}

async fn minx_ws (_req: HttpRequest, _stream: web::Payload, _srv: web::Data<Addr<WsChatServer>>) -> Result<HttpResponse, Error> {
    let mut _session = WsChatSession {
		id: 0,
		hb: Instant::now (),
		addr: _srv.get_ref ().clone (),
	};
    ws::start (_session, &_req, _stream)
}

struct WsChatSession {
    id: i64,
    hb: Instant,
    addr: Addr<WsChatServer>,
}
impl WsChatSession {
	fn hb (&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval (Duration::from_secs (5), |act, ctx| {
            if Instant::now ().duration_since (act.hb) > Duration::from_secs (10) {
                act.addr.do_send (WsDisconnect { id: act.id });
                ctx.stop ();
                return;
            }
            ctx.ping (b"");
        });
    }
}
impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;
    fn started (&mut self, _ctx: &mut Self::Context) {
        self.hb (_ctx);
        let _addr = _ctx.address ();
        let _val = self.addr.send (WsConnect { addr: _addr.recipient (), });
        _val.into_actor (self).then (|res, act, ctx| {
            match res {
                Ok (res) => act.id = res,
                _ => ctx.stop (),
            }
            fut::ready (())
        }).wait (_ctx);
    }

    fn stopping (&mut self, _: &mut Self::Context) -> Running {
        // notify chat server
        self.addr.do_send (WsDisconnect { id: self.id });
        Running::Stop
    }
}
impl Handler<WsMessage> for WsChatSession {
    type Result = ();
    fn handle (&mut self, _msg: WsMessage, _ctx: &mut Self::Context) {
        match _msg.0 {
            WsPayloadData::Text (_text) => _ctx.text (_text),
            WsPayloadData::Binary (_bin) => _ctx.binary (_bin),
        };
    }
}






#[derive (Message)]
#[rtype (i64)]
pub struct WsConnect {
    pub addr: Recipient<WsMessage>,
}
impl Handler<WsConnect> for WsChatServer {
    type Result = i64;
    fn handle (&mut self, msg: WsConnect, _: &mut Self::Context) -> Self::Result {
        let id = self.m_inc.fetch_add (1, atomic::Ordering::Acquire);
        self.m_sessions.insert (id, msg.addr);
        id
    }
}

#[derive (Message)]
#[rtype (result = "()")]
pub struct WsDisconnect {
    pub id: i64,
}
impl Handler<WsDisconnect> for WsChatServer {
    type Result = ();
    fn handle (&mut self, msg: WsDisconnect, _: &mut Self::Context) {
        self.m_sessions.remove (&msg.id);
    }
}

#[derive (Message)]
#[rtype (result = "()")]
pub struct WsBinaryMessage {
    pub id: i64,
    pub msg: Bytes,
}
impl Handler<WsBinaryMessage> for WsChatServer {
    type Result = ();
    fn handle (&mut self, _msg: WsBinaryMessage, _: &mut Context<Self>) {
        // TODO
        //self.send_message (&msg.room, msg.msg.as_str (), msg.id);
    }
}

#[derive (Message)]
#[rtype (result = "()")]
pub struct WsStringMessage {
    pub id: i64,
    pub msg: String,
}
impl Handler<WsStringMessage> for WsChatServer {
    type Result = ();
    fn handle (&mut self, _msg: WsStringMessage, _: &mut Context<Self>) {
        // TODO
        //self.send_message (&msg.room, msg.msg.as_str (), msg.id);
    }
}
