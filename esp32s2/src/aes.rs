#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x20 - AES key register %s"]
    pub key_: [crate::Reg<key_::KEY__SPEC>; 8],
    #[doc = "0x20..0x30 - Source data register %s"]
    pub text_in_: [crate::Reg<text_in_::TEXT_IN__SPEC>; 4],
    #[doc = "0x30..0x40 - Result data register %s"]
    pub text_out_: [crate::Reg<text_out_::TEXT_OUT__SPEC>; 4],
    #[doc = "0x40 - AES working mode configuration register"]
    pub mode: crate::Reg<mode::MODE_SPEC>,
    #[doc = "0x44 - Endian configuration register"]
    pub endian: crate::Reg<endian::ENDIAN_SPEC>,
    #[doc = "0x48 - Operation start controlling register"]
    pub trigger: crate::Reg<trigger::TRIGGER_SPEC>,
    #[doc = "0x4c - Operation status register"]
    pub state: crate::Reg<state::STATE_SPEC>,
    #[doc = "0x50..0x60 - initialization vector"]
    pub iv_: [crate::Reg<iv_::IV__SPEC>; 4],
    #[doc = "0x60..0x70 - GCM hash subkey"]
    pub h_: [crate::Reg<h_::H__SPEC>; 4],
    #[doc = "0x70..0x80 - J0"]
    pub j0_: [crate::Reg<j0_::J0__SPEC>; 4],
    #[doc = "0x80..0x90 - T0"]
    pub t0_: [crate::Reg<t0_::T0__SPEC>; 4],
    #[doc = "0x90 - DMA enable register"]
    pub dma_enable: crate::Reg<dma_enable::DMA_ENABLE_SPEC>,
    #[doc = "0x94 - Block operation type register"]
    pub block_mode: crate::Reg<block_mode::BLOCK_MODE_SPEC>,
    #[doc = "0x98 - Block number configuration register"]
    pub block_num: crate::Reg<block_num::BLOCK_NUM_SPEC>,
    #[doc = "0x9c - Standard incrementing function register"]
    pub inc_sel: crate::Reg<inc_sel::INC_SEL_SPEC>,
    #[doc = "0xa0 - AAD block number configuration register"]
    pub aad_block_num: crate::Reg<aad_block_num::AAD_BLOCK_NUM_SPEC>,
    #[doc = "0xa4 - Remainder bit number of plaintext/ciphertext"]
    pub remainder_bit_num: crate::Reg<remainder_bit_num::REMAINDER_BIT_NUM_SPEC>,
    #[doc = "0xa8 - Operation continue controlling register"]
    pub continue_op: crate::Reg<continue_op::CONTINUE_OP_SPEC>,
    #[doc = "0xac - DMA-AES interrupt clear register"]
    pub int_clr: crate::Reg<int_clr::INT_CLR_SPEC>,
    #[doc = "0xb0 - DMA-AES interrupt enable register"]
    pub int_ena: crate::Reg<int_ena::INT_ENA_SPEC>,
    #[doc = "0xb4 - Version control register"]
    pub date: crate::Reg<date::DATE_SPEC>,
    #[doc = "0xb8 - Operation exit controlling register"]
    pub dma_exit: crate::Reg<dma_exit::DMA_EXIT_SPEC>,
}
#[doc = "KEY_ register accessor: an alias for `Reg<KEY__SPEC>`"]
pub type KEY_ = crate::Reg<key_::KEY__SPEC>;
#[doc = "AES key register %s"]
pub mod key_;
#[doc = "TEXT_IN_ register accessor: an alias for `Reg<TEXT_IN__SPEC>`"]
pub type TEXT_IN_ = crate::Reg<text_in_::TEXT_IN__SPEC>;
#[doc = "Source data register %s"]
pub mod text_in_;
#[doc = "TEXT_OUT_ register accessor: an alias for `Reg<TEXT_OUT__SPEC>`"]
pub type TEXT_OUT_ = crate::Reg<text_out_::TEXT_OUT__SPEC>;
#[doc = "Result data register %s"]
pub mod text_out_;
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "AES working mode configuration register"]
pub mod mode;
#[doc = "ENDIAN register accessor: an alias for `Reg<ENDIAN_SPEC>`"]
pub type ENDIAN = crate::Reg<endian::ENDIAN_SPEC>;
#[doc = "Endian configuration register"]
pub mod endian;
#[doc = "TRIGGER register accessor: an alias for `Reg<TRIGGER_SPEC>`"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "Operation start controlling register"]
pub mod trigger;
#[doc = "STATE register accessor: an alias for `Reg<STATE_SPEC>`"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "Operation status register"]
pub mod state;
#[doc = "IV_ register accessor: an alias for `Reg<IV__SPEC>`"]
pub type IV_ = crate::Reg<iv_::IV__SPEC>;
#[doc = "initialization vector"]
pub mod iv_;
#[doc = "H_ register accessor: an alias for `Reg<H__SPEC>`"]
pub type H_ = crate::Reg<h_::H__SPEC>;
#[doc = "GCM hash subkey"]
pub mod h_;
#[doc = "J0_ register accessor: an alias for `Reg<J0__SPEC>`"]
pub type J0_ = crate::Reg<j0_::J0__SPEC>;
#[doc = "J0"]
pub mod j0_;
#[doc = "T0_ register accessor: an alias for `Reg<T0__SPEC>`"]
pub type T0_ = crate::Reg<t0_::T0__SPEC>;
#[doc = "T0"]
pub mod t0_;
#[doc = "DMA_ENABLE register accessor: an alias for `Reg<DMA_ENABLE_SPEC>`"]
pub type DMA_ENABLE = crate::Reg<dma_enable::DMA_ENABLE_SPEC>;
#[doc = "DMA enable register"]
pub mod dma_enable;
#[doc = "BLOCK_MODE register accessor: an alias for `Reg<BLOCK_MODE_SPEC>`"]
pub type BLOCK_MODE = crate::Reg<block_mode::BLOCK_MODE_SPEC>;
#[doc = "Block operation type register"]
pub mod block_mode;
#[doc = "BLOCK_NUM register accessor: an alias for `Reg<BLOCK_NUM_SPEC>`"]
pub type BLOCK_NUM = crate::Reg<block_num::BLOCK_NUM_SPEC>;
#[doc = "Block number configuration register"]
pub mod block_num;
#[doc = "INC_SEL register accessor: an alias for `Reg<INC_SEL_SPEC>`"]
pub type INC_SEL = crate::Reg<inc_sel::INC_SEL_SPEC>;
#[doc = "Standard incrementing function register"]
pub mod inc_sel;
#[doc = "AAD_BLOCK_NUM register accessor: an alias for `Reg<AAD_BLOCK_NUM_SPEC>`"]
pub type AAD_BLOCK_NUM = crate::Reg<aad_block_num::AAD_BLOCK_NUM_SPEC>;
#[doc = "AAD block number configuration register"]
pub mod aad_block_num;
#[doc = "REMAINDER_BIT_NUM register accessor: an alias for `Reg<REMAINDER_BIT_NUM_SPEC>`"]
pub type REMAINDER_BIT_NUM = crate::Reg<remainder_bit_num::REMAINDER_BIT_NUM_SPEC>;
#[doc = "Remainder bit number of plaintext/ciphertext"]
pub mod remainder_bit_num;
#[doc = "CONTINUE_OP register accessor: an alias for `Reg<CONTINUE_OP_SPEC>`"]
pub type CONTINUE_OP = crate::Reg<continue_op::CONTINUE_OP_SPEC>;
#[doc = "Operation continue controlling register"]
pub mod continue_op;
#[doc = "INT_CLR register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "DMA-AES interrupt clear register"]
pub mod int_clr;
#[doc = "INT_ENA register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "DMA-AES interrupt enable register"]
pub mod int_ena;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
#[doc = "DMA_EXIT register accessor: an alias for `Reg<DMA_EXIT_SPEC>`"]
pub type DMA_EXIT = crate::Reg<dma_exit::DMA_EXIT_SPEC>;
#[doc = "Operation exit controlling register"]
pub mod dma_exit;