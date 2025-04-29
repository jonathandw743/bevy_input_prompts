#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
pub mod directory_representation {
    pub const FILE_PATHS: [&'static str; 241] = [
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/mouse.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_arrows.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/mouse_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_backspace.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_tab.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_capslock.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_enter.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_plus.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_shift.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_space.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_backspace_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_tab_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_shift_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_space_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_capslock_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_enter_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_plus_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_0.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_1.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_2.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_3.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_4.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_5.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_6.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_7.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_8.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_9.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_a.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_alt.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_any.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_apostrophe.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_asterisk.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_b.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_c.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_caret.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_colon.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_comma.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_command.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_ctrl.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_d.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_delete.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_e.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_end.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_equals.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_escape.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_exclamation.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_f.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_f1.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_f10.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_f11.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_f12.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_f2.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_f3.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_f4.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_f5.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_f6.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_f7.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_f8.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_f9.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_function.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_g.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_h.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_home.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_i.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_insert.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_j.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_k.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_l.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_m.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_minus.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_n.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_numlock.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_o.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_option.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_p.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_period.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_printscreen.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_q.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_question.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_quote.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_r.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_return.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_s.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_semicolon.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_t.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_tilde.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_u.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_v.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_w.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_win.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_x.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_y.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_z.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/mouse_scroll.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_0_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_1_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_2_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_3_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_4_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_5_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_6_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_7_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_8_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_9_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_a_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_alt_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_any_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_apostrophe_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_asterisk_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_b_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_c_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_caret_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_colon_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_comma_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_command_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_ctrl_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_d_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_delete_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_e_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_end_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_equals_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_escape_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_exclamation_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_f10_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_f11_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_f12_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_f1_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_f2_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_f3_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_f4_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_f5_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_f6_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_f7_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_f8_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_f9_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_f_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_function_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_g_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_h_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_home_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_i_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_insert_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_j_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_k_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_l_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_m_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_minus_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_n_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_numlock_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_o_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_option_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_p_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_period_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_printscreen_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_q_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_question_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_quote_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_r_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_return_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_s_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_semicolon_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_t_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_tilde_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_u_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_v_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_w_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_win_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_x_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_y_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_z_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_arrows_down.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_arrows_up.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/mouse_left.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/mouse_right.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/mouse_vertical.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_arrows_left.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_arrows_right.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_backspace_icon.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_tab_icon.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/mouse_scroll_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_arrows_vertical.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_arrow_down.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_arrow_up.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_arrows_down_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_arrows_up_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/mouse_left_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/mouse_right_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_capslock_icon.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_shift_icon.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_space_icon.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/mouse_horizontal.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_arrow_left.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_arrow_right.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_arrows_left_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_arrows_right_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_backspace_icon_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_tab_icon_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_arrows_horizontal.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_arrows_vertical_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_arrow_down_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_arrow_up_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_page_down.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_page_up.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_capslock_icon_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_space_icon_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_shift_icon_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_arrow_left_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_arrow_right_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_arrows_horizontal_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_page_up_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_page_down_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_numpad_enter.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_numpad_plus.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_bracket_close.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_bracket_greater.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_bracket_less.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_bracket_open.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_numpad_enter_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_numpad_plus_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/mouse_scroll_down.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/mouse_scroll_up.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_bracket_greater_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_bracket_open_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_bracket_close_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_bracket_less_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_slash_back.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_slash_forward.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/mouse_scroll_vertical.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/mouse_scroll_up_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/mouse_scroll_down_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_slash_back_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_slash_forward_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/mouse_scroll_vertical_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_tab_icon_alternative.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_backspace_icon_alternative.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_tab_icon_alternative_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_backspace_icon_alternative_outline.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_arrows_all.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/keyboard_arrows_none.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/mouse_move.png",
        "assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default/mouse_small.png",
    ];
    pub const _0: &[usize] = &[17, 94];
    pub const _1: &[usize] = &[18, 95];
    pub const _2: &[usize] = &[19, 96];
    pub const _3: &[usize] = &[20, 97];
    pub const _4: &[usize] = &[21, 98];
    pub const _5: &[usize] = &[22, 99];
    pub const _6: &[usize] = &[23, 100];
    pub const _7: &[usize] = &[24, 101];
    pub const _8: &[usize] = &[25, 102];
    pub const _9: &[usize] = &[26, 103];
    pub const _a: &[usize] = &[27, 104];
    pub const _all: &[usize] = &[237];
    pub const _alt: &[usize] = &[28, 105];
    pub const _alternative: &[usize] = &[233, 234, 235, 236];
    pub const _any: &[usize] = &[29, 106];
    pub const _apostrophe: &[usize] = &[30, 107];
    pub const _arrow: &[usize] = &[181, 182, 191, 192, 199, 200, 206, 207];
    pub const _arrows: &[usize] = &[
        1,
        170,
        171,
        175,
        176,
        180,
        183,
        184,
        193,
        194,
        197,
        198,
        208,
        237,
        238,
    ];
    pub const _asterisk: &[usize] = &[31, 108];
    pub const _b: &[usize] = &[32, 109];
    pub const _back: &[usize] = &[225, 230];
    pub const _backspace: &[usize] = &[3, 10, 177, 195, 234, 236];
    pub const _bracket: &[usize] = &[213, 214, 215, 216, 221, 222, 223, 224];
    pub const _c: &[usize] = &[33, 110];
    pub const _capslock: &[usize] = &[5, 14, 187, 203];
    pub const _caret: &[usize] = &[34, 111];
    pub const _close: &[usize] = &[213, 223];
    pub const _colon: &[usize] = &[35, 112];
    pub const _comma: &[usize] = &[36, 113];
    pub const _command: &[usize] = &[37, 114];
    pub const _ctrl: &[usize] = &[38, 115];
    pub const _d: &[usize] = &[39, 116];
    pub const _delete: &[usize] = &[40, 117];
    pub const _down: &[usize] = &[170, 181, 183, 199, 201, 210, 219, 229];
    pub const _e: &[usize] = &[41, 118];
    pub const _end: &[usize] = &[42, 119];
    pub const _enter: &[usize] = &[6, 15, 211, 217];
    pub const _equals: &[usize] = &[43, 120];
    pub const _escape: &[usize] = &[44, 121];
    pub const _exclamation: &[usize] = &[45, 122];
    pub const _f: &[usize] = &[46, 135];
    pub const _f1: &[usize] = &[47, 126];
    pub const _f10: &[usize] = &[48, 123];
    pub const _f11: &[usize] = &[49, 124];
    pub const _f12: &[usize] = &[50, 125];
    pub const _f2: &[usize] = &[51, 127];
    pub const _f3: &[usize] = &[52, 128];
    pub const _f4: &[usize] = &[53, 129];
    pub const _f5: &[usize] = &[54, 130];
    pub const _f6: &[usize] = &[55, 131];
    pub const _f7: &[usize] = &[56, 132];
    pub const _f8: &[usize] = &[57, 133];
    pub const _f9: &[usize] = &[58, 134];
    pub const _forward: &[usize] = &[226, 231];
    pub const _function: &[usize] = &[59, 136];
    pub const _g: &[usize] = &[60, 137];
    pub const _greater: &[usize] = &[214, 221];
    pub const _h: &[usize] = &[61, 138];
    pub const _home: &[usize] = &[62, 139];
    pub const _horizontal: &[usize] = &[190, 197, 208];
    pub const _i: &[usize] = &[63, 140];
    pub const _icon: &[usize] = &[
        177,
        178,
        187,
        188,
        189,
        195,
        196,
        203,
        204,
        205,
        233,
        234,
        235,
        236,
    ];
    pub const _insert: &[usize] = &[64, 141];
    pub const _j: &[usize] = &[65, 142];
    pub const _k: &[usize] = &[66, 143];
    pub const _keyboard: &[usize] = &[
        1,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
        14,
        15,
        16,
        17,
        18,
        19,
        20,
        21,
        22,
        23,
        24,
        25,
        26,
        27,
        28,
        29,
        30,
        31,
        32,
        33,
        34,
        35,
        36,
        37,
        38,
        39,
        40,
        41,
        42,
        43,
        44,
        45,
        46,
        47,
        48,
        49,
        50,
        51,
        52,
        53,
        54,
        55,
        56,
        57,
        58,
        59,
        60,
        61,
        62,
        63,
        64,
        65,
        66,
        67,
        68,
        69,
        70,
        71,
        72,
        73,
        74,
        75,
        76,
        77,
        78,
        79,
        80,
        81,
        82,
        83,
        84,
        85,
        86,
        87,
        88,
        89,
        90,
        91,
        92,
        94,
        95,
        96,
        97,
        98,
        99,
        100,
        101,
        102,
        103,
        104,
        105,
        106,
        107,
        108,
        109,
        110,
        111,
        112,
        113,
        114,
        115,
        116,
        117,
        118,
        119,
        120,
        121,
        122,
        123,
        124,
        125,
        126,
        127,
        128,
        129,
        130,
        131,
        132,
        133,
        134,
        135,
        136,
        137,
        138,
        139,
        140,
        141,
        142,
        143,
        144,
        145,
        146,
        147,
        148,
        149,
        150,
        151,
        152,
        153,
        154,
        155,
        156,
        157,
        158,
        159,
        160,
        161,
        162,
        163,
        164,
        165,
        166,
        167,
        168,
        169,
        170,
        171,
        175,
        176,
        177,
        178,
        180,
        181,
        182,
        183,
        184,
        187,
        188,
        189,
        191,
        192,
        193,
        194,
        195,
        196,
        197,
        198,
        199,
        200,
        201,
        202,
        203,
        204,
        205,
        206,
        207,
        208,
        209,
        210,
        211,
        212,
        213,
        214,
        215,
        216,
        217,
        218,
        221,
        222,
        223,
        224,
        225,
        226,
        230,
        231,
        233,
        234,
        235,
        236,
        237,
        238,
    ];
    pub const _l: &[usize] = &[67, 144];
    pub const _left: &[usize] = &[172, 175, 185, 191, 193, 206];
    pub const _less: &[usize] = &[215, 224];
    pub const _m: &[usize] = &[68, 145];
    pub const _minus: &[usize] = &[69, 146];
    pub const _mouse: &[usize] = &[
        0,
        2,
        93,
        172,
        173,
        174,
        179,
        185,
        186,
        190,
        219,
        220,
        227,
        228,
        229,
        232,
        239,
        240,
    ];
    pub const _move: &[usize] = &[239];
    pub const _n: &[usize] = &[70, 147];
    pub const _none: &[usize] = &[238];
    pub const _numlock: &[usize] = &[71, 148];
    pub const _numpad: &[usize] = &[211, 212, 217, 218];
    pub const _o: &[usize] = &[72, 149];
    pub const _open: &[usize] = &[216, 222];
    pub const _option: &[usize] = &[73, 150];
    pub const _outline: &[usize] = &[
        2,
        10,
        11,
        12,
        13,
        14,
        15,
        16,
        94,
        95,
        96,
        97,
        98,
        99,
        100,
        101,
        102,
        103,
        104,
        105,
        106,
        107,
        108,
        109,
        110,
        111,
        112,
        113,
        114,
        115,
        116,
        117,
        118,
        119,
        120,
        121,
        122,
        123,
        124,
        125,
        126,
        127,
        128,
        129,
        130,
        131,
        132,
        133,
        134,
        135,
        136,
        137,
        138,
        139,
        140,
        141,
        142,
        143,
        144,
        145,
        146,
        147,
        148,
        149,
        150,
        151,
        152,
        153,
        154,
        155,
        156,
        157,
        158,
        159,
        160,
        161,
        162,
        163,
        164,
        165,
        166,
        167,
        168,
        169,
        179,
        183,
        184,
        185,
        186,
        193,
        194,
        195,
        196,
        198,
        199,
        200,
        203,
        204,
        205,
        206,
        207,
        208,
        209,
        210,
        217,
        218,
        221,
        222,
        223,
        224,
        228,
        229,
        230,
        231,
        232,
        235,
        236,
    ];
    pub const _p: &[usize] = &[74, 151];
    pub const _page: &[usize] = &[201, 202, 209, 210];
    pub const _period: &[usize] = &[75, 152];
    pub const _plus: &[usize] = &[7, 16, 212, 218];
    pub const _printscreen: &[usize] = &[76, 153];
    pub const _q: &[usize] = &[77, 154];
    pub const _question: &[usize] = &[78, 155];
    pub const _quote: &[usize] = &[79, 156];
    pub const _r: &[usize] = &[80, 157];
    pub const _return: &[usize] = &[81, 158];
    pub const _right: &[usize] = &[173, 176, 186, 192, 194, 207];
    pub const _s: &[usize] = &[82, 159];
    pub const _scroll: &[usize] = &[93, 179, 219, 220, 227, 228, 229, 232];
    pub const _semicolon: &[usize] = &[83, 160];
    pub const _shift: &[usize] = &[8, 12, 188, 205];
    pub const _slash: &[usize] = &[225, 226, 230, 231];
    pub const _small: &[usize] = &[240];
    pub const _space: &[usize] = &[9, 13, 189, 204];
    pub const _t: &[usize] = &[84, 161];
    pub const _tab: &[usize] = &[4, 11, 178, 196, 233, 235];
    pub const _tilde: &[usize] = &[85, 162];
    pub const _u: &[usize] = &[86, 163];
    pub const _up: &[usize] = &[171, 182, 184, 200, 202, 209, 220, 228];
    pub const _v: &[usize] = &[87, 164];
    pub const _vertical: &[usize] = &[174, 180, 198, 227, 232];
    pub const _w: &[usize] = &[88, 165];
    pub const _win: &[usize] = &[89, 166];
    pub const _x: &[usize] = &[90, 167];
    pub const _y: &[usize] = &[91, 168];
    pub const _z: &[usize] = &[92, 169];
}
/// Assumes each slice is sorted
/// slower for debug builds
pub fn file_index(tokens_associated_files: &[&[usize]]) -> usize {
    let mut i = 0;
    while i < tokens_associated_files.len() && tokens_associated_files[i].is_empty() {
        i += 1;
    }
    if i >= tokens_associated_files.len() {
        return 0;
    }
    let mut possible_files = Vec::from(tokens_associated_files[i]);
    for k in 1..tokens_associated_files.len() {
        let mut new_possible_files = Vec::with_capacity(possible_files.len());
        let mut i = 0;
        let mut j = 0;
        while i < possible_files.len() && j < tokens_associated_files[k].len() {
            if possible_files[i] == tokens_associated_files[k][j] {
                new_possible_files.push(possible_files[i]);
                i += 1;
                j += 1;
            } else if possible_files[i] < tokens_associated_files[k][j] {
                i += 1;
            } else if possible_files[i] > tokens_associated_files[k][j] {
                j += 1;
            }
        }
        if !new_possible_files.is_empty() {
            possible_files = new_possible_files;
        }
    }
    possible_files[0]
}
