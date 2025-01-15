pub trait TradeOptions<T> {
    type Args;
    type Res;
    // type PutArgs;
    // type PutRes;
    // type CallArgs;
    // type CallRes;
    type TradeArgs;
    type TradeRes;
    type ExecuteArgs;
    type ExecuteRes;

    fn new(ledger: T, args: Self::Args) -> impl std::future::Future<Output = Self::Res> + Send;
    // fn put(&self, args: Self::PutArgs) -> impl std::future::Future<Output = Self::PutRes> + Send;
    // fn call(&self, args: Self::CallArgs) -> impl std::future::Future<Output = Self::CallRes> + Send;
    fn trade(ledger: T,args: Self::TradeArgs) -> impl std::future::Future<Output = Self::TradeRes> + Send;
    fn execute(ledger: T, args: Self::ExecuteArgs) -> impl std::future::Future<Output = Self::ExecuteRes> + Send;
    fn execute_offer(ledger: T, args: Self::ExecuteArgs) -> impl std::future::Future<Output = Self::ExecuteRes> + Send;
}
