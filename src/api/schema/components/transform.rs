use super::{sink, source, state, Component, INVARIANT};
use crate::api::schema::metrics;
use async_graphql::Object;

#[derive(Debug, Clone)]
pub struct Data {
    pub name: String,
    pub component_type: String,
    pub inputs: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Transform(pub Data);

#[Object]
impl Transform {
    /// Transform name
    pub async fn name(&self) -> &str {
        &self.0.name
    }

    /// Transform type
    pub async fn component_type(&self) -> &str {
        &*self.0.component_type
    }

    /// Source inputs
    pub async fn sources(&self) -> Vec<source::Source> {
        self.0
            .inputs
            .iter()
            .filter_map(
                |name| match state::COMPONENTS.read().expect(INVARIANT).get(name) {
                    Some(t) => match t {
                        Component::Source(s) => Some(s.clone()),
                        _ => None,
                    },
                    _ => None,
                },
            )
            .collect()
    }

    /// Transform outputs
    pub async fn transforms(&self) -> Vec<Transform> {
        state::filter_components(|(_name, components)| match components {
            Component::Transform(t) if t.0.inputs.contains(&self.0.name) => Some(t.clone()),
            _ => None,
        })
    }

    /// Sink outputs
    pub async fn sinks(&self) -> Vec<sink::Sink> {
        state::filter_components(|(_name, components)| match components {
            Component::Sink(s) if s.0.inputs.contains(&self.0.name) => Some(s.clone()),
            _ => None,
        })
    }

    /// Metric indicating events processed for the current transform
    pub async fn processed_events_total(&self) -> Option<metrics::ProcessedEventsTotal> {
        metrics::component_processed_events_total(&self.0.name)
    }

    /// Metric indicating bytes processed for the current transform
    pub async fn processed_bytes_total(&self) -> Option<metrics::ProcessedBytesTotal> {
        metrics::component_processed_bytes_total(&self.0.name)
    }
}
