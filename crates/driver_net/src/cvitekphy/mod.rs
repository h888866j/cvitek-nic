
pub use super::CvitekPhyTraits;
pub use super::CvitekPhyDevice;
use core::marker::PhantomData;
use super::CvitekNicDevice;
pub struct CvitekPhy<A>
where
    A: CvitekPhyTraits,
{
    device: CvitekPhyDevice<A>,
    phantom: PhantomData<A>,
}