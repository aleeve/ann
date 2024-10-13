use protocol::flwr::client_message::Msg as ClientMsg;
use protocol::flwr::server_message::Msg as ServerMsg;
use crate::logic::model::client;

async fn handle_server_message(msg: ServerMsg) -> ClientMsg {
    match msg {
        ServerMsg::EvaluateIns(m) => (),
        ServerMsg::FitIns(m) => (),
        ServerMsg::ReconnectIns(m) => (),
        ServerMsg::GetPropertiesIns(m) => (),
        ServerMsg::GetParametersIns(m) => (),
    }
    todo!("hmm")
}
