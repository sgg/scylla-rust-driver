use std::str::FromStr;

/// A type that provides a partitioning algorithm
pub trait Partitioner {
    /// TODO(sgg): doc
    ///
    /// # Examples
    /// `org.apache.cassandra.dht.Murmur3Partitioner` -> `Murmur3Partitioner`
    /// `org.apache.cassandra.dht.RandomPartitioner` -> `RandomPartitioner`
    const CLASS_NAME: &'static str;

    /// The returned by the Partitioner.
    type Token: Token;
    type ParseErr:;

    /// Generate a Token for the routing key.
    fn hash(key: impl AsRef<[u8]>) -> Self::Token;

    /// Parse a token retrieved from Cassandra
    fn parse_token(token: impl AsRef<str>) -> Result<Self::Token, Self::ParseErr>;
}

pub trait Token {}

mod murmur3 {
    use super::*;

    pub struct Murmur3Partitioner {}

    impl Partitioner for Murmur3Partitioner {
        const CLASS_NAME: &'static str = "Murmur3Partitioner";
        type Token = Murmur3Token;
        type ParseErr = <Murmur3Token as FromStr>::Err;

        fn hash(partition_key: impl AsRef<[u8]>) -> Self::Token {
            todo!()
        }

        fn parse_token(token: impl AsRef<str>) -> Result<Self::Token, Self::ParseErr> {
            todo!()
        }
    }

    /// A token returned by the [`Murmur3`] partitioner.
    ///
    /// ## References
    ///
    /// [Scylla Source](https://github.com/scylladb/scylla/blob/f186de909d461003e83ce24f256c2fb6884b6004/dht/token.hh#L45)
    /// [Cassandra Source](https://github.com/apache/cassandra/blob/5cbdb2e58e870535af61204898a1e2bbf6cb5f64/src/java/org/apache/cassandra/dht/Murmur3Partitioner.java#L143)
    /// [DataStax Java Driver](https://github.com/datastax/java-driver/blob/ef56d561d97adcae48e0e6e8807f334aedc0d783/core/src/main/java/com/datastax/oss/driver/internal/core/metadata/token/Murmur3Token.java#L26)
    /// [DataStax C++ Driver](https://github.com/datastax/cpp-driver/blob/7f193cb347948b79b1eadf44c8235ee04d66f3cf/src/token_map_impl.hpp#L111:8)
    pub struct Murmur3Token {
        inner: i64,
    }

    impl std::str::FromStr for Murmur3Token {
        type Err = std::num::ParseIntError;
        fn from_str(s: &str) -> Result<Self, std::num::ParseIntError> {
            Ok(Self { inner: s.parse()? })
        }
    }
}

/// Implementation for the random partitioner
mod random {
    use super::*;
    /// TODO
    ///
    /// https://github.com/datastax/cpp-driver/blob/7f193cb347948b79b1eadf44c8235ee04d66f3cf/src/token_map_impl.cpp#L131
    pub struct RandomPartitioner {}

    impl Partitioner for RandomPartitioner {
        const CLASS_NAME: &'static str = "RandomPartitioner";
        type Token = RandomToken;
        type ParseErr = <RandomToken as FromStr>::Err;

        fn hash(key: impl AsRef<[u8]>) -> Self::Token {
            let md5::Digest(digest) = md5::compute(key);
            Self::Token {
                inner: i128::from_be_bytes(digest).abs(),
            }
        }

        fn parse_token(token: impl AsRef<str>) -> Result<Self::Token, Self::ParseErr> {
            token.as_ref().parse()
        }
    }

    pub struct RandomToken {
        inner: i128,
    }

    impl std::str::FromStr for RandomToken {
        type Err = std::num::ParseIntError;
        fn from_str(s: &str) -> Result<Self, std::num::ParseIntError> {
            Ok(Self { inner: s.parse()? })
        }
    }

    #[cfg(test)]
    mod tests {

    }
}
