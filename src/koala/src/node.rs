use interface::engine::EngineType;

use crate::engine::{IQueue, OQueue};

#[derive(Debug)]
pub struct Node {
    pub(crate) id: String,
    pub(crate) engine_type: EngineType,
    pub(crate) tx_input: Vec<IQueue>,
    pub(crate) tx_output: Vec<OQueue>,
    pub(crate) rx_input: Vec<IQueue>,
    pub(crate) rx_output: Vec<OQueue>,
}

impl Node {
    pub fn new(engine_type: EngineType) -> Self {
        Node {
            id: format!("{:?}", engine_type),
            engine_type,
            tx_input: Vec::new(),
            tx_output: Vec::new(),
            rx_input: Vec::new(),
            rx_output: Vec::new(),
        }
    }

    pub fn create_from_template(n: &crate::config::Node) -> Self {
        Node {
            id: n.id.clone(),
            engine_type: n.engine_type,
            tx_input: Vec::new(),
            tx_output: Vec::new(),
            rx_input: Vec::new(),
            rx_output: Vec::new(),
        }
    }
}

#[macro_export]
macro_rules! impl_vertex_for_engine {
    ($engine:ident, $node:ident) => {
        impl crate::engine::Vertex for $engine {
            #[inline]
            fn id(&self) -> &str {
                &self.$node.id
            }
            #[inline]
            fn engine_type(&self) -> interface::engine::EngineType {
                self.$node.engine_type
            }
            #[inline]
            fn tx_inputs(&mut self) -> &mut Vec<crate::engine::IQueue> {
                &mut self.$node.tx_input
            }
            fn tx_outputs(&self) -> &Vec<crate::engine::OQueue> {
                &self.$node.tx_output
            }
            fn rx_inputs(&mut self) -> &mut Vec<crate::engine::IQueue> {
                &mut self.$node.rx_input
            }
            fn rx_outputs(&self) -> &Vec<crate::engine::OQueue> {
                &self.$node.rx_output
            }
        }
    };
}
