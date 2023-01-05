use app_io::*;
use gtest::{Log, Program, System};

#[test]
fn test() {
    let system = System::new();

    system.init_logger();

    let program = Program::current(&system);
    let mut result = program.send_bytes(2, []);

    assert!(!result.main_failed());

    result = program.send(2, PingPong::Pong);

    assert!(result.log().is_empty());

    // meta_state()

    // AppStateQueryReply::AllState

    let mut expected_state = vec![];

    for mut actor in 0..=100 {
        actor += 2;
        result = program.send(actor, PingPong::Ping);

        assert!(result.contains(&Log::builder().payload(PingPong::Pong)));

        expected_state.push((actor.into(), 1))
    }

    let mut state = if let AppStateQueryReply::AllState(state) =
        program.meta_state(AppStateQuery::AllState).unwrap()
    {
        state
    } else {
        unreachable!();
    };

    expected_state.sort();
    state.sort();

    assert_eq!(state, expected_state);

    // AppStateQueryReply::PingCount

    result = program.send(2, PingPong::Ping);

    assert!(result.contains(&Log::builder().payload(PingPong::Pong)));

    let ping_count = if let AppStateQueryReply::PingCount(ping_count) = program
        .meta_state(AppStateQuery::PingCount(2.into()))
        .unwrap()
    {
        ping_count
    } else {
        unreachable!();
    };

    assert_eq!(ping_count, 2);
}
