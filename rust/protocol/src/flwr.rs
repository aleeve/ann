#[cfg(feature = "grpc")]
tonic::include_proto!("flwr.proto");

#[cfg(not(feature = "grpc"))]
include!(concat!(env!("OUT_DIR"), "/flwr.proto.rs"));

// use client_message::FitRes;
// use crate::flwr::server_message;
// use crate::flwr::{ClientMessage as ClientMessage_rpc, ServerMessage as ServerMessage_rpc};
// use crate::ws_messages::{ClientMessage as ClientMessage_ws, ServerMessage as ServerMessage_ws};

// impl From<ClientMessage_ws> for ClientMessage_rpc {
//     fn from(value: ClientMessage_ws) -> Self {
//         match value {
//             ClientMessage_ws::DisconnectRes { reason } => {
//                 let ds = client_message::DisconnectRes {
//                     reason: reason.into(),
//                 };
//             }
//             ClientMessage_ws::FitRes {
//                 status,
//                 parmeters,
//                 num_examples,
//                 metrics,
//             } => {
//                 let fr = client_message::FitRes {
//                     status: Some(status.into()),
//                     parameters: Some(parmeters.into()),
//                     num_examples,
//                     metrics: todo!(),
//                 };
//             }
//             ClientMessage_ws::EvaluateRes {
//                 status,
//                 loss,
//                 num_examples,
//                 metrics,
//             } => todo!(),
//             ClientMessage_ws::GetPropertiesRes { status, properties } => todo!(),
//             ClientMessage_ws::GetParametersRes { Status, parameters } => todo!(),
//         };
//         Self::default()
//     }
// }

// impl Into<ServerMessage_ws> for ServerMessage_rpc {
//     fn into(self: ServerMessage_rpc) -> ServerMessage_ws {
//         let msg = self.msg.unwrap();
//         match msg {
//             server_message::Msg::FitIns(msg) => {} // server_message::GetPropertiesIns { config } => {}
//             server_message::Msg::GetPropertiesIns(msg) => {} // server_message::GetPropertiesIns { config } => {}
//             server_message::Msg::GetParametersIns(msg) => {} // server_message::GetPropertiesIns { config } => {}
//             server_message::Msg::EvaluateIns(msg) => {} // server_message::GetPropertiesIns { config } => {}
//             server_message::Msg::FitIns(msg) => {} // server_message::GetPropertiesIns { config } => {}
//             server_message::Msg::ReconnectIns(msg) => {} // server_message::GetPropertiesIns { config } => {}
//         };
//         todo!()
//     }
// }
