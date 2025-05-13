#![no_std]
extern crate alloc;
use alloc::{format, string::String};

pub use fs_extra::error::Error as CopyAssetsError;

pub mod gamepad_brand;

mod char;
mod gamepad_axis;
mod gamepad_button;
mod key;
mod key_code;
mod keyboard_input;
mod mouse_button;

#[cfg(feature = "kenney_input_prompts")]
pub use kenney_input_prompts::tokenize_dir::_kenney_input_prompts_1_4 as kenney_tokenize;
#[cfg(feature = "xelu_free_controller_key_prompts")]
pub use xelu_free_controller_key_prompts::tokenize_dir::_Xelu_Free_Controller_Key_Prompts as xelu_tokenize;

#[derive(Clone, Copy)]
pub enum Pack {
    #[cfg(feature = "kenney_input_prompts")]
    Kenney,
    #[cfg(feature = "xelu_free_controller_key_prompts")]
    Xelu,
}

pub fn first_file_path(pack: Pack, files: impl tokenize_dir::ToConstraints) -> Option<String> {
    let file_index = tokenize_dir::first_value_nonstrict(files.to_constraints())?;
    Some(format!(
        "bevy_input_prompts/{}",
        match pack {
            #[cfg(feature = "kenney_input_prompts")]
            Pack::Kenney => kenney_input_prompts::tokenize_dir::FILE_PATHS.get(file_index)?,

            #[cfg(feature = "xelu_free_controller_key_prompts")]
            Pack::Xelu =>
                xelu_free_controller_key_prompts::tokenize_dir::FILE_PATHS.get(file_index)?,
        }
    ))
}

pub fn copy_assets() -> Result<(), CopyAssetsError> {
    #[cfg(feature = "kenney_input_prompts")]
    kenney_input_prompts::copy_assets("assets/bevy_input_prompts")?;
    #[cfg(feature = "xelu_free_controller_key_prompts")]
    xelu_free_controller_key_prompts::copy_assets("assets/bevy_input_prompts")?;
    Ok(())
}

pub trait FileConstraints: Sized {
    type Constraints<'c>: tokenize_dir::ToConstraints;
    fn file_constriants<'c>(self, pack: Pack) -> Self::Constraints<'c>;
    // we also have this so the suer doesn't have to do .file_path_extra::<&[&[usize]]>(pack, &[])
    // just to leave it without any extra contraints
    // because the compiler won't apply a relevant type to &[]
    fn file_path(
        self,
        pack: Pack,
    ) -> Option<String> {
        first_file_path(pack, self.file_constriants(pack))
    }
    fn file_path_extra(
        self,
        pack: Pack,
        extra_contraints: impl tokenize_dir::ToConstraints,
    ) -> Option<String> {
        first_file_path(pack, (self.file_constriants(pack), extra_contraints))
    }
}
