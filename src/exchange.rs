use std::fmt::Debug;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    model::{
        Balance, CancelAllOrdersRequest, CancelOrderRequest, Candle, GetHistoricRatesRequest,
        GetOrderHistoryRequest, GetOrderRequest, GetPriceTickerRequest, OpenLimitOrderRequest,
        OpenMarketOrderRequest, Order, OrderBookRequest, OrderBookResponse, OrderCanceled,
        Paginator, Ticker, Trade, TradeHistoryRequest,
    },
    shared::Result,
};

pub struct OpenLimits {}

impl OpenLimits {
    pub async fn instantiate<Exc: Exchange + ExchangeInstantiation>(
        parameters: Exc::Parameters,
    ) -> ExchangeWrapper<Exc> {
        ExchangeWrapper::new(Exc::new(parameters).await)
    }
}

pub struct ExchangeWrapper<Exc: Exchange + ?Sized> {
    inner: Exc,
}

impl<Exc: Exchange> ExchangeWrapper<Exc> {
    pub fn new(inner: Exc) -> Self {
        Self { inner }
    }
}
/*
impl<Exc: 'static + Exchange> Deref for ExchangeWrapper<Exc> {
    type Target =
        dyn Exchange<OrderId = Exc::OrderId, TradeId = Exc::TradeId, Pagination = Exc::Pagination>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
*/
#[async_trait]
pub trait ExchangeInstantiation {
    type Parameters;

    async fn new(parameters: Self::Parameters) -> Self;
}

pub trait ExchangeSpec: Unpin {
    type OrderId: Debug + Clone + Serialize + DeserializeOwned;
    type TradeId: Debug + Clone + Serialize + DeserializeOwned;
    type Pagination: Debug + Clone + Serialize + DeserializeOwned;
}

#[async_trait]
pub trait ExchangeMarketData: ExchangeSpec + Sized {
    async fn order_book(&self, req: &OrderBookRequest) -> Result<OrderBookResponse>;
    async fn get_price_ticker(&self, req: &GetPriceTickerRequest) -> Result<Ticker>;
    async fn get_trade_history(&self, req: &TradeHistoryRequest<Self>) -> Result<Vec<Trade<Self>>>;
    async fn get_historic_rates(&self, req: &GetHistoricRatesRequest<Self>) -> Result<Vec<Candle>>;
}

#[async_trait]
pub trait ExchangeAccount: ExchangeSpec + Sized {
    async fn limit_buy(&self, req: &OpenLimitOrderRequest) -> Result<Order<Self>>;
    async fn limit_sell(&self, req: &OpenLimitOrderRequest) -> Result<Order<Self>>;
    async fn market_buy(&self, req: &OpenMarketOrderRequest) -> Result<Order<Self>>;
    async fn market_sell(&self, req: &OpenMarketOrderRequest) -> Result<Order<Self>>;
    async fn cancel_order(&self, req: &CancelOrderRequest<Self>) -> Result<OrderCanceled<Self>>;
    async fn cancel_all_orders(
        &self,
        req: &CancelAllOrdersRequest,
    ) -> Result<Vec<OrderCanceled<Self>>>;
    async fn get_all_open_orders(&self) -> Result<Vec<Order<Self>>>;
    async fn get_order_history(
        &self,
        req: &GetOrderHistoryRequest<Self>,
    ) -> Result<Vec<Order<Self>>>;
    async fn get_account_balances(
        &self,
        paginator: Option<&Paginator<Self>>,
    ) -> Result<Vec<Balance>>;
    async fn get_order(&self, req: &GetOrderRequest<Self>) -> Result<Order<Self>>;
}

#[async_trait]
pub trait Exchange {
    async fn refresh_market_info(&self) -> Result<()>;
}
