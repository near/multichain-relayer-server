// example below:
// from https://docs.rs/ethers/latest/ethers/middleware/trait.Middleware.html#required-associated-types
use ethers_providers::{Middleware, MiddlewareError, PendingTransaction};
use ethers_core::types::{BlockId, Bytes, H160, H256, U256};
use thiserror::Error;
use async_trait::async_trait;

#[derive(Debug)]
pub(crate) struct MyMiddleware<M>(M);

impl<M> MyMiddleware<M> {
    pub fn new(inner: M) -> Self {
        MyMiddleware(inner)
    }
}

#[derive(Error, Debug)]
pub enum MyError<M: Middleware> {
    #[error("{0}")]
    MiddlewareError(M::Error),

    // Add any middleware specific errors here
}

impl<M: Middleware> MiddlewareError for MyError<M> {
    type Inner = M::Error;

    fn from_err(src: M::Error) -> MyError<M> {
        MyError::MiddlewareError(src)
    }

    fn as_inner(&self) -> Option<&Self::Inner> {
        match self {
            MyError::MiddlewareError(e) => Some(e),
            _ => None,
        }
    }
}

#[async_trait]
impl<M> Middleware for MyMiddleware<M>
    where
        M: Middleware,
{
    type Error = MyError<M>;
    type Provider = M::Provider;
    type Inner = M;

    fn inner(&self) -> &M {
        &self.0
    }

    /// Overrides send_raw_transaction method
    async fn send_raw_transaction(&self, tx: Bytes) -> Result<PendingTransaction<'_, M::Provider>, Self::Error> {
        println!("Sending raw transaction {tx:?}");
        self.inner().send_raw_transaction(tx).await.map_err(MiddlewareError::from_err)
    }

}
