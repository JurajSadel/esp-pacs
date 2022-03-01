#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC common configure register"]
    pub rtc_options0: crate::Reg<rtc_options0::RTC_OPTIONS0_SPEC>,
    #[doc = "0x04 - configure min sleep time"]
    pub rtc_slp_timer0: crate::Reg<rtc_slp_timer0::RTC_SLP_TIMER0_SPEC>,
    #[doc = "0x08 - configure sleep time hi"]
    pub rtc_slp_timer1: crate::Reg<rtc_slp_timer1::RTC_SLP_TIMER1_SPEC>,
    #[doc = "0x0c - update rtc main timer"]
    pub rtc_time_update: crate::Reg<rtc_time_update::RTC_TIME_UPDATE_SPEC>,
    #[doc = "0x10 - read rtc_main timer low bits"]
    pub rtc_time_low0: crate::Reg<rtc_time_low0::RTC_TIME_LOW0_SPEC>,
    #[doc = "0x14 - read rtc_main timer high bits"]
    pub rtc_time_high0: crate::Reg<rtc_time_high0::RTC_TIME_HIGH0_SPEC>,
    #[doc = "0x18 - configure chip sleep"]
    pub rtc_state0: crate::Reg<rtc_state0::RTC_STATE0_SPEC>,
    #[doc = "0x1c - rtc state wait time"]
    pub rtc_timer1: crate::Reg<rtc_timer1::RTC_TIMER1_SPEC>,
    #[doc = "0x20 - rtc monitor state delay time"]
    pub rtc_timer2: crate::Reg<rtc_timer2::RTC_TIMER2_SPEC>,
    #[doc = "0x24 - No public"]
    pub rtc_timer3: crate::Reg<rtc_timer3::RTC_TIMER3_SPEC>,
    #[doc = "0x28 - No public"]
    pub rtc_timer4: crate::Reg<rtc_timer4::RTC_TIMER4_SPEC>,
    #[doc = "0x2c - configure min sleep time"]
    pub rtc_timer5: crate::Reg<rtc_timer5::RTC_TIMER5_SPEC>,
    #[doc = "0x30 - No public"]
    pub rtc_timer6: crate::Reg<rtc_timer6::RTC_TIMER6_SPEC>,
    #[doc = "0x34 - analog configure register"]
    pub rtc_ana_conf: crate::Reg<rtc_ana_conf::RTC_ANA_CONF_SPEC>,
    #[doc = "0x38 - get reset state"]
    pub rtc_reset_state: crate::Reg<rtc_reset_state::RTC_RESET_STATE_SPEC>,
    #[doc = "0x3c - configure wakeup state"]
    pub rtc_wakeup_state: crate::Reg<rtc_wakeup_state::RTC_WAKEUP_STATE_SPEC>,
    #[doc = "0x40 - configure rtc interrupt register"]
    pub int_ena_rtc: crate::Reg<int_ena_rtc::INT_ENA_RTC_SPEC>,
    #[doc = "0x44 - rtc interrupt register"]
    pub int_raw_rtc: crate::Reg<int_raw_rtc::INT_RAW_RTC_SPEC>,
    #[doc = "0x48 - rtc interrupt register"]
    pub int_st_rtc: crate::Reg<int_st_rtc::INT_ST_RTC_SPEC>,
    #[doc = "0x4c - rtc interrupt register"]
    pub int_clr_rtc: crate::Reg<int_clr_rtc::INT_CLR_RTC_SPEC>,
    #[doc = "0x50 - Reserved register"]
    pub rtc_store0: crate::Reg<rtc_store0::RTC_STORE0_SPEC>,
    #[doc = "0x54 - Reserved register"]
    pub rtc_store1: crate::Reg<rtc_store1::RTC_STORE1_SPEC>,
    #[doc = "0x58 - Reserved register"]
    pub rtc_store2: crate::Reg<rtc_store2::RTC_STORE2_SPEC>,
    #[doc = "0x5c - Reserved register"]
    pub rtc_store3: crate::Reg<rtc_store3::RTC_STORE3_SPEC>,
    #[doc = "0x60 - Reserved register"]
    pub rtc_ext_xtl_conf: crate::Reg<rtc_ext_xtl_conf::RTC_EXT_XTL_CONF_SPEC>,
    #[doc = "0x64 - ext wakeup configure"]
    pub rtc_ext_wakeup_conf: crate::Reg<rtc_ext_wakeup_conf::RTC_EXT_WAKEUP_CONF_SPEC>,
    #[doc = "0x68 - reject sleep register"]
    pub rtc_slp_reject_conf: crate::Reg<rtc_slp_reject_conf::RTC_SLP_REJECT_CONF_SPEC>,
    #[doc = "0x6c - conigure cpu freq"]
    pub rtc_cpu_period_conf: crate::Reg<rtc_cpu_period_conf::RTC_CPU_PERIOD_CONF_SPEC>,
    #[doc = "0x70 - No public"]
    pub rtc_sdio_act_conf: crate::Reg<rtc_sdio_act_conf::RTC_SDIO_ACT_CONF_SPEC>,
    #[doc = "0x74 - configure clock register"]
    pub rtc_clk_conf: crate::Reg<rtc_clk_conf::RTC_CLK_CONF_SPEC>,
    #[doc = "0x78 - configure slow clk"]
    pub rtc_slow_clk_conf: crate::Reg<rtc_slow_clk_conf::RTC_SLOW_CLK_CONF_SPEC>,
    #[doc = "0x7c - configure flash power"]
    pub rtc_sdio_conf: crate::Reg<rtc_sdio_conf::RTC_SDIO_CONF_SPEC>,
    #[doc = "0x80 - No public"]
    pub rtc_bias_conf: crate::Reg<rtc_bias_conf::RTC_BIAS_CONF_SPEC>,
    #[doc = "0x84 - configure rtc regulator"]
    pub rtc: crate::Reg<rtc::RTC_SPEC>,
    #[doc = "0x88 - configure rtc power"]
    pub rtc_pwc: crate::Reg<rtc_pwc::RTC_PWC_SPEC>,
    #[doc = "0x8c - No public"]
    pub rtc_regulator_drv_ctrl: crate::Reg<rtc_regulator_drv_ctrl::RTC_REGULATOR_DRV_CTRL_SPEC>,
    #[doc = "0x90 - configure digital power"]
    pub dig_pwc: crate::Reg<dig_pwc::DIG_PWC_SPEC>,
    #[doc = "0x94 - congigure digital power isolation"]
    pub dig_iso: crate::Reg<dig_iso::DIG_ISO_SPEC>,
    #[doc = "0x98 - configure rtc watch dog"]
    pub rtc_wdtconfig0: crate::Reg<rtc_wdtconfig0::RTC_WDTCONFIG0_SPEC>,
    #[doc = "0x9c - stage0 hold time"]
    pub rtc_wdtconfig1: crate::Reg<rtc_wdtconfig1::RTC_WDTCONFIG1_SPEC>,
    #[doc = "0xa0 - stage1 hold time"]
    pub rtc_wdtconfig2: crate::Reg<rtc_wdtconfig2::RTC_WDTCONFIG2_SPEC>,
    #[doc = "0xa4 - stage2 hold time"]
    pub rtc_wdtconfig3: crate::Reg<rtc_wdtconfig3::RTC_WDTCONFIG3_SPEC>,
    #[doc = "0xa8 - stage3 hold time"]
    pub rtc_wdtconfig4: crate::Reg<rtc_wdtconfig4::RTC_WDTCONFIG4_SPEC>,
    #[doc = "0xac - rtc wdt feed"]
    pub rtc_wdtfeed: crate::Reg<rtc_wdtfeed::RTC_WDTFEED_SPEC>,
    #[doc = "0xb0 - configure rtc watch dog"]
    pub rtc_wdtwprotect: crate::Reg<rtc_wdtwprotect::RTC_WDTWPROTECT_SPEC>,
    #[doc = "0xb4 - congfigure super watch dog"]
    pub rtc_swd_conf: crate::Reg<rtc_swd_conf::RTC_SWD_CONF_SPEC>,
    #[doc = "0xb8 - super watch dog key"]
    pub rtc_swd_wprotect: crate::Reg<rtc_swd_wprotect::RTC_SWD_WPROTECT_SPEC>,
    #[doc = "0xbc - configure cpu stall by sw"]
    pub rtc_sw_cpu_stall: crate::Reg<rtc_sw_cpu_stall::RTC_SW_CPU_STALL_SPEC>,
    #[doc = "0xc0 - reserved register"]
    pub rtc_store4: crate::Reg<rtc_store4::RTC_STORE4_SPEC>,
    #[doc = "0xc4 - reserved register"]
    pub rtc_store5: crate::Reg<rtc_store5::RTC_STORE5_SPEC>,
    #[doc = "0xc8 - reserved register"]
    pub rtc_store6: crate::Reg<rtc_store6::RTC_STORE6_SPEC>,
    #[doc = "0xcc - reserved register"]
    pub rtc_store7: crate::Reg<rtc_store7::RTC_STORE7_SPEC>,
    #[doc = "0xd0 - reserved register"]
    pub rtc_low_power_st: crate::Reg<rtc_low_power_st::RTC_LOW_POWER_ST_SPEC>,
    #[doc = "0xd4 - No public"]
    pub rtc_diag0: crate::Reg<rtc_diag0::RTC_DIAG0_SPEC>,
    #[doc = "0xd8 - rtc pad hold configure"]
    pub rtc_pad_hold: crate::Reg<rtc_pad_hold::RTC_PAD_HOLD_SPEC>,
    #[doc = "0xdc - configure digtal pad hold"]
    pub dig_pad_hold: crate::Reg<dig_pad_hold::DIG_PAD_HOLD_SPEC>,
    #[doc = "0xe0 - configure ext1 wakeup"]
    pub rtc_ext_wakeup1: crate::Reg<rtc_ext_wakeup1::RTC_EXT_WAKEUP1_SPEC>,
    #[doc = "0xe4 - check ext wakeup1 status"]
    pub rtc_ext_wakeup1_status: crate::Reg<rtc_ext_wakeup1_status::RTC_EXT_WAKEUP1_STATUS_SPEC>,
    #[doc = "0xe8 - congfigure brownout"]
    pub rtc_brown_out: crate::Reg<rtc_brown_out::RTC_BROWN_OUT_SPEC>,
    #[doc = "0xec - RTC timer low 32 bits"]
    pub rtc_time_low1: crate::Reg<rtc_time_low1::RTC_TIME_LOW1_SPEC>,
    #[doc = "0xf0 - RTC timer high 16 bits"]
    pub rtc_time_high1: crate::Reg<rtc_time_high1::RTC_TIME_HIGH1_SPEC>,
    #[doc = "0xf4 - xtal 32k watch dog backup clock factor"]
    pub rtc_xtal32k_clk_factor: crate::Reg<rtc_xtal32k_clk_factor::RTC_XTAL32K_CLK_FACTOR_SPEC>,
    #[doc = "0xf8 - configure xtal32k"]
    pub rtc_xtal32k_conf: crate::Reg<rtc_xtal32k_conf::RTC_XTAL32K_CONF_SPEC>,
    #[doc = "0xfc - configure ulp"]
    pub rtc_ulp_cp_timer: crate::Reg<rtc_ulp_cp_timer::RTC_ULP_CP_TIMER_SPEC>,
    #[doc = "0x100 - configure ulp"]
    pub rtc_ulp_cp_ctrl: crate::Reg<rtc_ulp_cp_ctrl::RTC_ULP_CP_CTRL_SPEC>,
    #[doc = "0x104 - configure ulp-riscv"]
    pub rtc_cocpu_ctrl: crate::Reg<rtc_cocpu_ctrl::RTC_COCPU_CTRL_SPEC>,
    #[doc = "0x108 - configure touch controller"]
    pub rtc_touch_ctrl1: crate::Reg<rtc_touch_ctrl1::RTC_TOUCH_CTRL1_SPEC>,
    #[doc = "0x10c - configure touch controller"]
    pub rtc_touch_ctrl2: crate::Reg<rtc_touch_ctrl2::RTC_TOUCH_CTRL2_SPEC>,
    #[doc = "0x110 - configure touch controller"]
    pub rtc_touch_scan_ctrl: crate::Reg<rtc_touch_scan_ctrl::RTC_TOUCH_SCAN_CTRL_SPEC>,
    #[doc = "0x114 - configure touch controller"]
    pub rtc_touch_slp_thres: crate::Reg<rtc_touch_slp_thres::RTC_TOUCH_SLP_THRES_SPEC>,
    #[doc = "0x118 - configure touch controller"]
    pub rtc_touch_approach: crate::Reg<rtc_touch_approach::RTC_TOUCH_APPROACH_SPEC>,
    #[doc = "0x11c - configure touch controller"]
    pub rtc_touch_filter_ctrl: crate::Reg<rtc_touch_filter_ctrl::RTC_TOUCH_FILTER_CTRL_SPEC>,
    #[doc = "0x120 - usb configure"]
    pub rtc_usb_conf: crate::Reg<rtc_usb_conf::RTC_USB_CONF_SPEC>,
    #[doc = "0x124 - configure touch controller"]
    pub rtc_touch_timeout_ctrl: crate::Reg<rtc_touch_timeout_ctrl::RTC_TOUCH_TIMEOUT_CTRL_SPEC>,
    #[doc = "0x128 - get reject casue"]
    pub rtc_slp_reject_cause: crate::Reg<rtc_slp_reject_cause::RTC_SLP_REJECT_CAUSE_SPEC>,
    #[doc = "0x12c - rtc common configure"]
    pub rtc_option1: crate::Reg<rtc_option1::RTC_OPTION1_SPEC>,
    #[doc = "0x130 - get wakeup cause"]
    pub rtc_slp_wakeup_cause: crate::Reg<rtc_slp_wakeup_cause::RTC_SLP_WAKEUP_CAUSE_SPEC>,
    #[doc = "0x134 - configure ulp sleep time"]
    pub rtc_ulp_cp_timer_1: crate::Reg<rtc_ulp_cp_timer_1::RTC_ULP_CP_TIMER_1_SPEC>,
    #[doc = "0x138 - oneset rtc interrupt"]
    pub int_ena_rtc_w1ts: crate::Reg<int_ena_rtc_w1ts::INT_ENA_RTC_W1TS_SPEC>,
    #[doc = "0x13c - oneset clr rtc interrupt enable"]
    pub int_ena_rtc_w1tc: crate::Reg<int_ena_rtc_w1tc::INT_ENA_RTC_W1TC_SPEC>,
    #[doc = "0x140 - configure retention"]
    pub retention_ctrl: crate::Reg<retention_ctrl::RETENTION_CTRL_SPEC>,
    #[doc = "0x144 - configure power glitch"]
    pub pg_ctrl: crate::Reg<pg_ctrl::PG_CTRL_SPEC>,
    #[doc = "0x148 - No public"]
    pub rtc_fib_sel: crate::Reg<rtc_fib_sel::RTC_FIB_SEL_SPEC>,
    #[doc = "0x14c - configure touch dac"]
    pub touch_dac: crate::Reg<touch_dac::TOUCH_DAC_SPEC>,
    #[doc = "0x150 - configure touch dac"]
    pub touch_dac1: crate::Reg<touch_dac1::TOUCH_DAC1_SPEC>,
    #[doc = "0x154 - configure ulp diable"]
    pub rtc_cocpu_disable: crate::Reg<rtc_cocpu_disable::RTC_COCPU_DISABLE_SPEC>,
    _reserved86: [u8; 0xa4],
    #[doc = "0x1fc - version register"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "RTC_OPTIONS0 register accessor: an alias for `Reg<RTC_OPTIONS0_SPEC>`"]
