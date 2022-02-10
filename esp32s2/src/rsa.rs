#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    #[doc = "0x800 - Register to store M'"]
    pub m_prime: crate::Reg<m_prime::M_PRIME_SPEC>,
    #[doc = "0x804 - RSA length mode"]
    pub mode: crate::Reg<mode::MODE_SPEC>,
    #[doc = "0x808 - RSA clean register"]
    pub clean: crate::Reg<clean::CLEAN_SPEC>,
    #[doc = "0x80c - Modular exponentiation starting bit"]
    pub modexp_start: crate::Reg<modexp_start::MODEXP_START_SPEC>,
    #[doc = "0x810 - Modular multiplication starting bit"]
    pub modmult_start: crate::Reg<modmult_start::MODMULT_START_SPEC>,
    #[doc = "0x814 - Normal multiplication starting bit"]
    pub mult_start: crate::Reg<mult_start::MULT_START_SPEC>,
    #[doc = "0x818 - RSA idle register"]
    pub idle: crate::Reg<idle::IDLE_SPEC>,
    #[doc = "0x81c - RSA clear interrupt register"]
    pub clear_interrupt: crate::Reg<clear_interrupt::CLEAR_INTERRUPT_SPEC>,
    #[doc = "0x820 - The constant_time option"]
    pub constant_time: crate::Reg<constant_time::CONSTANT_TIME_SPEC>,
    #[doc = "0x824 - The search option"]
    pub search_enable: crate::Reg<search_enable::SEARCH_ENABLE_SPEC>,
    #[doc = "0x828 - The search position"]
    pub search_pos: crate::Reg<search_pos::SEARCH_POS_SPEC>,
    #[doc = "0x82c - RSA interrupt enable register"]
    pub interrupt_ena: crate::Reg<interrupt_ena::INTERRUPT_ENA_SPEC>,
    #[doc = "0x830 - Version control register"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "M_PRIME register accessor: an alias for `Reg<M_PRIME_SPEC>`"]
pub type M_PRIME = crate::Reg<m_prime::M_PRIME_SPEC>;
#[doc = "Register to store M'"]
pub mod m_prime;
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "RSA length mode"]
pub mod mode;
#[doc = "CLEAN register accessor: an alias for `Reg<CLEAN_SPEC>`"]
pub type CLEAN = crate::Reg<clean::CLEAN_SPEC>;
#[doc = "RSA clean register"]
pub mod clean;
#[doc = "MODEXP_START register accessor: an alias for `Reg<MODEXP_START_SPEC>`"]
pub type MODEXP_START = crate::Reg<modexp_start::MODEXP_START_SPEC>;
#[doc = "Modular exponentiation starting bit"]
pub mod modexp_start;
#[doc = "MODMULT_START register accessor: an alias for `Reg<MODMULT_START_SPEC>`"]
pub type MODMULT_START = crate::Reg<modmult_start::MODMULT_START_SPEC>;
#[doc = "Modular multiplication starting bit"]
pub mod modmult_start;
#[doc = "MULT_START register accessor: an alias for `Reg<MULT_START_SPEC>`"]
pub type MULT_START = crate::Reg<mult_start::MULT_START_SPEC>;
#[doc = "Normal multiplication starting bit"]
pub mod mult_start;
#[doc = "IDLE register accessor: an alias for `Reg<IDLE_SPEC>`"]
pub type IDLE = crate::Reg<idle::IDLE_SPEC>;
#[doc = "RSA idle register"]
pub mod idle;
#[doc = "CLEAR_INTERRUPT register accessor: an alias for `Reg<CLEAR_INTERRUPT_SPEC>`"]
pub type CLEAR_INTERRUPT = crate::Reg<clear_interrupt::CLEAR_INTERRUPT_SPEC>;
#[doc = "RSA clear interrupt register"]
pub mod clear_interrupt;
#[doc = "CONSTANT_TIME register accessor: an alias for `Reg<CONSTANT_TIME_SPEC>`"]
pub type CONSTANT_TIME = crate::Reg<constant_time::CONSTANT_TIME_SPEC>;
#[doc = "The constant_time option"]
pub mod constant_time;
#[doc = "SEARCH_ENABLE register accessor: an alias for `Reg<SEARCH_ENABLE_SPEC>`"]
pub type SEARCH_ENABLE = crate::Reg<search_enable::SEARCH_ENABLE_SPEC>;
#[doc = "The search option"]
pub mod search_enable;
#[doc = "SEARCH_POS register accessor: an alias for `Reg<SEARCH_POS_SPEC>`"]
pub type SEARCH_POS = crate::Reg<search_pos::SEARCH_POS_SPEC>;
#[doc = "The search position"]
pub mod search_pos;
#[doc = "INTERRUPT_ENA register accessor: an alias for `Reg<INTERRUPT_ENA_SPEC>`"]
pub type INTERRUPT_ENA = crate::Reg<interrupt_ena::INTERRUPT_ENA_SPEC>;
#[doc = "RSA interrupt enable register"]
pub mod interrupt_ena;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
