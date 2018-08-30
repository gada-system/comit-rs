use common_types::{ledger::Ledger, TradingSymbol};
use event_store::Event;
use exchange_api_client::OfferResponseBody;
use secret::Secret;
use std::marker::PhantomData;
use swaps::TradeId;

// State after exchange has made an offer
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OfferCreated<Buy, Sell>
where
    Buy: Ledger,
    Sell: Ledger,
{
    pub uid: TradeId,
    pub symbol: TradingSymbol,
    pub rate: f64,
    pub buy_amount: Buy::Quantity,
    pub sell_amount: Sell::Quantity,
}

impl<Buy: Ledger, Sell: Ledger> From<OfferResponseBody<Buy, Sell>> for OfferCreated<Buy, Sell> {
    fn from(offer: OfferResponseBody<Buy, Sell>) -> Self {
        OfferCreated {
            uid: offer.uid,
            symbol: offer.symbol,
            rate: offer.rate,
            buy_amount: offer.buy_amount,
            sell_amount: offer.sell_amount,
        }
    }
}

impl<Buy: Ledger, Sell: Ledger> Event for OfferCreated<Buy, Sell> {
    type Prev = ();
}

// State after client accepts trade offer
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrderCreated<Buy, Sell>
where
    Buy: Ledger,
    Sell: Ledger,
{
    pub uid: TradeId,
    pub client_success_address: Buy::Address,
    pub client_refund_address: Sell::Address,
    pub secret: Secret,
    pub long_relative_timelock: Sell::LockDuration,
}

impl<Buy: Ledger, Sell: Ledger> Event for OrderCreated<Buy, Sell> {
    type Prev = OfferCreated<Buy, Sell>;
}

#[derive(Clone, Debug)]
pub struct OrderTaken<Buy, Sell>
where
    Buy: Ledger,
    Sell: Ledger,
{
    pub uid: TradeId,
    pub exchange_refund_address: Buy::Address,
    pub exchange_success_address: Sell::Address,
    pub exchange_contract_time_lock: Buy::LockDuration,
}

impl<Buy: Ledger, Sell: Ledger> Event for OrderTaken<Buy, Sell> {
    type Prev = OrderCreated<Buy, Sell>;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ContractDeployed<Buy, Sell>
where
    Buy: Ledger,
    Sell: Ledger,
{
    pub uid: TradeId,
    pub address: Buy::Address,
    phantom: PhantomData<Sell>,
}

impl<Buy: Ledger, Sell: Ledger> ContractDeployed<Buy, Sell> {
    pub fn new(uid: TradeId, address: Buy::Address) -> ContractDeployed<Buy, Sell> {
        ContractDeployed {
            uid,
            address,
            phantom: PhantomData,
        }
    }
}

impl<Buy: Ledger, Sell: Ledger> Event for ContractDeployed<Buy, Sell> {
    type Prev = OrderTaken<Buy, Sell>;
}
