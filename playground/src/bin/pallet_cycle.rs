// ANCHOR: all
// ANCHOR: preamble
use quokkasim::prelude::*;
use std::time::{Duration, SystemTime};
// ANCHOR_END: preamble
fn main() {

    // Components

    // ANCHOR: components
    let mut df = DistributionFactory::new(97531);

    let mut empty_pallets: DefaultDiscStock<i32, DiscStockState, DiscStockLog<i32>> = DefaultDiscStock::new()
        .with_name("EmptyPallets")
        .with_code("EP")
        .with_initial_resources(vec![0,1,2,3].into());
    let (empty_pallets_mbox, empty_pallets_addr) = empty_pallets.create_mailbox();

    let mut empty_pallet_transport = DefaultDiscProcess::new()
        .with_name("EmptyPalletTransport")
        .with_code("EPT")
        .with_process_time_distr(df.create(DistributionConfig::Exponential { mean: 60.0 }).unwrap());
    let (empty_pallet_transport_mbox, empty_pallet_transport_addr) = empty_pallet_transport.create_mailbox();

    let mut loaded_pallets: DefaultDiscStock<i32, DiscStockState, DiscStockLog<i32>> = DefaultDiscStock::new()
        .with_name("FullPallets")
        .with_code("FP")
        .with_initial_resources(vec![4,5,6,7].into());
    let (loaded_pallets_mbox, loaded_pallets_addr) = loaded_pallets.create_mailbox();

    let mut loaded_pallet_transport = DefaultDiscProcess::new()
        .with_name("FullPalletTransport")
        .with_code("FPT")
        .with_process_time_distr(df.create(DistributionConfig::Exponential { mean: 120.0 }).unwrap());
    let (loaded_pallet_transport_mbox, loaded_pallet_transport_addr) = loaded_pallet_transport.create_mailbox();
            
    // Connections

    let mut c = Connection {};
    c.connect((&mut empty_pallets, &empty_pallets_addr), (&mut empty_pallet_transport, &empty_pallet_transport_addr)).unwrap();
    c.connect((&mut empty_pallet_transport, &empty_pallet_transport_addr), (&mut loaded_pallets, &loaded_pallets_addr)).unwrap();
    c.connect((&mut loaded_pallets, &loaded_pallets_addr), (&mut loaded_pallet_transport, &loaded_pallet_transport_addr)).unwrap();
    c.connect((&mut loaded_pallet_transport, &loaded_pallet_transport_addr), (&mut empty_pallets, &empty_pallets_addr)).unwrap();

    // ANCHOR_END: components

    // Loggers
    // ANCHOR: loggers
    let process_logger = EventQueue::<DiscProcessLog<i32>>::new();
    empty_pallet_transport.log_emitter.connect_sink(&process_logger);
    loaded_pallet_transport.log_emitter.connect_sink(&process_logger);


    let stock_logger = EventQueue::<DiscStockLog<i32>>::new();
    empty_pallets.log_emitter.connect_sink(&stock_logger);
    loaded_pallets.log_emitter.connect_sink(&stock_logger);
    // ANCHOR_END: loggers

    // Simulation initialisation
    // ANCHOR: sim
    let sim_init = SimInit::new()
        .add_model(empty_pallets, empty_pallets_mbox, "EP")
        .add_model(empty_pallet_transport, empty_pallet_transport_mbox, "EPT")
        .add_model(loaded_pallets, loaded_pallets_mbox, "FP")
        .add_model(loaded_pallet_transport, loaded_pallet_transport_mbox, "FPT");

    let start_time = MonotonicTime::try_from_date_time(2025, 7, 1, 0, 0, 0, 0).unwrap();
    let duration = Duration::from_secs(3600);

    let (mut sim, _) = sim_init.init(start_time).unwrap();
    // ANCHOR_END: sim
    // ANCHOR: run
    let time_at_start = SystemTime::now();
    sim.step_until(start_time + duration).unwrap();
    let time_at_end = SystemTime::now();

    // Output logs

    for log in process_logger.into_reader() {
        println!("{:?}", log);
    }

    for log in stock_logger.into_reader() {
        println!("{:?}", log);
    }

    println!("Execution time: {:?}", time_at_end.duration_since(time_at_start));

    // ANCHOR_END: run
}