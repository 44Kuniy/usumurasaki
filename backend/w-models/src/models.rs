mod affiliate_item;
mod chennel;
mod errors;
mod partner;
mod video;

pub use affiliate_item::*;
pub use chennel::*;
pub use errors::*;
pub use partner::*;
pub use video::*;

pub trait ToSqlValues<T> {
    fn into_sql_values(self) -> String;
}

pub trait ToSqlValuesWithStringArg<T> {
    fn into_sql_values(self, arg: String) -> String;
}

pub trait ToSqlValuesWithi64Arg<T> {
    fn into_sql_values(self, arg: i64) -> String;
}

pub trait ToSqlValue {
    fn into_sql_value(self) -> String;
}
