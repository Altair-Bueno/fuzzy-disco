/// The Document trait marks structs as "mongo approved", which means they are
/// designed for consistency on a mongodb database
pub trait Document {}

/// The `IntoDocument` trait allows external data structures to be transformed
/// into valid Document implementations. This enforces the developer to always
/// insert the right data on mongo
///
/// Because mongo doesn't enforce an scheme, this trait discourages unsafe
/// deserialization of data, preventing data corruption. Unsafe
/// `Deserialization` structures can implement the `IntoDocument` trait for
/// easy validation of data
pub trait IntoDocument<D>
where
    D: Document,
{
    type Err;

    fn validate(&self) -> Result<D, Self::Err>;
}
