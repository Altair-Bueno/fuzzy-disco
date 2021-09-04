pub trait Document{

}

pub trait IntoDocument<D> where D:Document{
    type Err;

    fn validate(self:Self) -> Result<D,Self::Err>;
}