use core::ptr::{read_volatile, write_volatile};
const  REG_SDMA_DMA_CH_REMAP0:usize=	0x03000154;
const  REG_SDMA_DMA_CH_REMAP1:usize=	0x03000158;

const table_size:usize=8;

struct Dma_Remap_Item{
    hs_id:usize,
    channel_id:u8,
}

const REMAP_TABLE:[Dma_Remap_Item;table_size]=[
    Dma_Remap_Item{
        hs_id:13,
        channel_id:1
    },
    Dma_Remap_Item{
        hs_id:12,
        channel_id:0
    },
    Dma_Remap_Item{
        hs_id:7,
        channel_id:2
    },
    Dma_Remap_Item{
        hs_id:0,
        channel_id:3
    },
    Dma_Remap_Item{
        hs_id:20,
        channel_id:4
    },
    Dma_Remap_Item{
        hs_id:21,
        channel_id:5
    },
    Dma_Remap_Item{
        hs_id:2,
        channel_id:6
    },
    Dma_Remap_Item{
        hs_id:38,
        channel_id:7
    },
];

pub fn dma_hs_remap_init()
{
    let mut remap0_val=0;
    let mut remap1_val=0;
    for i in 0..table_size{
        let hs_id=REMAP_TABLE[i].hs_id;
        let mut channel_id=REMAP_TABLE[i].channel_id;
        if channel_id < 4{
            remap0_val |= hs_id << (channel_id << 3);
        } else {
            channel_id -= 4;
            remap1_val |= hs_id << (channel_id << 3);
        }
    }
    remap0_val |= 1 << 31;
	remap1_val |= 1 << 31;
    unsafe{
        write_volatile(REG_SDMA_DMA_CH_REMAP0 as *mut u32,remap0_val as u32);
        write_volatile(REG_SDMA_DMA_CH_REMAP1 as *mut u32,remap1_val as u32);
    }
}