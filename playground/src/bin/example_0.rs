// ANCHOR: all
// ANCHOR: preamble
use std::{error::Error, time::Duration};
use quokkasim::{define_model_enums, prelude::*}; 
use std::fs::create_dir_all;

define_model_enums! {
    pub enum ComponentModel {}
    pub enum ComponentModelAddress {}
    pub enum ComponentLogger {}
    pub enum ScheduledEvent {}
}

impl CustomComponentConnection for ComponentModel {
    fn connect_components(a: &mut Self, b: &mut Self, n: Option<usize>) -> Result<(), Box<dyn Error>> {
        match (a, b) {
            (a, b) => Err(format!("No component connection defined from {} to {} (n={:?})", a, b, n).into()),
        }
    }
}

impl CustomLoggerConnection for ComponentLogger { 
    type ComponentType = ComponentModel;
    fn connect_logger(a: &mut Self, b: &mut Self::ComponentType, n: Option<usize>) -> Result<(), Box<dyn Error>> {
        match (a, b, n) {
            (a, b, _) => Err(format!("No logger connection defined from {} to {} (n={:?})", a, b, n).into()),
        }
    }
}

impl CustomInit for ComponentModelAddress {
    fn initialise(&mut self, simu: &mut Simulation) -> Result<(), ExecutionError> {
        let notif_meta = NotificationMetadata {
            time: simu.time(),
            element_from: "Init".into(),
            message: "Start".into(),
        };
        match self {
            _ => {
                Err(ExecutionError::BadQuery)
            }
        }
    }
}
// ANCHOR_END: preamble

fn main() {
    
    /*
     * Create components
     */
    // ANCHOR: components
    let mut stock1 = ComponentModel::VectorStockF64(
        VectorStock::new()
            .with_name("Stock1".to_string())
            .with_low_capacity(5.)
            .with_max_capacity(30.)
            .with_initial_vector(30.),
        Mailbox::new()
    );

    let mut process1 = ComponentModel::VectorProcessF64(
        VectorProcess::new()
            .with_name("Process1".to_string())
            .with_process_quantity_distr(Distribution::Constant(12.))
            .with_process_time_distr(Distribution::Constant(3.)),
        Mailbox::new()
    );
    let mut process_addr = process1.get_address();

    let mut stock2 = ComponentModel::VectorStockF64(
        VectorStock::new()
            .with_name("Stock2".to_string())
            .with_low_capacity(15.)
            .with_max_capacity(30.)
            .with_initial_vector(0.),
        Mailbox::new()
    );

    connect_components!(&mut stock1, &mut process1).unwrap();
    connect_components!(&mut process1, &mut stock2).unwrap();
    // ANCHOR_END: components

    /*
     * Create loggers
     */
    // ANCHOR: loggers
    let mut stock_logger = ComponentLogger::VectorStockLoggerF64(VectorStockLogger::new("StockLogger".into()));
    let mut process_logger = ComponentLogger::VectorProcessLoggerF64(VectorProcessLogger::new("ProcessLogger".into()));

    connect_logger!(&mut stock_logger, &mut stock1).unwrap();
    connect_logger!(&mut stock_logger, &mut stock2).unwrap();
    connect_logger!(&mut process_logger, &mut process1).unwrap();
    // ANCHOR_END: loggers

    /*
     * Build simulation
     */
    // ANCHOR: sim
    let mut sim_builder = SimInit::new();
    sim_builder = register_component!(sim_builder, stock1);
    sim_builder = register_component!(sim_builder, process1);
    sim_builder = register_component!(sim_builder, stock2);

    let start_time = MonotonicTime::EPOCH;
    let mut simu = sim_builder.init(MonotonicTime::EPOCH).unwrap().0;
    // ANCHOR_END: sim

    /*
     * Run simulation
     */
    // ANCHOR: run
    process_addr.initialise(&mut simu).unwrap();

    let end_time = start_time + Duration::from_secs(3600);
    simu.step_until(end_time).unwrap();

    let output_dir = "outputs/example_0";
    create_dir_all(&output_dir).unwrap();
    process_logger.write_csv(output_dir).unwrap();
    stock_logger.write_csv(output_dir).unwrap();
    // ANCHOR_END: run
}
// ANCHOR_END: all