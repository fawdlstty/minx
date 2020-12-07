use actix::{Actor, StreamHandler};
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, Responder, get};
use actix_web::web;
use actix_web_actors::ws;
use async_trait::async_trait;
use std::collections::HashMap;

use crate::ServiceModule;

pub struct Http {
	m_port: i32,
}

#[async_trait]
impl ServiceModule for Http {
	fn get_name (&self) -> &'static str { "http" }
	async fn async_entry (&self) {
		match HttpServer::new (|| {
			App::new ()
				//.service (minx_hello)
				//.service (minx_ws)
				//.route ("/hey", web::get ().to (manual_hello))
		}).bind ("127.0.0.1:8080") {
		    Ok (mut _server) => { _server.run ().await; () },
		    Err (_e) => println! ("http listen failed: {}", _e.to_string ()),
		};
		let _c = 5;
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



struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle (&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping (msg)) => ctx.pong (&msg),
            Ok(ws::Message::Text (text)) => ctx.text (text),
            Ok(ws::Message::Binary (bin)) => ctx.binary (bin),
            _ => (),
        }
    }
}

#[get ("/minx_ws")]
async fn minx_ws (req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start (MyWs {}, &req, stream)
}
