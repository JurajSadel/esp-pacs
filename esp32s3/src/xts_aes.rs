#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x40 - Plaintext register %s"]
    pub plain_: [crate::Reg<plain_::PLAIN__SPEC>; 16],
    #[doc = "0x40 - XTS-AES line-size register"]
    pub linesize: crate::Reg<linesize::LINESIZE_SPEC>,
    #[doc = "0x44 - XTS-AES destination register"]
    pub destination: crate::Reg<destination::DESTINATION_SPEC>,
    #[doc = "0x48 - physical address"]
    pub physical_address: crate::Reg<physical_address::PHYSICAL_ADDRESS_SPEC>,
    #[doc = "0x4c - XTS-AES trigger register"]
    pub trigger: crate::Reg<trigger::TRIGGER_SPEC>,
    #[doc = "0x50 - XTS-AES release control register"]
    pub release: crate::Reg<release::RELEASE_SPEC>,
    #[doc = "0x54 - XTS-AES destroy control register"]
    pub destroy: crate::Reg<destroy::DESTROY_SPEC>,
    #[doc = "0x58 - XTS-AES status register"]
    pub state: crate::Reg<state::STATE_SPEC>,
    #[doc = "0x5c - XTS-AES version control register"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "PLAIN_ register accessor: an alias for `Reg<PLAIN__SPEC>`"]
pub type PLAIN_ = crate::Reg<plain_::PLAIN__SPEC>;
#[doc = "Plaintext register %s"]
pub mod plain_;
#[doc = "LINESIZE register accessor: an alias for `Reg<LINESIZE_SPEC>`"]
pub type LINESIZE = crate::Reg<linesize::LINESIZE_SPEC>;
#[doc = "XTS-AES line-size register"]
pub mod linesize;
#[doc = "DESTINATION register accessor: an alias for `Reg<DESTINATION_SPEC>`"]
pub type DESTINATION = crate::Reg<destination::DESTINATION_SPEC>;
#[doc = "XTS-AES destination register"]
pub mod destination;
#[doc = "PHYSICAL_ADDRESS register accessor: an alias for `Reg<PHYSICAL_ADDRESS_SPEC>`"]
pub type PHYSICAL_ADDRESS = crate::Reg<physical_address::PHYSICAL_ADDRESS_SPEC>;
#[doc = "physical address"]
pub mod physical_address;
#[doc = "TRIGGER register accessor: an alias for `Reg<TRIGGER_SPEC>`"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "XTS-AES trigger register"]
pub mod trigger;
#[doc = "RELEASE register accessor: an alias for `Reg<RELEASE_SPEC>`"]
pub type RELEASE = crate::Reg<release::RELEASE_SPEC>;
#[doc = "XTS-AES release control register"]
pub mod release;
#[doc = "DESTROY register accessor: an alias for `Reg<DESTROY_SPEC>`"]
pub type DESTROY = crate::Reg<destroy::DESTROY_SPEC>;
#[doc = "XTS-AES destroy control register"]
pub mod destroy;
#[doc = "STATE register accessor: an alias for `Reg<STATE_SPEC>`"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "XTS-AES status register"]
pub mod state;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "XTS-AES version control register"]
pub mod date;
