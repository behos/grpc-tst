mod api;

use api::Mul;
use futures::{channel::mpsc, SinkExt};
use tokio::time;
use tonic::transport::Channel;
use anyhow::{Result, Context, bail};

use crate::api::{op::Op as Oper, Add, CalculatorClient, Op};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let mut client = CalculatorClient::connect("http://[::1]:10000").await?;
    let start = time::Instant::now();
    for _ in 0..20_000 {
        assert_eq!(72, run_equation(&mut client).await?);
    }
    let elapsed = start.elapsed();
    println!("ran in {} millis", elapsed.as_millis());
    Ok(())
}

async fn run_equation(client: &mut CalculatorClient<Channel>) -> Result<i32> {
    // a simple equation (4 + 5) * 8
    let (mut snd, rcv) = mpsc::channel(1);
    snd.send(Op {
        op: Some(Oper::Add(Add { amount: 4 })),
    }).await?;
    let mut res = client.calc(rcv).await?.into_inner();
    let _ = res.message().await?.context("no result")?;
    snd.send(Op {
        op: Some(Oper::Add(Add { amount: 5 })),
    }).await?;

    let _ = res.message().await?.context("no result")?;
    snd.send(Op {
        op: Some(Oper::Mul(Mul { amount: 8 })),
    }).await?;

    if let Some(Oper::Mul(Mul { amount })) = res.message().await?.context("no result")?.op {
        Ok(amount)
    } else {
        bail!("oops! wrong response")
    }
}
