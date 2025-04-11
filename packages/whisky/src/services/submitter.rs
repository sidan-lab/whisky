use async_trait::async_trait;
// use cardano_serialization_lib::error::WError;

// use crate::model::Action;

#[async_trait]
pub trait Submitter: Send {}
