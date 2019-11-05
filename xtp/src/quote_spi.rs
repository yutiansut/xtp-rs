use libc::{c_int, c_void};
use std::slice::from_raw_parts;
use xtp_sys::{XTPMD, XTPRI, XTPST};

pub trait QuoteSpi {
    fn on_disconnected(&self, _reason: i32) {}
    fn on_error(&self, _error_info: &XTPRI) {}
    fn on_sub_market_data(&self, _ticker: &XTPST, _error_info: &XTPRI, _is_last: bool) {}
    fn on_unsub_market_data(&self, _ticker: &XTPST, _error_info: &XTPRI, _is_last: bool) {}
    fn on_depth_market_data(
        &self,
        _market_data: &XTPMD,
        _bid1_qty: &[i64],
        _max_bid1_count: i32,
        _ask1_qty: &[i64],
        _max_ask1_count: i32,
    ) {
    }
    fn on_sub_order_book(&self, _ticker: &XTPST, _error_info: &XTPRI, _is_last: bool) {}
}

#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnDisconnected(spi: *mut c_void, reason: c_int) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    spi.on_disconnected(reason as i32);
}

#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnError(spi: *mut c_void, response_info: *const XTPRI) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let response_info = unsafe { &*response_info };
    spi.on_error(response_info);
}

#[no_mangle]
pub extern "C" fn QuoteSpiStub_Rust_OnSubMarketData(
    spi: *mut c_void,
    ticker: *const XTPST,
    error_info: *const XTPRI,
    is_last: bool,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let ticker = unsafe { &*ticker };
    let error_info = unsafe { &*error_info };

    spi.on_sub_market_data(ticker, error_info, is_last);
}

#[no_mangle]
extern "C" fn QuoteSpiStub_Rust_OnUnSubMarketData(
    spi: *mut c_void,
    ticker: *const XTPST,
    error_info: *const XTPRI,
    is_last: bool,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };
    let ticker = unsafe { &*ticker };
    let error_info = unsafe { &*error_info };
    spi.on_unsub_market_data(ticker, error_info, is_last);
}

#[no_mangle]
extern "C" fn QuoteSpiStub_Rust_OnDepthMarketData(
    spi: *mut c_void,
    market_data: *const XTPMD,
    bid1_qty: *const i64,
    bid1_count: i32,
    max_bid1_count: i32,
    ask1_qty: *const i64,
    ask1_count: i32,
    max_ask1_count: i32,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };

    let bid1_qty = unsafe { from_raw_parts(bid1_qty, bid1_count as usize) };
    let ask1_qty = unsafe { from_raw_parts(ask1_qty, ask1_count as usize) };

    spi.on_depth_market_data(
        unsafe { &*market_data },
        bid1_qty,
        max_bid1_count,
        ask1_qty,
        max_ask1_count,
    );
}

#[no_mangle]
extern "C" fn QuoteSpiStub_Rust_OnSubOrderBook(
    spi: *mut c_void,
    ticker: *const XTPST,
    error_info: *const XTPRI,
    is_last: bool,
) {
    let spi = unsafe { unwrap_quote_spi(spi) };

    spi.on_sub_order_book(unsafe { &*ticker }, unsafe { &*error_info }, is_last);
}
// extern "C" fn QuoteSpiStub_Rust_OnUnSubOrderBook(spi: *mut c_void, ticker: *const XTPST, error_info: *const XTPRI, bool is_last);
// extern "C" fn QuoteSpiStub_Rust_OnOrderBook(spi: *mut c_void, order_book: *const XTPOB);
// extern "C" fn QuoteSpiStub_Rust_OnSubTickByTick(spi: *mut c_void, ticker: *const XTPST, error_info: *const XTPRI, bool is_last);
// extern "C" fn QuoteSpiStub_Rust_OnUnSubTickByTick(spi: *mut c_void, ticker: *const XTPST, error_info: *const XTPRI, bool is_last);
// extern "C" fn QuoteSpiStub_Rust_OnTickByTick(spi: *mut c_void, tbt_data: *const XTPTBT);
// extern "C" fn QuoteSpiStub_Rust_OnSubscribeAllMarketData(spi: *mut c_void, exchange_id: XTP_EXCHANGE_TYPE, error_info: *const XTPRI);
// extern "C" fn QuoteSpiStub_Rust_OnUnSubscribeAllMarketData(spi: *mut c_void, exchange_id: XTP_EXCHANGE_TYPE, error_info: *const XTPRI);
// extern "C" fn QuoteSpiStub_Rust_OnSubscribeAllOrderBook(spi: *mut c_void, exchange_id: XTP_EXCHANGE_TYPE, error_info: *const XTPRI);
// extern "C" fn QuoteSpiStub_Rust_OnUnSubscribeAllOrderBook(spi: *mut c_void, exchange_id: XTP_EXCHANGE_TYPE, error_info: *const XTPRI);
// extern "C" fn QuoteSpiStub_Rust_OnSubscribeAllTickByTick(spi: *mut c_void, exchange_id: XTP_EXCHANGE_TYPE, error_info: *const XTPRI);
// extern "C" fn QuoteSpiStub_Rust_OnUnSubscribeAllTickByTick(spi: *mut c_void, exchange_id: XTP_EXCHANGE_TYPE, error_info: *const XTPRI);
// extern "C" fn QuoteSpiStub_Rust_OnQueryAllTickers(spi: *mut c_void, ticker_info: *const XTPQSI, error_info: *const XTPRI, bool is_last);
// extern "C" fn QuoteSpiStub_Rust_OnQueryTickersPriceInfo(spi: *mut c_void, ticker_info: *const XTPTPI, error_info: *const XTPRI, bool is_last);
// extern "C" fn QuoteSpiStub_Rust_OnSubscribeAllOptionMarketData(spi: *mut c_void, exchange_id: XTP_EXCHANGE_TYPE, error_info: *const XTPRI);
// extern "C" fn QuoteSpiStub_Rust_OnUnSubscribeAllOptionMarketData(spi: *mut c_void, exchange_id: XTP_EXCHANGE_TYPE, error_info: *const XTPRI);
// extern "C" fn QuoteSpiStub_Rust_OnSubscribeAllOptionOrderBook(spi: *mut c_void, exchange_id: XTP_EXCHANGE_TYPE, error_info: *const XTPRI);
// extern "C" fn QuoteSpiStub_Rust_OnUnSubscribeAllOptionOrderBook(spi: *mut c_void, exchange_id: XTP_EXCHANGE_TYPE, error_info: *const XTPRI);
// extern "C" fn QuoteSpiStub_Rust_OnSubscribeAllOptionTickByTick(spi: *mut c_void, exchange_id: XTP_EXCHANGE_TYPE, error_info: *const XTPRI);
// extern "C" fn QuoteSpiStub_Rust_OnUnSubscribeAllOptionTickByTick(spi: *mut c_void, exchange_id: XTP_EXCHANGE_TYPE, error_info: *const XTPRI);
#[no_mangle]
extern "C" fn QuoteSpiStub_Rust_Destructor(spi: *mut c_void) {
    let _: Box<dyn QuoteSpi> = Box::from_raw(*spi as *mut dyn QuoteSpi);
}

unsafe fn unwrap_quote_spi<'a>(spi: *mut c_void) -> &'a mut dyn QuoteSpi {
    &mut **(spi as *mut *mut dyn QuoteSpi)
}