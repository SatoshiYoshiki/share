pub mod exports {
    pub use super::NumberStatus as NumberStatusEnum;
}

#[derive(SqlType)]
#[postgres(type_name = "NumberStatusEnum")]
pub struct NumberStatus;

#[derive(Debug, FromSqlRow, AsExpression)]
#[sql_type = "NumberStatus"]
pub enum NumberStatusEnum {
    Available,
    Using,
    ReleaseTarget,
}

use std::io::Write;

use diesel::backend::Backend;
use diesel::serialize::{self, IsNull, Output, ToSql};

impl<Db: Backend> ToSql<NumberStatus, Db> for NumberStatusEnum {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Db>) -> serialize::Result {
        match *self {
            NumberStatusEnum::Available => out.write_all(b"AVAILABLE")?,
            NumberStatusEnum::Using => out.write_all(b"USING")?,
            NumberStatusEnum::ReleaseTarget => out.write_all(b"RELEASE_TARGET")?,
        }
        Ok(IsNull::No)
    }
}

use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;

impl FromSql<NumberStatus, Pg> for NumberStatusEnum {
    fn from_sql(bytes: Option<&<Pg as Backend>::RawValue>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"AVAILABLE" => Ok(NumberStatusEnum::Available),
            b"USING" => Ok(NumberStatusEnum::Using),
            b"RELEASE_TARGET" => Ok(NumberStatusEnum::ReleaseTarget),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
