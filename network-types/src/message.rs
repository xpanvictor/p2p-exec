pub mod execution {
    include!(concat!(env!("OUT_DIR"), "/execution.rs"));
}

#[cfg(test)]
mod test {
    use crate::message::execution;

    #[test]
    fn check_execution_msg() {
        let msg = execution::ExecutionTask { id: 1 };
        assert!(msg.id == 1);
    }
}
