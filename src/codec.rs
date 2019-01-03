/*use bytes::Bytes;
use bytes::BytesMut;

pub struct Codec;

impl tokio::codec::Encoder for Codec {
    type Item = ();
    type Error = ();
    fn encode(
        &mut self, 
        item: Self::Item, 
        dst: &mut BytesMut
    ) -> Result<(), Self::Error>
    {
        unimplemented!()
    }
}

impl tokio::codec::Decoder for Codec {
    type Item = ();
    type Error = ();

    fn decode(
        &mut self, 
        src: &mut BytesMut
    ) -> Result<Option<Self::Item>, Self::Error>
    {
        unimplemented!()
    }
}
*/