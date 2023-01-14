use futures::{SinkExt, StreamExt, TryFutureExt, FutureExt};
use grpcio::{DuplexSink, RequestStream, RpcContext, WriteFlags};

use anyhow::Result;

use self::{
    calculator::Op,
    calculator::{Add, Mul, Op_oneof_op as Oper},
    calculator_grpc::Calculator,
};

mod calculator;
mod calculator_grpc;

pub(crate) use calculator_grpc::create_calculator;

#[derive(Clone)]
pub(crate) struct CalculatorService {}

impl Calculator for CalculatorService {
    fn calc(&mut self, ctx: RpcContext, stream: RequestStream<Op>, sink: DuplexSink<Op>) {
        ctx.spawn(
            handle_stream(stream, sink)
                .map_err(|err| println!("error during operations: {err:?}"))
                .map(|_| ()),
        )
    }
}

async fn handle_stream(mut stream: RequestStream<Op>, mut sink: DuplexSink<Op>) -> Result<()> {
    let mut val = 0;
    while let Some(op) = stream.next().await {
        let op = op?;
        let (res, val_upd) = do_op(op, val);
        val = val_upd;
        sink.send((res, WriteFlags::default())).await?;
    }
    Ok(())
}

fn do_op(req: Op, mut val: i32) -> (Op, i32) {
    match req.op {
        Some(Oper::add(Add { amount, .. })) => {
            val += amount;
            (
                Op {
                    op: Some(Oper::add(Add { amount: val, ..Default::default() })),
                    ..Default::default()
                },
                val,
            )
        }
        Some(Oper::mul(Mul { amount, .. })) => {
            val *= amount;
            (
                Op {
                    op: Some(Oper::mul(Mul { amount: val, ..Default::default() })),
                    ..Default::default()
                },
                val,
            )
        }
        None => (Default::default(), val),
    }
}
