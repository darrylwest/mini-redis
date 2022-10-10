
use crate::{Connection, Frame, Parse, ParseError};
use bytes::Bytes;
use tracing::instrument;

/// Returns db size
///
#[derive(Debug, Default)]
pub struct DbSize { 
    dbsize: Option<String>,
}

impl DbSize {
    /// Create a new `DbSize` command.
    pub fn new(dbsize: Option<String>) -> DbSize {
        DbSize { dbsize }
    }

    /// Parse a `DbSize` instance from a received frame.
    ///
    pub(crate) fn parse_frames(parse: &mut Parse) -> crate::Result<DbSize> {
        match parse.next_string() {
            Ok(dbsize) => Ok(DbSize::new(Some(dbsize))),
            Err(ParseError::EndOfStream) => Ok(DbSize::new(Some("0".to_string()))),
            Err(e) => Err(e.into()),
        }
    }

    /// Apply the `DbSize` command and return size.
    ///
    /// The response is written to `dst`. This is called by the server in order
    /// to execute a received command.
    #[instrument(skip(self, dst))]
    pub(crate) async fn apply(self, dst: &mut Connection) -> crate::Result<()> {
        let response = match self.dbsize {
            None => Frame::Simple("0".to_string()),
            Some(dbsize) => Frame::Bulk(Bytes::from(dbsize)),
        };

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
        frame.push_bulk(Bytes::from("ping".as_bytes()));
        if let Some(msg) = self.dbsize {
            frame.push_bulk(Bytes::from(msg));
        }
        frame
    }
}
