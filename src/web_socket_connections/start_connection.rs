use crate::web_socket_connections::{ws::WsConn,lobby::Lobby};
use actix::Addr;
use actix_web::{get,web::{Data,Payload,Path},Error,HttpResponse,HttpRequest};
use actix_web_actors::ws;
use uuid::Uuid;

#[get("/{group_id}")]
// #[get("/conn_to_ws")]
pub async fn start_connection(
    req:HttpRequest,
    stream:Payload,
    path:Path<Uuid>,
    srv:Data<Addr<Lobby>>
) -> Result<HttpResponse,Error>{
    let group_id = path.into_inner();
    // let group_id = Uuid::new_v4();
    let ws = WsConn::new(
        group_id,
        srv.get_ref().clone(), 
    );
    let resp = ws::start(ws, &req, stream)?;

    Ok(resp)
}