pub type RTC_OPTIONS0 = crate::Reg<rtc_options0::RTC_OPTIONS0_SPEC>;
#[doc = "RTC common configure register"]
pub mod rtc_options0;
#[doc = "RTC_SLP_TIMER0 register accessor: an alias for `Reg<RTC_SLP_TIMER0_SPEC>`"]
pub type RTC_SLP_TIMER0 = crate::Reg<rtc_slp_timer0::RTC_SLP_TIMER0_SPEC>;
#[doc = "configure min sleep time"]
pub mod rtc_slp_timer0;
#[doc = "RTC_SLP_TIMER1 register accessor: an alias for `Reg<RTC_SLP_TIMER1_SPEC>`"]
pub type RTC_SLP_TIMER1 = crate::Reg<rtc_slp_timer1::RTC_SLP_TIMER1_SPEC>;
#[doc = "configure sleep time hi"]
pub mod rtc_slp_timer1;
#[doc = "RTC_TIME_UPDATE register accessor: an alias for `Reg<RTC_TIME_UPDATE_SPEC>`"]
pub type RTC_TIME_UPDATE = crate::Reg<rtc_time_update::RTC_TIME_UPDATE_SPEC>;
#[doc = "update rtc main timer"]
pub mod rtc_time_update;
#[doc = "RTC_TIME_LOW0 register accessor: an alias for `Reg<RTC_TIME_LOW0_SPEC>`"]
pub type RTC_TIME_LOW0 = crate::Reg<rtc_time_low0::RTC_TIME_LOW0_SPEC>;
#[doc = "read rtc_main timer low bits"]
pub mod rtc_time_low0;
#[doc = "RTC_TIME_HIGH0 register accessor: an alias for `Reg<RTC_TIME_HIGH0_SPEC>`"]
pub type RTC_TIME_HIGH0 = crate::Reg<rtc_time_high0::RTC_TIME_HIGH0_SPEC>;
#[doc = "read rtc_main timer high bits"]
pub mod rtc_time_high0;
#[doc = "RTC_STATE0 register accessor: an alias for `Reg<RTC_STATE0_SPEC>`"]
pub type RTC_STATE0 = crate::Reg<rtc_state0::RTC_STATE0_SPEC>;
#[doc = "configure chip sleep"]
pub mod rtc_state0;
#[doc = "RTC_TIMER1 register accessor: an alias for `Reg<RTC_TIMER1_SPEC>`"]
pub type RTC_TIMER1 = crate::Reg<rtc_timer1::RTC_TIMER1_SPEC>;
#[doc = "rtc state wait time"]
pub mod rtc_timer1;
#[doc = "RTC_TIMER2 register accessor: an alias for `Reg<RTC_TIMER2_SPEC>`"]
pub type RTC_TIMER2 = crate::Reg<rtc_timer2::RTC_TIMER2_SPEC>;
#[doc = "rtc monitor state delay time"]
pub mod rtc_timer2;
#[doc = "RTC_TIMER3 register accessor: an alias for `Reg<RTC_TIMER3_SPEC>`"]
pub type RTC_TIMER3 = crate::Reg<rtc_timer3::RTC_TIMER3_SPEC>;
#[doc = "No public"]
pub mod rtc_timer3;
#[doc = "RTC_TIMER4 register accessor: an alias for `Reg<RTC_TIMER4_SPEC>`"]
pub type RTC_TIMER4 = crate::Reg<rtc_timer4::RTC_TIMER4_SPEC>;
#[doc = "No public"]
pub mod rtc_timer4;
#[doc = "RTC_TIMER5 register accessor: an alias for `Reg<RTC_TIMER5_SPEC>`"]
pub type RTC_TIMER5 = crate::Reg<rtc_timer5::RTC_TIMER5_SPEC>;
#[doc = "configure min sleep time"]
pub mod rtc_timer5;
#[doc = "RTC_TIMER6 register accessor: an alias for `Reg<RTC_TIMER6_SPEC>`"]
pub type RTC_TIMER6 = crate::Reg<rtc_timer6::RTC_TIMER6_SPEC>;
#[doc = "No public"]
pub mod rtc_timer6;
#[doc = "RTC_ANA_CONF register accessor: an alias for `Reg<RTC_ANA_CONF_SPEC>`"]
pub type RTC_ANA_CONF = crate::Reg<rtc_ana_conf::RTC_ANA_CONF_SPEC>;
#[doc = "analog configure register"]
pub mod rtc_ana_conf;
#[doc = "RTC_RESET_STATE register accessor: an alias for `Reg<RTC_RESET_STATE_SPEC>`"]
pub type RTC_RESET_STATE = crate::Reg<rtc_reset_state::RTC_RESET_STATE_SPEC>;
#[doc = "get reset state"]
pub mod rtc_reset_state;
#[doc = "RTC_WAKEUP_STATE register accessor: an alias for `Reg<RTC_WAKEUP_STATE_SPEC>`"]
pub type RTC_WAKEUP_STATE = crate::Reg<rtc_wakeup_state::RTC_WAKEUP_STATE_SPEC>;
#[doc = "configure wakeup state"]
pub mod rtc_wakeup_state;
#[doc = "INT_ENA_RTC register accessor: an alias for `Reg<INT_ENA_RTC_SPEC>`"]
pub type INT_ENA_RTC = crate::Reg<int_ena_rtc::INT_ENA_RTC_SPEC>;
#[doc = "configure rtc interrupt register"]
pub mod int_ena_rtc;
#[doc = "INT_RAW_RTC register accessor: an alias for `Reg<INT_RAW_RTC_SPEC>`"]
pub type INT_RAW_RTC = crate::Reg<int_raw_rtc::INT_RAW_RTC_SPEC>;
#[doc = "rtc interrupt register"]
pub mod int_raw_rtc;
#[doc = "INT_ST_RTC register accessor: an alias for `Reg<INT_ST_RTC_SPEC>`"]
pub type INT_ST_RTC = crate::Reg<int_st_rtc::INT_ST_RTC_SPEC>;
#[doc = "rtc interrupt register"]
pub mod int_st_rtc;
#[doc = "INT_CLR_RTC register accessor: an alias for `Reg<INT_CLR_RTC_SPEC>`"]
pub type INT_CLR_RTC = crate::Reg<int_clr_rtc::INT_CLR_RTC_SPEC>;
#[doc = "rtc interrupt register"]
pub mod int_clr_rtc;
#[doc = "RTC_STORE0 register accessor: an alias for `Reg<RTC_STORE0_SPEC>`"]
pub type RTC_STORE0 = crate::Reg<rtc_store0::RTC_STORE0_SPEC>;
#[doc = "Reserved register"]
pub mod rtc_store0;
#[doc = "RTC_STORE1 register accessor: an alias for `Reg<RTC_STORE1_SPEC>`"]
pub type RTC_STORE1 = crate::Reg<rtc_store1::RTC_STORE1_SPEC>;
#[doc = "Reserved register"]
pub mod rtc_store1;
#[doc = "RTC_STORE2 register accessor: an alias for `Reg<RTC_STORE2_SPEC>`"]
pub type RTC_STORE2 = crate::Reg<rtc_store2::RTC_STORE2_SPEC>;
#[doc = "Reserved register"]
pub mod rtc_store2;
#[doc = "RTC_STORE3 register accessor: an alias for `Reg<RTC_STORE3_SPEC>`"]
pub type RTC_STORE3 = crate::Reg<rtc_store3::RTC_STORE3_SPEC>;
#[doc = "Reserved register"]
pub mod rtc_store3;
#[doc = "RTC_EXT_XTL_CONF register accessor: an alias for `Reg<RTC_EXT_XTL_CONF_SPEC>`"]
pub type RTC_EXT_XTL_CONF = crate::Reg<rtc_ext_xtl_conf::RTC_EXT_XTL_CONF_SPEC>;
#[doc = "Reserved register"]
pub mod rtc_ext_xtl_conf;
#[doc = "RTC_EXT_WAKEUP_CONF register accessor: an alias for `Reg<RTC_EXT_WAKEUP_CONF_SPEC>`"]
pub type RTC_EXT_WAKEUP_CONF = crate::Reg<rtc_ext_wakeup_conf::RTC_EXT_WAKEUP_CONF_SPEC>;
#[doc = "ext wakeup configure"]
pub mod rtc_ext_wakeup_conf;
#[doc = "RTC_SLP_REJECT_CONF register accessor: an alias for `Reg<RTC_SLP_REJECT_CONF_SPEC>`"]
pub type RTC_SLP_REJECT_CONF = crate::Reg<rtc_slp_reject_conf::RTC_SLP_REJECT_CONF_SPEC>;
#[doc = "reject sleep register"]
pub mod rtc_slp_reject_conf;
#[doc = "RTC_CPU_PERIOD_CONF register accessor: an alias for `Reg<RTC_CPU_PERIOD_CONF_SPEC>`"]
pub type RTC_CPU_PERIOD_CONF = crate::Reg<rtc_cpu_period_conf::RTC_CPU_PERIOD_CONF_SPEC>;
#[doc = "conigure cpu freq"]
pub mod rtc_cpu_period_conf;
#[doc = "RTC_SDIO_ACT_CONF register accessor: an alias for `Reg<RTC_SDIO_ACT_CONF_SPEC>`"]
pub type RTC_SDIO_ACT_CONF = crate::Reg<rtc_sdio_act_conf::RTC_SDIO_ACT_CONF_SPEC>;
#[doc = "No public"]
pub mod rtc_sdio_act_conf;
#[doc = "RTC_CLK_CONF register accessor: an alias for `Reg<RTC_CLK_CONF_SPEC>`"]
pub type RTC_CLK_CONF = crate::Reg<rtc_clk_conf::RTC_CLK_CONF_SPEC>;
#[doc = "configure clock register"]
pub mod rtc_clk_conf;
#[doc = "RTC_SLOW_CLK_CONF register accessor: an alias for `Reg<RTC_SLOW_CLK_CONF_SPEC>`"]
pub type RTC_SLOW_CLK_CONF = crate::Reg<rtc_slow_clk_conf::RTC_SLOW_CLK_CONF_SPEC>;
#[doc = "configure slow clk"]
pub mod rtc_slow_clk_conf;
#[doc = "RTC_SDIO_CONF register accessor: an alias for `Reg<RTC_SDIO_CONF_SPEC>`"]
pub type RTC_SDIO_CONF = crate::Reg<rtc_sdio_conf::RTC_SDIO_CONF_SPEC>;
#[doc = "configure flash power"]
pub mod rtc_sdio_conf;
#[doc = "RTC_BIAS_CONF register accessor: an alias for `Reg<RTC_BIAS_CONF_SPEC>`"]
pub type RTC_BIAS_CONF = crate::Reg<rtc_bias_conf::RTC_BIAS_CONF_SPEC>;
#[doc = "No public"]
pub mod rtc_bias_conf;
#[doc = "RTC register accessor: an alias for `Reg<RTC_SPEC>`"]
pub type RTC = crate::Reg<rtc::RTC_SPEC>;
#[doc = "configure rtc regulator"]
pub mod rtc;
#[doc = "RTC_PWC register accessor: an alias for `Reg<RTC_PWC_SPEC>`"]
pub type RTC_PWC = crate::Reg<rtc_pwc::RTC_PWC_SPEC>;
#[doc = "configure rtc power"]
pub mod rtc_pwc;
#[doc = "RTC_REGULATOR_DRV_CTRL register accessor: an alias for `Reg<RTC_REGULATOR_DRV_CTRL_SPEC>`"]
pub type RTC_REGULATOR_DRV_CTRL = crate::Reg<rtc_regulator_drv_ctrl::RTC_REGULATOR_DRV_CTRL_SPEC>;
#[doc = "No public"]
pub mod rtc_regulator_drv_ctrl;
#[doc = "DIG_PWC register accessor: an alias for `Reg<DIG_PWC_SPEC>`"]
pub type DIG_PWC = crate::Reg<dig_pwc::DIG_PWC_SPEC>;
#[doc = "configure digital power"]
pub mod dig_pwc;
#[doc = "DIG_ISO register accessor: an alias for `Reg<DIG_ISO_SPEC>`"]
pub type DIG_ISO = crate::Reg<dig_iso::DIG_ISO_SPEC>;
#[doc = "congigure digital power isolation"]
pub mod dig_iso;
#[doc = "RTC_WDTCONFIG0 register accessor: an alias for `Reg<RTC_WDTCONFIG0_SPEC>`"]
pub type RTC_WDTCONFIG0 = crate::Reg<rtc_wdtconfig0::RTC_WDTCONFIG0_SPEC>;
#[doc = "configure rtc watch dog"]
pub mod rtc_wdtconfig0;
#[doc = "RTC_WDTCONFIG1 register accessor: an alias for `Reg<RTC_WDTCONFIG1_SPEC>`"]
pub type RTC_WDTCONFIG1 = crate::Reg<rtc_wdtconfig1::RTC_WDTCONFIG1_SPEC>;
#[doc = "stage0 hold time"]
pub mod rtc_wdtconfig1;
#[doc = "RTC_WDTCONFIG2 register accessor: an alias for `Reg<RTC_WDTCONFIG2_SPEC>`"]
pub type RTC_WDTCONFIG2 = crate::Reg<rtc_wdtconfig2::RTC_WDTCONFIG2_SPEC>;
#[doc = "stage1 hold time"]
pub mod rtc_wdtconfig2;
#[doc = "RTC_WDTCONFIG3 register accessor: an alias for `Reg<RTC_WDTCONFIG3_SPEC>`"]
pub type RTC_WDTCONFIG3 = crate::Reg<rtc_wdtconfig3::RTC_WDTCONFIG3_SPEC>;
#[doc = "stage2 hold time"]
pub mod rtc_wdtconfig3;
#[doc = "RTC_WDTCONFIG4 register accessor: an alias for `Reg<RTC_WDTCONFIG4_SPEC>`"]
pub type RTC_WDTCONFIG4 = crate::Reg<rtc_wdtconfig4::RTC_WDTCONFIG4_SPEC>;
#[doc = "stage3 hold time"]
pub mod rtc_wdtconfig4;
#[doc = "RTC_WDTFEED register accessor: an alias for `Reg<RTC_WDTFEED_SPEC>`"]
pub type RTC_WDTFEED = crate::Reg<rtc_wdtfeed::RTC_WDTFEED_SPEC>;
#[doc = "rtc wdt feed"]
pub mod rtc_wdtfeed;
#[doc = "RTC_WDTWPROTECT register accessor: an alias for `Reg<RTC_WDTWPROTECT_SPEC>`"]
pub type RTC_WDTWPROTECT = crate::Reg<rtc_wdtwprotect::RTC_WDTWPROTECT_SPEC>;
#[doc = "configure rtc watch dog"]
pub mod rtc_wdtwprotect;
#[doc = "RTC_SWD_CONF register accessor: an alias for `Reg<RTC_SWD_CONF_SPEC>`"]
pub type RTC_SWD_CONF = crate::Reg<rtc_swd_conf::RTC_SWD_CONF_SPEC>;
#[doc = "congfigure super watch dog"]
pub mod rtc_swd_conf;
#[doc = "RTC_SWD_WPROTECT register accessor: an alias for `Reg<RTC_SWD_WPROTECT_SPEC>`"]
pub type RTC_SWD_WPROTECT = crate::Reg<rtc_swd_wprotect::RTC_SWD_WPROTECT_SPEC>;
#[doc = "super watch dog key"]
pub mod rtc_swd_wprotect;
#[doc = "RTC_SW_CPU_STALL register accessor: an alias for `Reg<RTC_SW_CPU_STALL_SPEC>`"]
pub type RTC_SW_CPU_STALL = crate::Reg<rtc_sw_cpu_stall::RTC_SW_CPU_STALL_SPEC>;
#[doc = "configure cpu stall by sw"]
pub mod rtc_sw_cpu_stall;
#[doc = "RTC_STORE4 register accessor: an alias for `Reg<RTC_STORE4_SPEC>`"]
pub type RTC_STORE4 = crate::Reg<rtc_store4::RTC_STORE4_SPEC>;
#[doc = "reserved register"]
pub mod rtc_store4;
#[doc = "RTC_STORE5 register accessor: an alias for `Reg<RTC_STORE5_SPEC>`"]
pub type RTC_STORE5 = crate::Reg<rtc_store5::RTC_STORE5_SPEC>;
#[doc = "reserved register"]
pub mod rtc_store5;
#[doc = "RTC_STORE6 register accessor: an alias for `Reg<RTC_STORE6_SPEC>`"]
pub type RTC_STORE6 = crate::Reg<rtc_store6::RTC_STORE6_SPEC>;
#[doc = "reserved register"]
pub mod rtc_store6;
#[doc = "RTC_STORE7 register accessor: an alias for `Reg<RTC_STORE7_SPEC>`"]
pub type RTC_STORE7 = crate::Reg<rtc_store7::RTC_STORE7_SPEC>;
#[doc = "reserved register"]
pub mod rtc_store7;
#[doc = "RTC_LOW_POWER_ST register accessor: an alias for `Reg<RTC_LOW_POWER_ST_SPEC>`"]
pub type RTC_LOW_POWER_ST = crate::Reg<rtc_low_power_st::RTC_LOW_POWER_ST_SPEC>;
#[doc = "reserved register"]
pub mod rtc_low_power_st;
#[doc = "RTC_DIAG0 register accessor: an alias for `Reg<RTC_DIAG0_SPEC>`"]
pub type RTC_DIAG0 = crate::Reg<rtc_diag0::RTC_DIAG0_SPEC>;
#[doc = "No public"]
pub mod rtc_diag0;
#[doc = "RTC_PAD_HOLD register accessor: an alias for `Reg<RTC_PAD_HOLD_SPEC>`"]
pub type RTC_PAD_HOLD = crate::Reg<rtc_pad_hold::RTC_PAD_HOLD_SPEC>;
#[doc = "rtc pad hold configure"]
pub mod rtc_pad_hold;
#[doc = "DIG_PAD_HOLD register accessor: an alias for `Reg<DIG_PAD_HOLD_SPEC>`"]
pub type DIG_PAD_HOLD = crate::Reg<dig_pad_hold::DIG_PAD_HOLD_SPEC>;
#[doc = "configure digtal pad hold"]
pub mod dig_pad_hold;
#[doc = "RTC_EXT_WAKEUP1 register accessor: an alias for `Reg<RTC_EXT_WAKEUP1_SPEC>`"]
pub type RTC_EXT_WAKEUP1 = crate::Reg<rtc_ext_wakeup1::RTC_EXT_WAKEUP1_SPEC>;
#[doc = "configure ext1 wakeup"]
pub mod rtc_ext_wakeup1;
#[doc = "RTC_EXT_WAKEUP1_STATUS register accessor: an alias for `Reg<RTC_EXT_WAKEUP1_STATUS_SPEC>`"]
pub type RTC_EXT_WAKEUP1_STATUS = crate::Reg<rtc_ext_wakeup1_status::RTC_EXT_WAKEUP1_STATUS_SPEC>;
#[doc = "check ext wakeup1 status"]
pub mod rtc_ext_wakeup1_status;
#[doc = "RTC_BROWN_OUT register accessor: an alias for `Reg<RTC_BROWN_OUT_SPEC>`"]
pub type RTC_BROWN_OUT = crate::Reg<rtc_brown_out::RTC_BROWN_OUT_SPEC>;
#[doc = "congfigure brownout"]
pub mod rtc_brown_out;
#[doc = "RTC_TIME_LOW1 register accessor: an alias for `Reg<RTC_TIME_LOW1_SPEC>`"]
pub type RTC_TIME_LOW1 = crate::Reg<rtc_time_low1::RTC_TIME_LOW1_SPEC>;
#[doc = "RTC timer low 32 bits"]
pub mod rtc_time_low1;
#[doc = "RTC_TIME_HIGH1 register accessor: an alias for `Reg<RTC_TIME_HIGH1_SPEC>`"]
pub type RTC_TIME_HIGH1 = crate::Reg<rtc_time_high1::RTC_TIME_HIGH1_SPEC>;
#[doc = "RTC timer high 16 bits"]
pub mod rtc_time_high1;
#[doc = "RTC_XTAL32K_CLK_FACTOR register accessor: an alias for `Reg<RTC_XTAL32K_CLK_FACTOR_SPEC>`"]
pub type RTC_XTAL32K_CLK_FACTOR = crate::Reg<rtc_xtal32k_clk_factor::RTC_XTAL32K_CLK_FACTOR_SPEC>;
#[doc = "xtal 32k watch dog backup clock factor"]
pub mod rtc_xtal32k_clk_factor;
#[doc = "RTC_XTAL32K_CONF register accessor: an alias for `Reg<RTC_XTAL32K_CONF_SPEC>`"]
pub type RTC_XTAL32K_CONF = crate::Reg<rtc_xtal32k_conf::RTC_XTAL32K_CONF_SPEC>;
#[doc = "configure xtal32k"]
pub mod rtc_xtal32k_conf;
#[doc = "RTC_ULP_CP_TIMER register accessor: an alias for `Reg<RTC_ULP_CP_TIMER_SPEC>`"]
pub type RTC_ULP_CP_TIMER = crate::Reg<rtc_ulp_cp_timer::RTC_ULP_CP_TIMER_SPEC>;
#[doc = "configure ulp"]
pub mod rtc_ulp_cp_timer;
#[doc = "RTC_ULP_CP_CTRL register accessor: an alias for `Reg<RTC_ULP_CP_CTRL_SPEC>`"]
pub type RTC_ULP_CP_CTRL = crate::Reg<rtc_ulp_cp_ctrl::RTC_ULP_CP_CTRL_SPEC>;
#[doc = "configure ulp"]
pub mod rtc_ulp_cp_ctrl;
#[doc = "RTC_COCPU_CTRL register accessor: an alias for `Reg<RTC_COCPU_CTRL_SPEC>`"]
pub type RTC_COCPU_CTRL = crate::Reg<rtc_cocpu_ctrl::RTC_COCPU_CTRL_SPEC>;
#[doc = "configure ulp-riscv"]
pub mod rtc_cocpu_ctrl;
#[doc = "RTC_TOUCH_CTRL1 register accessor: an alias for `Reg<RTC_TOUCH_CTRL1_SPEC>`"]
pub type RTC_TOUCH_CTRL1 = crate::Reg<rtc_touch_ctrl1::RTC_TOUCH_CTRL1_SPEC>;
#[doc = "configure touch controller"]
pub mod rtc_touch_ctrl1;
#[doc = "RTC_TOUCH_CTRL2 register accessor: an alias for `Reg<RTC_TOUCH_CTRL2_SPEC>`"]
pub type RTC_TOUCH_CTRL2 = crate::Reg<rtc_touch_ctrl2::RTC_TOUCH_CTRL2_SPEC>;
#[doc = "configure touch controller"]
pub mod rtc_touch_ctrl2;
#[doc = "RTC_TOUCH_SCAN_CTRL register accessor: an alias for `Reg<RTC_TOUCH_SCAN_CTRL_SPEC>`"]
pub type RTC_TOUCH_SCAN_CTRL = crate::Reg<rtc_touch_scan_ctrl::RTC_TOUCH_SCAN_CTRL_SPEC>;
#[doc = "configure touch controller"]
pub mod rtc_touch_scan_ctrl;
#[doc = "RTC_TOUCH_SLP_THRES register accessor: an alias for `Reg<RTC_TOUCH_SLP_THRES_SPEC>`"]
pub type RTC_TOUCH_SLP_THRES = crate::Reg<rtc_touch_slp_thres::RTC_TOUCH_SLP_THRES_SPEC>;
#[doc = "configure touch controller"]
pub mod rtc_touch_slp_thres;
#[doc = "RTC_TOUCH_APPROACH register accessor: an alias for `Reg<RTC_TOUCH_APPROACH_SPEC>`"]
pub type RTC_TOUCH_APPROACH = crate::Reg<rtc_touch_approach::RTC_TOUCH_APPROACH_SPEC>;
#[doc = "configure touch controller"]
pub mod rtc_touch_approach;
#[doc = "RTC_TOUCH_FILTER_CTRL register accessor: an alias for `Reg<RTC_TOUCH_FILTER_CTRL_SPEC>`"]
pub type RTC_TOUCH_FILTER_CTRL = crate::Reg<rtc_touch_filter_ctrl::RTC_TOUCH_FILTER_CTRL_SPEC>;
#[doc = "configure touch controller"]
pub mod rtc_touch_filter_ctrl;
#[doc = "RTC_USB_CONF register accessor: an alias for `Reg<RTC_USB_CONF_SPEC>`"]
pub type RTC_USB_CONF = crate::Reg<rtc_usb_conf::RTC_USB_CONF_SPEC>;
#[doc = "usb configure"]
pub mod rtc_usb_conf;
#[doc = "RTC_TOUCH_TIMEOUT_CTRL register accessor: an alias for `Reg<RTC_TOUCH_TIMEOUT_CTRL_SPEC>`"]
pub type RTC_TOUCH_TIMEOUT_CTRL = crate::Reg<rtc_touch_timeout_ctrl::RTC_TOUCH_TIMEOUT_CTRL_SPEC>;
#[doc = "configure touch controller"]
pub mod rtc_touch_timeout_ctrl;
#[doc = "RTC_SLP_REJECT_CAUSE register accessor: an alias for `Reg<RTC_SLP_REJECT_CAUSE_SPEC>`"]
pub type RTC_SLP_REJECT_CAUSE = crate::Reg<rtc_slp_reject_cause::RTC_SLP_REJECT_CAUSE_SPEC>;
#[doc = "get reject casue"]
pub mod rtc_slp_reject_cause;
#[doc = "RTC_OPTION1 register accessor: an alias for `Reg<RTC_OPTION1_SPEC>`"]
pub type RTC_OPTION1 = crate::Reg<rtc_option1::RTC_OPTION1_SPEC>;
#[doc = "rtc common configure"]
pub mod rtc_option1;
#[doc = "RTC_SLP_WAKEUP_CAUSE register accessor: an alias for `Reg<RTC_SLP_WAKEUP_CAUSE_SPEC>`"]
pub type RTC_SLP_WAKEUP_CAUSE = crate::Reg<rtc_slp_wakeup_cause::RTC_SLP_WAKEUP_CAUSE_SPEC>;
#[doc = "get wakeup cause"]
pub mod rtc_slp_wakeup_cause;
#[doc = "RTC_ULP_CP_TIMER_1 register accessor: an alias for `Reg<RTC_ULP_CP_TIMER_1_SPEC>`"]
pub type RTC_ULP_CP_TIMER_1 = crate::Reg<rtc_ulp_cp_timer_1::RTC_ULP_CP_TIMER_1_SPEC>;
#[doc = "configure ulp sleep time"]
pub mod rtc_ulp_cp_timer_1;
#[doc = "INT_ENA_RTC_W1TS register accessor: an alias for `Reg<INT_ENA_RTC_W1TS_SPEC>`"]
pub type INT_ENA_RTC_W1TS = crate::Reg<int_ena_rtc_w1ts::INT_ENA_RTC_W1TS_SPEC>;
#[doc = "oneset rtc interrupt"]
pub mod int_ena_rtc_w1ts;
#[doc = "INT_ENA_RTC_W1TC register accessor: an alias for `Reg<INT_ENA_RTC_W1TC_SPEC>`"]
pub type INT_ENA_RTC_W1TC = crate::Reg<int_ena_rtc_w1tc::INT_ENA_RTC_W1TC_SPEC>;
#[doc = "oneset clr rtc interrupt enable"]
pub mod int_ena_rtc_w1tc;
#[doc = "RETENTION_CTRL register accessor: an alias for `Reg<RETENTION_CTRL_SPEC>`"]
pub type RETENTION_CTRL = crate::Reg<retention_ctrl::RETENTION_CTRL_SPEC>;
#[doc = "configure retention"]
pub mod retention_ctrl;
#[doc = "PG_CTRL register accessor: an alias for `Reg<PG_CTRL_SPEC>`"]
pub type PG_CTRL = crate::Reg<pg_ctrl::PG_CTRL_SPEC>;
#[doc = "configure power glitch"]
pub mod pg_ctrl;
#[doc = "RTC_FIB_SEL register accessor: an alias for `Reg<RTC_FIB_SEL_SPEC>`"]
pub type RTC_FIB_SEL = crate::Reg<rtc_fib_sel::RTC_FIB_SEL_SPEC>;
#[doc = "No public"]
pub mod rtc_fib_sel;
#[doc = "TOUCH_DAC register accessor: an alias for `Reg<TOUCH_DAC_SPEC>`"]
pub type TOUCH_DAC = crate::Reg<touch_dac::TOUCH_DAC_SPEC>;
#[doc = "configure touch dac"]
pub mod touch_dac;
#[doc = "TOUCH_DAC1 register accessor: an alias for `Reg<TOUCH_DAC1_SPEC>`"]
pub type TOUCH_DAC1 = crate::Reg<touch_dac1::TOUCH_DAC1_SPEC>;
#[doc = "configure touch dac"]
pub mod touch_dac1;
#[doc = "RTC_COCPU_DISABLE register accessor: an alias for `Reg<RTC_COCPU_DISABLE_SPEC>`"]
pub type RTC_COCPU_DISABLE = crate::Reg<rtc_cocpu_disable::RTC_COCPU_DISABLE_SPEC>;
#[doc = "configure ulp diable"]
pub mod rtc_cocpu_disable;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "version register"]
pub mod date;