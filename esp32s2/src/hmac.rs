#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    #[doc = "0x40 - HMAC start control register"]
    pub set_start: crate::Reg<set_start::SET_START_SPEC>,
    #[doc = "0x44 - HMAC parameter configuration register"]
    pub set_para_purpose: crate::Reg<set_para_purpose::SET_PARA_PURPOSE_SPEC>,
    #[doc = "0x48 - HMAC key configuration register"]
    pub set_para_key: crate::Reg<set_para_key::SET_PARA_KEY_SPEC>,
    #[doc = "0x4c - HMAC configuration completion register"]
    pub set_para_finish: crate::Reg<set_para_finish::SET_PARA_FINISH_SPEC>,
    #[doc = "0x50 - HMAC one message control register"]
    pub set_message_one: crate::Reg<set_message_one::SET_MESSAGE_ONE_SPEC>,
    #[doc = "0x54 - HMAC message continue register"]
    pub set_message_ing: crate::Reg<set_message_ing::SET_MESSAGE_ING_SPEC>,
    #[doc = "0x58 - HMAC message end register"]
    pub set_message_end: crate::Reg<set_message_end::SET_MESSAGE_END_SPEC>,
    #[doc = "0x5c - HMAC read result completion register"]
    pub set_result_finish: crate::Reg<set_result_finish::SET_RESULT_FINISH_SPEC>,
    #[doc = "0x60 - Invalidate JTAG result register"]
    pub set_invalidate_jtag: crate::Reg<set_invalidate_jtag::SET_INVALIDATE_JTAG_SPEC>,
    #[doc = "0x64 - Invalidate digital signature result register"]
    pub set_invalidate_ds: crate::Reg<set_invalidate_ds::SET_INVALIDATE_DS_SPEC>,
    #[doc = "0x68 - The matching result between key and purpose user configured"]
    pub query_error: crate::Reg<query_error::QUERY_ERROR_SPEC>,
    #[doc = "0x6c - The busy state of HMAC module"]
    pub query_busy: crate::Reg<query_busy::QUERY_BUSY_SPEC>,
    _reserved12: [u8; 0x10],
    #[doc = "0x80..0xc0 - Message register %s"]
    pub wr_message_: [crate::Reg<wr_message_::WR_MESSAGE__SPEC>; 16],
    #[doc = "0xc0..0xe0 - Hash result register %s"]
    pub rd_result_: [crate::Reg<rd_result_::RD_RESULT__SPEC>; 8],
    _reserved14: [u8; 0x10],
    #[doc = "0xf0 - Software padding register"]
    pub set_message_pad: crate::Reg<set_message_pad::SET_MESSAGE_PAD_SPEC>,
    #[doc = "0xf4 - One block message register."]
    pub one_block: crate::Reg<one_block::ONE_BLOCK_SPEC>,
    #[doc = "0xf8 - Version control register"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "SET_START register accessor: an alias for `Reg<SET_START_SPEC>`"]
