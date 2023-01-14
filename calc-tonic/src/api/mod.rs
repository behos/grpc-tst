#[allow(non_camel_case_types)]
mod calculator;

use std::pin::Pin;

use futures::{Stream, StreamExt};
use tonic::{async_trait, Request, Response, Status, Streaming};

use self::calculator::{Op, Add, op::Op as Oper, Mul, calculator_server::Calculator};

pub(crate) use self::calculator::calculator_server::CalculatorServer;
pub(crate) struct CalculatorService {}

#[async_trait]
impl Calculator for CalculatorService {
    type calcStream = Pin<Box<dyn Stream<Item = Result<Op, Status>> + Send + 'static>>;

    async fn calc(
        &self,
        request: Request<Streaming<Op>>,
    ) -> Result<Response<Self::calcStream>, Status> {
        let mut stream = request.into_inner();
        let output = async_stream::try_stream! {
            let mut val = 0;
            while let Some(req) = stream.next().await {
                let req = req?;
                let (response, val_upd) = do_op(req, val);
                val = val_upd;
                yield response;
            }
        };

        Ok(Response::new(Box::pin(output) as Self::calcStream))
    }
}

fn do_op(req: Op, mut val: i32) -> (Op, i32) {
    match req.op {
        Some(Oper::Add(Add{ amount })) => {
            val += amount;
            (Op { op: Some(Oper::Add(Add{ amount: val })) }, val)
        },
        Some(Oper::Mul(Mul{ amount })) => {
            val *= amount;
            (Op { op: Some(Oper::Mul(Mul{ amount: val })) }, val)
        },
        None => (Op { op: None }, val),
    }
}
