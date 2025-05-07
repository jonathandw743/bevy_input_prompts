use fs_extra::error::Error;

pub mod gamepad_brand;

pub mod char;
pub mod gamepad_axis;
pub mod gamepad_button;
pub mod key;
pub mod key_code;
pub mod keyboard_input;

#[cfg(feature = "use_kenney_input_prompts")]
pub use kenney_input_prompts::tokenize_dir::_kenney_input_prompts_1_4 as kenney_tokenize;
#[cfg(feature = "use_xelu_free_controller_key_prompts")]
pub use xelu_free_controller_key_prompts::tokenize_dir::_Xelu_Free_Controller_Key_Prompts as xelu_tokenize;

#[derive(Clone, Copy)]
pub enum Pack {
    #[cfg(feature = "use_kenney_input_prompts")]
    Kenney,
    #[cfg(feature = "use_xelu_free_controller_key_prompts")]
    Xelu,
}

pub fn first_file_path<T: tokenize_dir::ToIter>(pack: Pack, files: T) -> Option<String> {
    let file_index = *files.file_indices().get(0)?;
    Some(format!("bevy_input_prompts/{}", match pack {
        #[cfg(feature = "use_kenney_input_prompts")]
        Pack::Kenney => kenney_input_prompts::tokenize_dir::FILE_PATHS.get(file_index)?,

        #[cfg(feature = "use_xelu_free_controller_key_prompts")]
        Pack::Xelu => xelu_free_controller_key_prompts::tokenize_dir::FILE_PATHS.get(file_index)?,
    }))
}

pub fn copy_assets() -> Result<(), Error> {
    #[cfg(feature = "use_kenney_input_prompts")]
    kenney_input_prompts::copy_assets("assets/bevy_input_prompts")?;
    #[cfg(feature = "use_xelu_free_controller_key_prompts")]
    xelu_free_controller_key_prompts::copy_assets("assets/bevy_input_prompts")?;
    Ok(())
}

pub trait ToFile {
    type Options;

    fn file_indices<'a, 'b>(&self, pack: Pack, extra: Self::Options) -> Option<&'a [&'b [usize]]>;

    fn file_path(
        &self,
        pack: Pack,
        options: Self::Options,
        extra_contraints: &[&[usize]],
    ) -> Option<String> {
        first_file_path(pack, [self.file_indices(pack, options)?, extra_contraints])
    }
}

pub trait ToFileDefault {
    fn file_indices<'a, 'b>(&self, pack: Pack) -> Option<&'a [&'b [usize]]>;

    fn file_path(&self, pack: Pack, extra_constraints: &[&[usize]]) -> Option<String> {
        first_file_path(pack, [self.file_indices(pack)?, extra_constraints])
    }
}
