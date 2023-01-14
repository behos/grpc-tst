// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_CALCULATOR_CALC: ::grpcio::Method<super::calculator::Op, super::calculator::Op> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/calculator.Calculator/calc",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct CalculatorClient {
    client: ::grpcio::Client,
}

impl CalculatorClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        CalculatorClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn calc_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::calculator::Op>, ::grpcio::ClientDuplexReceiver<super::calculator::Op>)> {
        self.client.duplex_streaming(&METHOD_CALCULATOR_CALC, opt)
    }

    pub fn calc(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::calculator::Op>, ::grpcio::ClientDuplexReceiver<super::calculator::Op>)> {
        self.calc_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Calculator {
    fn calc(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::calculator::Op>, sink: ::grpcio::DuplexSink<super::calculator::Op>);
}

pub fn create_calculator<S: Calculator + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_duplex_streaming_handler(&METHOD_CALCULATOR_CALC, move |ctx, req, resp| {
        instance.calc(ctx, req, resp)
    });
    builder.build()
}
