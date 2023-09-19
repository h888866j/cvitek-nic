pub use core::marker::PhantomData;
pub struct CvitekPhyDevice<A: CvitekPhyTraits>
{
    phantom: PhantomData<A>,
}
pub trait CvitekPhyTraits {
    fn phys_to_virt(pa: usize) -> usize {
        pa
    }
    fn virt_to_phys(va: usize) -> usize {
        va
    }
    fn dma_alloc_pages(pages: usize) -> (usize, usize);

    fn dma_free_pages(vaddr: usize, pages: usize);

    fn mdelay(m_times: usize);
}