pub use super::CvitekPhyTraits;
pub use super::CvitekPhyDevice;
use driver_common::BaseDriverOps;
use core::marker::PhantomData;
use std::fs::read_link;
use crate::PhyDriverOps;
mod consts;
use consts::*;
use core::ptr::{write_volatile,read_volatile};
pub struct CvitekPhy<A>
where
    A: CvitekPhyTraits,
{
    device: CvitekPhyDevice<A>,
    phantom: PhantomData<A>,
}

unsafe impl<A: CvitekPhyTraits> Sync for CvitekPhy<A> {}
unsafe impl<A: CvitekPhyTraits> Send for CvitekPhy<A> {}


impl<A> CvitekPhy<A>
where
    A: CvitekPhyTraits,
{
    pub fn init(traits_impl: A) -> Self {
        let device = CvitekPhyDevice::new();
        CvitekPhy { 
            device: device,
            phantom: PhantomData
        }
    }
}


impl <A:CvitekPhyTraits> BaseDriverOps for CvitekPhy<A> {
    fn device_name(&self) -> &str {
        "cvitek_phy"
    }

    fn device_type(&self) -> driver_common::DeviceType {
        driver_common::DeviceType::Phy
    }
}

impl <A:CvitekPhyTraits> PhyDriverOps for CvitekPhy<A> {
    fn configure(&self) {
        let mut val_base:u32=0;
        let mut val:u32=0;
        let mut tmp_val:u32=0;
        info!("CvitekPhy configure");
        unsafe{ write_volatile(0x03009804 as *mut u32, 0x0001) };
        unsafe{ write_volatile(0x0300907c as *mut u32, 0x0500) };
        unsafe{ write_volatile(0x03009040 as *mut u32, 0x0c00) };
        unsafe{ write_volatile(0x03009040 as *mut u32, 0x0c7e) };
        unsafe{ write_volatile(0x03009800 as *mut u32, 0x0906) };
        unsafe{ write_volatile(0x0300907c as *mut u32, 0x0500) };
        val_base= unsafe { read_volatile( EPHY_EFUSE_VALID_BIT_BASE as *mut u32 ) };
        val = val_base & EPHY_EFUSE_TXITUNE_FLAG;
        if val == EPHY_EFUSE_TXITUNE_FLAG
        {
            tmp_val=unsafe{read_volatile(0x03051024 as *mut u32)};
            val= ( (tmp_val >> 24) & 0xFF ) | ( ((tmp_val >> 16)& 0xFF) << 8 );
            clrsetbits(0x03009064, 0xFFFF, val);
        }
        else {
            unsafe{write_volatile(0x03009064 as *mut u32, 0x5a5a);}
        }
        val_base= unsafe { read_volatile( EPHY_EFUSE_VALID_BIT_BASE as *mut u32 ) };
        val = val_base & EPHY_EFUSE_EXECHORC_FLAG;
        if val == EPHY_EFUSE_EXECHORC_FLAG
        {
            tmp_val = unsafe{ read_volatile(0x03051024 as *mut u32)};
            val = (( tmp_val >> 8 ) & 0xFF)<<8;
            clrsetbits(0x03009054, 0xFF00, val);
        }
        else {
            unsafe{write_volatile(0x03009054 as *mut u32, 0x0000);}
        }
        val_base= unsafe { read_volatile( EPHY_EFUSE_VALID_BIT_BASE as *mut u32 ) };
        val = val_base & EPHY_EFUSE_TXRXTERM_FLAG;
        if val == EPHY_EFUSE_TXRXTERM_FLAG
        {
            tmp_val = unsafe{ read_volatile(0x03051020 as *mut u32)};
            val = (((tmp_val>>28) & 0xF) << 4) | ( ((tmp_val>>24) & 0xF )<<8 )  ;
            clrsetbits(0x03009058, 0xFF0, val);
        }
        else {
            unsafe{write_volatile(0x03009058 as *mut u32, 0x0bb0);}
        }
        unsafe{
            write_volatile(0x0300905c as *mut u32, 0x0c10);
            write_volatile(0x03009068 as *mut u32, 0x0003);
            write_volatile(0x03009054 as *mut u32, 0x0000);
            write_volatile(0x0300907c as *mut u32, 0x1000);
            write_volatile(0x03009068 as *mut u32, 0x1000);
            write_volatile(0x0300906c as *mut u32, 0x3020);
            write_volatile(0x03009070 as *mut u32, 0x5040);
            write_volatile(0x03009074 as *mut u32, 0x7060);
            write_volatile(0x03009058 as *mut u32, 0x1708);
            write_volatile(0x0300905c as *mut u32, 0x3827);
            write_volatile(0x03009060 as *mut u32, 0x5748);
            write_volatile(0x03009064 as *mut u32, 0x7867);
            write_volatile(0x0300907c as *mut u32, 0x1100);
            write_volatile(0x03009040 as *mut u32, 0x9080);
            write_volatile(0x03009044 as *mut u32, 0xb0a0);
            write_volatile(0x03009048 as *mut u32, 0xd0c0);
            write_volatile(0x0300904c as *mut u32, 0xf0e0);
            write_volatile(0x03009050 as *mut u32, 0x9788);
            write_volatile(0x03009054 as *mut u32, 0xb8a7);
            write_volatile(0x03009058 as *mut u32, 0xd7c8);
            write_volatile(0x0300905c as *mut u32, 0xf7c8);
            write_volatile(0x0300907c as *mut u32, 0x0500);
            val= read_volatile(0x03009040 as *mut u32) | 0x0001;
            write_volatile(0x03009040 as *mut u32, val);
            
        }
    }
    fn start(&self) {
        info!("CvitekPhy start");

    }
    fn stop(&self) {
        info!("CvitekPhy stop");
    }
}
pub fn clrsetbits(addr: u32,clear:u32, set:u32){
    unsafe{ 
        let mut val:u32=read_volatile(addr as *mut u32);
        val &= !clear;
        write_volatile(addr as *mut u32, val); 
    }
}