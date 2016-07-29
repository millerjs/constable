use bincode;
use std::io::Error as IOError;

quick_error! {
    #[derive(Debug)]
    pub enum ConstableError {
        /// IO Error
        Io(err: IOError) { from() }
        /// Encoding Error
        Encoding(err: bincode::rustc_serialize::EncodingError) { from() }
        /// Decoding Error
        Decoding(err: bincode::rustc_serialize::DecodingError) { from() }
    }
}
