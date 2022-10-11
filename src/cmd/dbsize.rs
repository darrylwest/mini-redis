use crate::{Connection, Db, Frame, Parse, ParseError};
use bytes::Bytes;
use tracing::instrument;

/// Returns db size
///
#[derive(Debug, Default)]
pub struct DbSize {
    size: u64,
}

impl DbSize {
    /// Create a new `DbSize` command.
    pub fn new(size: u64) -> DbSize {
        DbSize { size }
    }

    /// return the number of elements in the db
    pub fn count(&self) -> u64 {
        self.size
    }

    /// Parse a `DbSize` instance from a received frame.
    ///
    /// if the return is successful the size is returned wrapped in a DbSize struct
    /// wrapped in Ok.
    ///
    pub(crate) fn parse_frames(parse: &mut Parse) -> crate::Result<DbSize> {
        match parse.next_int() {
            Ok(dbsize) => Ok(DbSize::new(dbsize)),
            Err(ParseError::EndOfStream) => Ok(DbSize::new(0u64)),
            Err(e) => Err(e.into()),
        }
    }

    /// Apply the `DbSize` command and return size.
    ///
    /// The response is written to `dst`. This is called by the server in order
    /// to execute a received command.
    #[instrument(skip(self, dst))]
    pub(crate) async fn apply(self, db: &Db, dst: &mut Connection) -> crate::Result<()> {
        let size = db.dbsize(); // .to_string();

        // let response = Frame::Simple(size);
        let response = Frame::Integer(size);

        // Write the response back to the client
        dst.write_frame(&response).await?;

        Ok(())
    }

    /// Converts the command into an equivalent `Frame`.
    ///
    /// This is called by the client when encoding a `DbSize` command sent to the server.
    ///
    pub(crate) fn into_frame(self) -> Frame {
        let mut frame = Frame::array();
        frame.push_bulk(Bytes::from("dbsize".as_bytes()));

        frame
    }
}
