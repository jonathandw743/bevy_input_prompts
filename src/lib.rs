use fs_extra::error::Error;

pub mod product_ids;
pub mod vendor_ids;

pub mod key_code;
pub mod gamepad_button;

pub enum Pack {
    #[cfg(feature = "use_kenney_input_prompts")]
    Kenney,
}

pub fn copy_assets() -> Result<(), Error> {
    #[cfg(feature = "use_kenney_input_prompts")]
    kenney_input_prompts::copy_assets("assets/bevy_input_prompts")?;
    Ok(())
}