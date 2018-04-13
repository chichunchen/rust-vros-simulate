extern crate simulator;

use simulator::Viewport;

#[test]
fn test_trace() {
    let t_1 = Viewport::new(100, 1, 100, 1200, 1200);
    let t_2 = t_1.create_new(1400, 1400);
    println!("{:?}", t_2);
    assert!(false);
}