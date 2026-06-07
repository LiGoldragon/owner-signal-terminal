use nota_next::{NotaDecode, NotaEncode, NotaSource};
use owner_signal_terminal::{
    CreateSession, Frame, FrameBody, OwnerTerminalOperationKind, OwnerTerminalReply,
    OwnerTerminalRequest, OwnerTerminalRequestUnimplemented, OwnerTerminalUnimplementedReason,
    RetireSession, SessionCreated, SessionRetired, TerminalCommand, TerminalCommandArgument,
    TerminalCommandExecutable, TerminalEnvironmentBinding, TerminalEnvironmentName,
    TerminalEnvironmentValue, TerminalExitStatus, TerminalName, TerminalWorkingDirectory,
};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, Request, SessionEpoch,
    SignalOperationHeads, SubReply,
};

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn terminal() -> TerminalName {
    TerminalName::new("operator")
}

fn command() -> TerminalCommand {
    TerminalCommand {
        executable: TerminalCommandExecutable::new("pi"),
        arguments: vec![TerminalCommandArgument::new("--model")],
    }
}

fn environment() -> TerminalEnvironmentBinding {
    TerminalEnvironmentBinding {
        name: TerminalEnvironmentName::new("TERM"),
        value: TerminalEnvironmentValue::new("xterm-256color"),
    }
}

fn data_socket_path() -> signal_engine_management::WirePath {
    signal_engine_management::WirePath::new("/run/persona/terminal/sessions/operator/data.sock")
}

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn round_trip_request(request: OwnerTerminalRequest) -> OwnerTerminalRequest {
    let frame = Frame::new(FrameBody::Request {
        exchange: exchange(),
        request: Request::from_payload(request),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Request { request, .. } => {
            let (payload, tail) = request.payloads.into_head_and_tail();
            assert!(tail.is_empty(), "one request payload is expected");
            payload
        }
        other => panic!("expected request operation, got {other:?}"),
    }
}

fn round_trip_reply(reply: OwnerTerminalReply) -> OwnerTerminalReply {
    let frame = Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply))),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            other => panic!("expected accepted reply, got {other:?}"),
        },
        other => panic!("expected reply operation, got {other:?}"),
    }
}

fn round_trip_nota<T>(value: T, expected: &str)
where
    T: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let encoded = value.to_nota();
    assert_eq!(encoded, expected);

    let recovered = NotaSource::new(&encoded)
        .parse::<T>()
        .expect("decode nota text");
    assert_eq!(recovered, value);
    assert!(
        CANONICAL.contains(expected),
        "examples/canonical.nota missing line: {expected}"
    );
}

#[test]
fn owner_terminal_requests_round_trip() {
    let create = OwnerTerminalRequest::CreateSession(CreateSession {
        name: terminal(),
        command: command(),
        environment: vec![environment()],
        working_directory: Some(TerminalWorkingDirectory::new("/workspace")),
    });
    assert_eq!(round_trip_request(create.clone()), create);

    let retire = OwnerTerminalRequest::RetireSession(RetireSession { name: terminal() });
    assert_eq!(round_trip_request(retire.clone()), retire);
}

#[test]
fn owner_terminal_replies_round_trip() {
    let created = OwnerTerminalReply::SessionCreated(SessionCreated {
        name: terminal(),
        data_socket_path: data_socket_path(),
    });
    assert_eq!(round_trip_reply(created.clone()), created);

    let retired = OwnerTerminalReply::SessionRetired(SessionRetired {
        name: terminal(),
        exit_status: Some(TerminalExitStatus::StatusUnavailable),
    });
    assert_eq!(round_trip_reply(retired.clone()), retired);

    let unimplemented =
        OwnerTerminalReply::OwnerTerminalRequestUnimplemented(OwnerTerminalRequestUnimplemented {
            terminal: terminal(),
            operation: OwnerTerminalOperationKind::CreateSession,
            reason: OwnerTerminalUnimplementedReason::NotBuiltYet,
        });
    assert_eq!(round_trip_reply(unimplemented.clone()), unimplemented);
}

#[test]
fn owner_terminal_request_heads_are_contract_local_operations() {
    assert_eq!(
        <OwnerTerminalRequest as SignalOperationHeads>::HEADS,
        &["CreateSession", "RetireSession"]
    );
}

#[test]
fn owner_terminal_request_exposes_contract_owned_operation_kind() {
    let create = OwnerTerminalRequest::CreateSession(CreateSession {
        name: terminal(),
        command: command(),
        environment: vec![environment()],
        working_directory: Some(TerminalWorkingDirectory::new("/workspace")),
    });
    assert_eq!(
        create.operation_kind(),
        OwnerTerminalOperationKind::CreateSession
    );

    let retire = OwnerTerminalRequest::RetireSession(RetireSession { name: terminal() });
    assert_eq!(
        retire.operation_kind(),
        OwnerTerminalOperationKind::RetireSession
    );
}

#[test]
fn owner_terminal_canonical_examples_round_trip() {
    round_trip_nota(
        OwnerTerminalRequest::CreateSession(CreateSession {
            name: terminal(),
            command: TerminalCommand {
                executable: TerminalCommandExecutable::new("pi"),
                arguments: Vec::new(),
            },
            environment: Vec::new(),
            working_directory: None,
        }),
        "(CreateSession ([operator] ([pi] []) [] None))",
    );
    round_trip_nota(
        OwnerTerminalRequest::RetireSession(RetireSession { name: terminal() }),
        "(RetireSession ([operator]))",
    );
    round_trip_nota(
        OwnerTerminalReply::SessionCreated(SessionCreated {
            name: terminal(),
            data_socket_path: data_socket_path(),
        }),
        "(SessionCreated ([operator] [/run/persona/terminal/sessions/operator/data.sock]))",
    );
    round_trip_nota(
        OwnerTerminalReply::SessionRetired(SessionRetired {
            name: terminal(),
            exit_status: Some(TerminalExitStatus::StatusUnavailable),
        }),
        "(SessionRetired ([operator] (Some (StatusUnavailable))))",
    );
    round_trip_nota(
        OwnerTerminalReply::OwnerTerminalRequestUnimplemented(OwnerTerminalRequestUnimplemented {
            terminal: terminal(),
            operation: OwnerTerminalOperationKind::CreateSession,
            reason: OwnerTerminalUnimplementedReason::NotBuiltYet,
        }),
        "(OwnerTerminalRequestUnimplemented ([operator] CreateSession NotBuiltYet))",
    );
}
