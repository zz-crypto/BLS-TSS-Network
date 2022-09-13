pub mod dkg_phase;
pub mod dkg_post_process;
pub mod dkg_success;
pub mod new_block;
pub mod new_dkg_task;
pub mod new_group_relay_confirmation_task;
pub mod new_group_relay_task;
pub mod new_randomness_task;
pub mod ready_to_fulfill_group_relay_confirmation_task;
pub mod ready_to_fulfill_group_relay_task;
pub mod ready_to_fulfill_randomness_task;
pub mod ready_to_handle_group_relay_confirmation_task;
pub mod ready_to_handle_group_relay_task;
pub mod ready_to_handle_randomness_task;
pub mod run_dkg;
pub mod types;

use self::types::Topic;
pub trait Event {
    fn topic(&self) -> Topic;
}
