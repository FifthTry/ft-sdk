//! The SQLite backend

use super::bind_collector::SqliteBindCollector;
use super::query_builder::SqliteQueryBuilder;
use diesel::sql_types::TypeMetadata;

/// The SQLite backend
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub struct Sqlite;

/// Determines how a bind parameter is given to SQLite
///
/// Diesel deals with bind parameters after serialization as opaque blobs of
/// bytes. However, SQLite instead has several functions where it expects the
/// relevant C types.
///
/// The variants of this struct determine what bytes are expected from
/// `ToSql` impls.
#[allow(missing_debug_implementations)]
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum SqliteType {
    /// Bind using `sqlite3_bind_blob`
    Binary,
    /// Bind using `sqlite3_bind_text`
    Text,
    /// `bytes` should contain an `f32`
    Float,
    /// `bytes` should contain an `f64`
    Double,
    /// `bytes` should contain an `i16`
    SmallInt,
    /// `bytes` should contain an `i32`
    Integer,
    /// `bytes` should contain an `i64`
    Long,
}

impl diesel::backend::Backend for Sqlite {
    type QueryBuilder = SqliteQueryBuilder;
    type RawValue<'a> = super::sqlite_value::SqliteValue<'a>;
    type BindCollector<'a> = SqliteBindCollector<'a>;
}

impl TypeMetadata for Sqlite {
    type TypeMetadata = SqliteType;
    type MetadataLookup = ();
}

impl diesel::sql_types::HasSqlType<diesel::sql_types::Float> for Sqlite {
    fn metadata(_lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        SqliteType::Float
    }
}

impl diesel::sql_types::HasSqlType<diesel::sql_types::Double> for Sqlite {
    fn metadata(_lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        SqliteType::Double
    }
}

impl diesel::sql_types::HasSqlType<diesel::sql_types::Integer> for Sqlite {
    fn metadata(_lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        SqliteType::Integer
    }
}

impl diesel::sql_types::HasSqlType<diesel::sql_types::BigInt> for Sqlite {
    fn metadata(_lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        SqliteType::Long
    }
}

impl diesel::sql_types::HasSqlType<diesel::sql_types::Timestamptz> for Sqlite {
    fn metadata(_lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        SqliteType::Long
    }
}

impl diesel::sql_types::HasSqlType<diesel::sql_types::SmallInt> for Sqlite {
    fn metadata(_lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        SqliteType::Integer
    }
}

impl diesel::sql_types::HasSqlType<diesel::sql_types::Binary> for Sqlite {
    fn metadata(_lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        SqliteType::Binary
    }
}

impl diesel::sql_types::HasSqlType<diesel::sql_types::Bool> for Sqlite {
    fn metadata(_lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        SqliteType::Integer
    }
}

impl diesel::sql_types::HasSqlType<diesel::sql_types::Text> for Sqlite {
    fn metadata(_lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        SqliteType::Text
    }
}

impl diesel::sql_types::HasSqlType<diesel::sql_types::Date> for Sqlite {
    fn metadata(_lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        SqliteType::Float
    }
}

impl diesel::sql_types::HasSqlType<diesel::sql_types::Time> for Sqlite {
    fn metadata(_lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        SqliteType::Float
    }
}

impl diesel::sql_types::HasSqlType<diesel::sql_types::Timestamp> for Sqlite {
    fn metadata(_lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        SqliteType::Float
    }
}

impl diesel::backend::SqlDialect for Sqlite {
    type ReturningClause = SqliteReturningClause;

    type OnConflictClause = SqliteOnConflictClause;

    type InsertWithDefaultKeyword =
        diesel::backend::sql_dialect::default_keyword_for_insert::DoesNotSupportDefaultKeyword;
    type BatchInsertSupport = SqliteBatchInsert;
    type ConcatClause = diesel::backend::sql_dialect::concat_clause::ConcatWithPipesClause;
    type DefaultValueClauseForInsert =
        diesel::backend::sql_dialect::default_value_clause::AnsiDefaultValueClause;

    type EmptyFromClauseSyntax =
        diesel::backend::sql_dialect::from_clause_syntax::AnsiSqlFromClauseSyntax;
    type SelectStatementSyntax =
        diesel::backend::sql_dialect::select_statement_syntax::AnsiSqlSelectStatement;

    type ExistsSyntax = diesel::backend::sql_dialect::exists_syntax::AnsiSqlExistsSyntax;
    type ArrayComparison = diesel::backend::sql_dialect::array_comparison::AnsiSqlArrayComparison;
}

impl diesel::backend::DieselReserveSpecialization for Sqlite {}
impl diesel::backend::TrustedBackend for Sqlite {}

#[derive(Debug, Copy, Clone)]
pub struct SqliteOnConflictClause;

impl diesel::backend::sql_dialect::on_conflict_clause::SupportsOnConflictClause
    for SqliteOnConflictClause
{
}
impl diesel::backend::sql_dialect::on_conflict_clause::PgLikeOnConflictClause
    for SqliteOnConflictClause
{
}

#[derive(Debug, Copy, Clone)]
pub struct SqliteBatchInsert;

#[derive(Debug, Copy, Clone)]
pub struct SqliteReturningClause;

impl diesel::backend::sql_dialect::returning_clause::SupportsReturningClause
    for SqliteReturningClause
{
}
