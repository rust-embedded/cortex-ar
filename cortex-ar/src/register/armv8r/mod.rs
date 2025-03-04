//! Access registers for Armv8-R only

pub mod cntfrq;
pub mod cnthctl;
pub mod cnthp_ctl;
pub mod cnthp_cval;
pub mod cnthp_tval;
pub mod cntkctl;
pub mod cntp_ctl;
pub mod cntp_cval;
pub mod cntp_tval;
pub mod cntpct;
pub mod cntv_ctl;
pub mod cntv_cval;
pub mod cntv_tval;
pub mod cntvct;
pub mod cntvoff;
pub mod hacr;
pub mod hactlr;
pub mod hactlr2;
pub mod hadfsr;
pub mod haifsr;
pub mod hamair0;
pub mod hamair1;
pub mod hcptr;
pub mod hcr;
pub mod hcr2;
pub mod hdcr;
pub mod hdfar;
pub mod hifar;
pub mod hmair0;
pub mod hmair1;
pub mod hmpuir;
pub mod hpfar;
pub mod hprbar;
pub mod hprbar0;
pub mod hprbar1;
pub mod hprbar10;
pub mod hprbar11;
pub mod hprbar12;
pub mod hprbar13;
pub mod hprbar14;
pub mod hprbar15;
pub mod hprbar2;
pub mod hprbar3;
pub mod hprbar4;
pub mod hprbar5;
pub mod hprbar6;
pub mod hprbar7;
pub mod hprbar8;
pub mod hprbar9;
pub mod hprenr;
pub mod hprlar;
pub mod hprlar0;
pub mod hprlar1;
pub mod hprlar10;
pub mod hprlar11;
pub mod hprlar12;
pub mod hprlar13;
pub mod hprlar14;
pub mod hprlar15;
pub mod hprlar2;
pub mod hprlar3;
pub mod hprlar4;
pub mod hprlar5;
pub mod hprlar6;
pub mod hprlar7;
pub mod hprlar8;
pub mod hprlar9;
pub mod hprselr;
pub mod hsctlr;
pub mod hsr;
pub mod hstr;
pub mod htpidr;
pub mod hvbar;
pub mod prbar;
pub mod prbar0;
pub mod prbar1;
pub mod prbar10;
pub mod prbar11;
pub mod prbar12;
pub mod prbar13;
pub mod prbar14;
pub mod prbar15;
pub mod prbar2;
pub mod prbar3;
pub mod prbar4;
pub mod prbar5;
pub mod prbar6;
pub mod prbar7;
pub mod prbar8;
pub mod prbar9;
pub mod prlar;
pub mod prlar0;
pub mod prlar1;
pub mod prlar10;
pub mod prlar11;
pub mod prlar12;
pub mod prlar13;
pub mod prlar14;
pub mod prlar15;
pub mod prlar2;
pub mod prlar3;
pub mod prlar4;
pub mod prlar5;
pub mod prlar6;
pub mod prlar7;
pub mod prlar8;
pub mod prlar9;
pub mod prselr;
pub mod vbar;

pub use cntfrq::Cntfrq;
pub use cnthctl::Cnthctl;
pub use cnthp_ctl::CnthpCtl;
pub use cnthp_cval::CnthpCval;
pub use cnthp_tval::CnthpTval;
pub use cntkctl::Cntkctl;
pub use cntp_ctl::CntpCtl;
pub use cntp_cval::CntpCval;
pub use cntp_tval::CntpTval;
pub use cntpct::CntPct;
pub use cntv_ctl::CntvCtl;
pub use cntv_cval::CntvCval;
pub use cntv_tval::CntvTval;
pub use cntvct::CntVct;
pub use cntvoff::CntVoff;
pub use hacr::Hacr;
pub use hactlr::Hactlr;
pub use hactlr2::Hactlr2;
pub use hadfsr::Hadfsr;
pub use haifsr::Haifsr;
pub use hamair0::Hamair0;
pub use hamair1::Hamair1;
pub use hcptr::Hcptr;
pub use hcr::Hcr;
pub use hcr2::Hcr2;
pub use hdcr::Hdcr;
pub use hdfar::Hdfar;
pub use hifar::Hifar;
pub use hmair0::Hmair0;
pub use hmair1::Hmair1;
pub use hmpuir::Hmpuir;
pub use hpfar::Hpfar;
pub use hprbar::Hprbar;
pub use hprbar0::Hprbar0;
pub use hprbar1::Hprbar1;
pub use hprbar10::Hprbar10;
pub use hprbar11::Hprbar11;
pub use hprbar12::Hprbar12;
pub use hprbar13::Hprbar13;
pub use hprbar14::Hprbar14;
pub use hprbar15::Hprbar15;
pub use hprbar2::Hprbar2;
pub use hprbar3::Hprbar3;
pub use hprbar4::Hprbar4;
pub use hprbar5::Hprbar5;
pub use hprbar6::Hprbar6;
pub use hprbar7::Hprbar7;
pub use hprbar8::Hprbar8;
pub use hprbar9::Hprbar9;
pub use hprenr::Hprenr;
pub use hprlar::Hprlar;
pub use hprlar0::Hprlar0;
pub use hprlar1::Hprlar1;
pub use hprlar10::Hprlar10;
pub use hprlar11::Hprlar11;
pub use hprlar12::Hprlar12;
pub use hprlar13::Hprlar13;
pub use hprlar14::Hprlar14;
pub use hprlar15::Hprlar15;
pub use hprlar2::Hprlar2;
pub use hprlar3::Hprlar3;
pub use hprlar4::Hprlar4;
pub use hprlar5::Hprlar5;
pub use hprlar6::Hprlar6;
pub use hprlar7::Hprlar7;
pub use hprlar8::Hprlar8;
pub use hprlar9::Hprlar9;
pub use hprselr::Hprselr;
pub use hsctlr::Hsctlr;
pub use hsr::Hsr;
pub use hstr::Hstr;
pub use htpidr::Htpidr;
pub use hvbar::Hvbar;
pub use prbar::Prbar;
pub use prbar0::Prbar0;
pub use prbar1::Prbar1;
pub use prbar10::Prbar10;
pub use prbar11::Prbar11;
pub use prbar12::Prbar12;
pub use prbar13::Prbar13;
pub use prbar14::Prbar14;
pub use prbar15::Prbar15;
pub use prbar2::Prbar2;
pub use prbar3::Prbar3;
pub use prbar4::Prbar4;
pub use prbar5::Prbar5;
pub use prbar6::Prbar6;
pub use prbar7::Prbar7;
pub use prbar8::Prbar8;
pub use prbar9::Prbar9;
pub use prlar::Prlar;
pub use prlar0::Prlar0;
pub use prlar1::Prlar1;
pub use prlar10::Prlar10;
pub use prlar11::Prlar11;
pub use prlar12::Prlar12;
pub use prlar13::Prlar13;
pub use prlar14::Prlar14;
pub use prlar15::Prlar15;
pub use prlar2::Prlar2;
pub use prlar3::Prlar3;
pub use prlar4::Prlar4;
pub use prlar5::Prlar5;
pub use prlar6::Prlar6;
pub use prlar7::Prlar7;
pub use prlar8::Prlar8;
pub use prlar9::Prlar9;
pub use prselr::Prselr;
pub use vbar::Vbar;
