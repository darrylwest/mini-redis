use crate::{Connection, Db, Frame, Parse, ParseError};
use bytes::Bytes;
use tracing::instrument;

/// Returns db size
///
#[derive(Debug, Default)]
pub struct Count {
    size: u64,
}

impl Count {
    /// Create a new `Count` command.
    pub fn new(size: u64) -> Count {
        Count { size }
    }

    /// return the number of elements in the db
    pub fn count(&self) -> u64 {
        self.size
    }

    /// Parse a `Count` instance from a received frame.
    ///
    /// if the return is successful the size is returned wrapped in a Count struct
    /// wrapped in Ok.
    ///
    pub(crate) fn parse_frames(parse: &mut Parse) -> crate::Result<Count> {
        match parse.next_int() {
            Ok(count) => Ok(Count::new(count)),
            Err(ParseError::EndOfStream) => Ok(Count::new(0u64)),
            Err(e) => Err(e.into()),
        }
    }

    /// Apply the `Count` command and return size.
    ///
    /// The response is written to `dst`. This is called by the server in order
    /// to execute a received command.
    #[instrument(skip(self, dst))]
    pub(crate) async fn apply(self, db: &Db, dst: &mut Connection) -> crate::Result<()> {
        let size = db.count(); // .to_string();

        // let response = Frame::Simple(size);
        let response = Frame::Integer(size);

        // Write the response back to the client
        dst.write_frame(&response).await?;

        Ok(())
    }

    /// Converts the command into an equivalent `Frame`.
    ///
    /// This is called by the client when encoding a `Count` command sent to the server.
    ///
    pub(crate) fn into_frame(self) -> Frame {
        let mut frame = Frame::array();
        frame.push_bulk(Bytes::from("count".as_bytes()));

        frame
    }
}
