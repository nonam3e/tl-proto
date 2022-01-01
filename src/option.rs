use crate::traits::*;

/// Skips serialization if `None`, serializes as `T` otherwise
impl<T> TlWrite for Option<T>
where
    T: TlWrite,
{
    fn max_size_hint(&self) -> usize {
        if let Some(item) = self {
            item.max_size_hint()
        } else {
            0
        }
    }

    fn write_to<P>(&self, packet: &mut P)
    where
        P: TlPacket,
    {
        if let Some(item) = self {
            item.write_to(packet)
        }
    }
}
