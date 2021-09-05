pub trait Document {}

pub trait IntoDocument<D>
where
    D: Document,
{
    type Err;

    fn validate(self) -> Result<D, Self::Err>;
}
