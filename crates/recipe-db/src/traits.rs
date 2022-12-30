// TODO: find out all the bazillion bounds that need to go
// on the outputs here so that I can actually be generic over them.
// e.g. Output needs to be Query + QueryFragment, the query result needs
// to be FromSqlRow, there's something to do with a QueryId...

pub trait All<Db> {
    type Output;
    fn all() -> Self::Output;
}

pub trait Findable<'a> {
    type Id;
    type FindById;
    type FindByIds;
    fn by_id(id: Self::Id) -> Self::FindById;
    fn by_ids(ids: &'a [Self::Id]) -> Self::FindByIds;
}
