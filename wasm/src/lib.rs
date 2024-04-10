// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           19
// Async Callback (empty):               1
// Total number of exported functions:  21

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    jex_sc_raffle
    (
        init => init
        prepareRaffle => prepare_raffle
        configureTicketPrice => configure_ticket_price
        startRaffle => start_raffle
        pickWinners => pick_winners
        clearEntries => clear_entries
        endRaffle => end_raffle
        buyTickets => buy_tickets
        getRaffleStatus => get_raffle_status
        getEntries => get_entries
        getBurnRatePercent => burn_rate_percent
        getFeesRatePercent => fees_rate_percent
        getFeesAddress => fees_address
        getPrizePoolPercent => prize_pool_rate_percent
        getRaffleName => raffle_name
        getState => state
        getTicketPrice => ticket_price
        getTicketSaleEndTimestamp => ticket_sale_end_timestamp
        getTicketTokens => ticket_tokens
        getWinners => winners
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
