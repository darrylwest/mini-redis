use crate::{Connection, Db, Frame, Parse};

use bytes::Bytes;
use tracing::{debug, instrument};

/// Keys returns all the database keys.
///
#[derive(Debug)]
pub struct Keys {
    /// Name of the key to get
    keys: String, // comma separated
}

impl Keys {
    /// Create a new `Keys` command which fetches `key`.
    pub fn new(keys: String) -> Keys {
        Keys { keys }
    }

    /// get the keys
    pub fn keys(&self) -> &String {
        &self.keys
    }

    /// Parse a `Keys` instance from a received frame.
    ///
    pub(crate) fn parse_frames(_parse: &mut Parse) -> crate::Result<Keys> {
        // The `Keys` string has already been consumed. The next value is the
        // name of the key to get. If the next value is not a string or the
        // input is fully consumed, then an error is returned.
        // let keys = parse.next_string()?;
        let keys = "123,456".to_string();

        Ok(Keys { keys })
    }

    /// Apply the `Keys` command to the specified `Db` instance.
    ///
    /// The response is written to `dst`. This is called by the server in order
    /// to execute a received command.
    #[instrument(skip(self, db, dst))]
    pub(crate) async fn apply(self, db: &Db, dst: &mut Connection) -> crate::Result<()> {
        // get the keys from the shared database state
        let _list = db.keys();

        let response = {
            // write the list to a byte array...
            let bytes = Bytes::new();
            Frame::Bulk(bytes)
        };

        debug!(?response);

        // Write the response back to the client
        dst.write_frame(&response).await?;

        Ok(())
    }

    /// Converts the `Keys` command into an equivalent `Frame`.
    ///
    /// This is called by the client when encoding a `Keys` command to send to
    /// the server.
    pub(crate) fn into_frame(self) -> Frame {
        let mut frame = Frame::array();
        frame.push_bulk(Bytes::from("keys".as_bytes()));

        frame
    }
}
