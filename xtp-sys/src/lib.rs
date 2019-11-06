#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// We are not generating these (enum) types through bindgen since their variants are defined in macro
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTP_SIDE_TYPE {
    XTP_SIDE_BUY = XTP_SIDE_BUY,
    XTP_SIDE_SELL = XTP_SIDE_SELL,
    XTP_SIDE_PURCHASE = XTP_SIDE_PURCHASE,
    XTP_SIDE_REDEMPTION = XTP_SIDE_REDEMPTION,
    XTP_SIDE_SPLIT = XTP_SIDE_SPLIT,
    XTP_SIDE_MERGE = XTP_SIDE_MERGE,
    XTP_SIDE_COVER = XTP_SIDE_COVER,
    XTP_SIDE_FREEZE = XTP_SIDE_FREEZE,
    XTP_SIDE_MARGIN_TRADE = XTP_SIDE_MARGIN_TRADE,
    XTP_SIDE_SHORT_SELL = XTP_SIDE_SHORT_SELL,
    XTP_SIDE_REPAY_MARGIN = XTP_SIDE_REPAY_MARGIN,
    XTP_SIDE_REPAY_STOCK = XTP_SIDE_REPAY_STOCK,
    XTP_SIDE_STOCK_REPAY_STOCK = XTP_SIDE_STOCK_REPAY_STOCK,
    XTP_SIDE_SURSTK_TRANS = XTP_SIDE_SURSTK_TRANS,
    XTP_SIDE_GRTSTK_TRANSIN = XTP_SIDE_GRTSTK_TRANSIN,
    XTP_SIDE_GRTSTK_TRANSOUT = XTP_SIDE_GRTSTK_TRANSOUT,
    XTP_SIDE_UNKNOWN = XTP_SIDE_UNKNOWN,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum XTP_POSITION_EFFECT_TYPE {
    XTP_POSITION_EFFECT_INIT = XTP_POSITION_EFFECT_INIT,
    XTP_POSITION_EFFECT_OPEN = XTP_POSITION_EFFECT_OPEN,
    XTP_POSITION_EFFECT_CLOSE = XTP_POSITION_EFFECT_CLOSE,
    XTP_POSITION_EFFECT_FORCECLOSE = XTP_POSITION_EFFECT_FORCECLOSE,
    XTP_POSITION_EFFECT_CLOSETODAY = XTP_POSITION_EFFECT_CLOSETODAY,
    XTP_POSITION_EFFECT_CLOSEYESTERDAY = XTP_POSITION_EFFECT_CLOSEYESTERDAY,
    XTP_POSITION_EFFECT_FORCEOFF = XTP_POSITION_EFFECT_FORCEOFF,
    XTP_POSITION_EFFECT_LOCALFORCECLOSE = XTP_POSITION_EFFECT_LOCALFORCECLOSE,
    XTP_POSITION_EFFECT_CREDIT_FORCE_COVER = XTP_POSITION_EFFECT_CREDIT_FORCE_COVER,
    XTP_POSITION_EFFECT_CREDIT_FORCE_CLEAR = XTP_POSITION_EFFECT_CREDIT_FORCE_CLEAR,
    XTP_POSITION_EFFECT_CREDIT_FORCE_DEBT = XTP_POSITION_EFFECT_CREDIT_FORCE_DEBT,
    XTP_POSITION_EFFECT_CREDIT_FORCE_UNCOND = XTP_POSITION_EFFECT_CREDIT_FORCE_UNCOND,
    XTP_POSITION_EFFECT_UNKNOWN = XTP_POSITION_EFFECT_UNKNOWN,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TXTPTradeTypeType {
    XTP_TRDT_COMMON = XTP_TRDT_COMMON,
    XTP_TRDT_CASH = XTP_TRDT_CASH,
    XTP_TRDT_PRIMARY = XTP_TRDT_PRIMARY,
    XTP_TRDT_CROSS_MKT_CASH = XTP_TRDT_CROSS_MKT_CASH,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TXTPOrderTypeType {
    XTP_ORDT_Normal = XTP_ORDT_Normal,
    XTP_ORDT_DeriveFromQuote = XTP_ORDT_DeriveFromQuote,
    XTP_ORDT_DeriveFromCombination = XTP_ORDT_DeriveFromCombination,
    XTP_ORDT_Combination = XTP_ORDT_Combination,
    XTP_ORDT_ConditionalOrder = XTP_ORDT_ConditionalOrder,
    XTP_ORDT_Swap = XTP_ORDT_Swap,
}

impl Drop for QuoteSpiStub {
    fn drop(&mut self) {
        unsafe { self.destruct() }
    }
}

impl Drop for TraderSpiStub {
    fn drop(&mut self) {
        unsafe { self.destruct() }
    }
}
