use std::collections::HashMap;

pub struct Status {
    code: Code,
    message: String,
}

pub struct Parameters {
    bytes: Vec<u8>,
    tensor_type: String,
}

pub enum Code {
    OK,
    GET_PROPERTIES_NOT_IMPLEMENTED,
    GET_PARAMETERS_NOT_IMPLEMENTED,
    FIT_NOT_IMPLEMENTED,
    EVALUATE_NOT_IMPLEMENTED,
}

pub enum Scalar {
    Double(f64),
    Sint64(i64),
    Bool(bool),
    String_(String),
    Bytes(Vec<u8>),
}

pub enum Reason {
    Unknown = 0,
    Reconnect = 1,
    PowerDisconnected = 2,
    WifiUnavailable = 3,
    Ack = 4,
}
impl From<Reason> for i32 {
    fn from(reason: Reason) -> Self {
        reason as i32
    }
}

pub enum ServerMessage {
    ReconnectIns {
        seconds: i64,
    },
    GetPropertiesIns {
        config: HashMap<String, Scalar>,
    },
    GetParametersIns {
        config: HashMap<String, Scalar>,
    },
    FitIns {
        parameters: Parameters,
        config: HashMap<String, Scalar>,
    },
    EvaluateIns {
        parameters: Parameters,
        config: HashMap<String, Scalar>,
    },
}

pub enum ClientMessage {
    DisconnectRes {
        reason: Reason,
    },
    GetPropertiesRes {
        status: Status,
        properties: HashMap<String, Scalar>,
    },
    GetParametersRes {
        Status: Status,
        parameters: Parameters,
    },
    FitRes {
        status: Status,
        parmeters: Parameters,
        num_examples: i64,
        metrics: HashMap<String, Scalar>,
    },
    EvaluateRes {
        status: Status,
        loss: f32,
        num_examples: i64,
        metrics: HashMap<String, Scalar>,
    },
}