pub type SET_START = crate::Reg<set_start::SET_START_SPEC>;
#[doc = "HMAC start control register"]
pub mod set_start;
#[doc = "SET_PARA_PURPOSE register accessor: an alias for `Reg<SET_PARA_PURPOSE_SPEC>`"]
pub type SET_PARA_PURPOSE = crate::Reg<set_para_purpose::SET_PARA_PURPOSE_SPEC>;
#[doc = "HMAC parameter configuration register"]
pub mod set_para_purpose;
#[doc = "SET_PARA_KEY register accessor: an alias for `Reg<SET_PARA_KEY_SPEC>`"]
pub type SET_PARA_KEY = crate::Reg<set_para_key::SET_PARA_KEY_SPEC>;
#[doc = "HMAC key configuration register"]
pub mod set_para_key;
#[doc = "SET_PARA_FINISH register accessor: an alias for `Reg<SET_PARA_FINISH_SPEC>`"]
pub type SET_PARA_FINISH = crate::Reg<set_para_finish::SET_PARA_FINISH_SPEC>;
#[doc = "HMAC configuration completion register"]
pub mod set_para_finish;
#[doc = "SET_MESSAGE_ONE register accessor: an alias for `Reg<SET_MESSAGE_ONE_SPEC>`"]
pub type SET_MESSAGE_ONE = crate::Reg<set_message_one::SET_MESSAGE_ONE_SPEC>;
#[doc = "HMAC one message control register"]
pub mod set_message_one;
#[doc = "SET_MESSAGE_ING register accessor: an alias for `Reg<SET_MESSAGE_ING_SPEC>`"]
pub type SET_MESSAGE_ING = crate::Reg<set_message_ing::SET_MESSAGE_ING_SPEC>;
#[doc = "HMAC message continue register"]
pub mod set_message_ing;
#[doc = "SET_MESSAGE_END register accessor: an alias for `Reg<SET_MESSAGE_END_SPEC>`"]
pub type SET_MESSAGE_END = crate::Reg<set_message_end::SET_MESSAGE_END_SPEC>;
#[doc = "HMAC message end register"]
pub mod set_message_end;
#[doc = "SET_RESULT_FINISH register accessor: an alias for `Reg<SET_RESULT_FINISH_SPEC>`"]
pub type SET_RESULT_FINISH = crate::Reg<set_result_finish::SET_RESULT_FINISH_SPEC>;
#[doc = "HMAC read result completion register"]
pub mod set_result_finish;
#[doc = "SET_INVALIDATE_JTAG register accessor: an alias for `Reg<SET_INVALIDATE_JTAG_SPEC>`"]
pub type SET_INVALIDATE_JTAG = crate::Reg<set_invalidate_jtag::SET_INVALIDATE_JTAG_SPEC>;
#[doc = "Invalidate JTAG result register"]
pub mod set_invalidate_jtag;
#[doc = "SET_INVALIDATE_DS register accessor: an alias for `Reg<SET_INVALIDATE_DS_SPEC>`"]
pub type SET_INVALIDATE_DS = crate::Reg<set_invalidate_ds::SET_INVALIDATE_DS_SPEC>;
#[doc = "Invalidate digital signature result register"]
pub mod set_invalidate_ds;
#[doc = "QUERY_ERROR register accessor: an alias for `Reg<QUERY_ERROR_SPEC>`"]
pub type QUERY_ERROR = crate::Reg<query_error::QUERY_ERROR_SPEC>;
#[doc = "The matching result between key and purpose user configured"]
pub mod query_error;
#[doc = "QUERY_BUSY register accessor: an alias for `Reg<QUERY_BUSY_SPEC>`"]
pub type QUERY_BUSY = crate::Reg<query_busy::QUERY_BUSY_SPEC>;
#[doc = "The busy state of HMAC module"]
pub mod query_busy;
#[doc = "WR_MESSAGE_ register accessor: an alias for `Reg<WR_MESSAGE__SPEC>`"]
pub type WR_MESSAGE_ = crate::Reg<wr_message_::WR_MESSAGE__SPEC>;
#[doc = "Message register %s"]
pub mod wr_message_;
#[doc = "RD_RESULT_ register accessor: an alias for `Reg<RD_RESULT__SPEC>`"]
pub type RD_RESULT_ = crate::Reg<rd_result_::RD_RESULT__SPEC>;
#[doc = "Hash result register %s"]
pub mod rd_result_;
#[doc = "SET_MESSAGE_PAD register accessor: an alias for `Reg<SET_MESSAGE_PAD_SPEC>`"]
pub type SET_MESSAGE_PAD = crate::Reg<set_message_pad::SET_MESSAGE_PAD_SPEC>;
#[doc = "Software padding register"]
pub mod set_message_pad;
#[doc = "ONE_BLOCK register accessor: an alias for `Reg<ONE_BLOCK_SPEC>`"]
pub type ONE_BLOCK = crate::Reg<one_block::ONE_BLOCK_SPEC>;
#[doc = "One block message register."]
pub mod one_block;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;