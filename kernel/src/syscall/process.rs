use crate::batch::run_next_app;

/// print the return value of the exited application and call run_next_app to switch to the next application
pub fn sys_exit(xstate: i32) -> ! {
    error!("application exited with code {}!!!", xstate);
    run_next_app()
}
