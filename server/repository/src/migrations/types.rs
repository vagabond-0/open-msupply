#[cfg(feature = "postgres")]
pub(crate) const DATE: &'static str = "DATE";
#[cfg(not(feature = "postgres"))]
pub(crate) const DATE: &'static str = "TEXT";
#[cfg(feature = "postgres")]
pub(crate) const DATETIME: &'static str = "TIMESTAMP";
#[cfg(not(feature = "postgres"))]
pub(crate) const DATETIME: &'static str = "TEXT";
#[cfg(feature = "postgres")]
pub(crate) const DOUBLE: &'static str = "DOUBLE PRECISION";
#[cfg(not(feature = "postgres"))]
pub(crate) const DOUBLE: &'static str = "REAL";
