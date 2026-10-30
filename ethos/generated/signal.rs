#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Configure(flow_nexus_types::Configuration),
    Declare(flow_nexus_types::Declaration),
    Threshold(flow_nexus_types::Threshold),
    Observe,
    Debug,
    Mutate(flow_nexus_types::evidence::MutationBatch),
    Read(flow_nexus_types::evidence::ReadSelector),
    Reconcile(flow_nexus_types::evidence::VerificationTarget),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Refused_Data {
    NotOwner,
    StoreRefused,
    ConfigurationRefused,
    Integrity(flow_nexus_types::evidence::Invariant),
    Unavailable,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    Configured,
    Declared,
    Thresholded,
    Observed(std::vec::Vec<flow_nexus_types::Declaration>),
    Debugged(flow_nexus_types::evidence::DebugProjection),
    Mutated,
    ReadRecords(std::vec::Vec<flow_nexus_types::evidence::Record>),
    Reconciled,
    Refused(Refused_Data),
}
