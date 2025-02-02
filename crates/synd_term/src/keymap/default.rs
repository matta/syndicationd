use crate::keymap::{macros::keymap, KeymapsConfig};

pub fn default() -> KeymapsConfig {
    let login = keymap!({
        "enter" => authenticate,
        "k" | "up" => move_up_authentication_provider,
        "j" | "down" => move_down_authentication_provider,
    });
    let tabs = keymap!({
        "tab" => move_right_tab_selection,
        "backtab" => move_left_tab_selection,
    });
    let entries = keymap!({
        "k" | "up" => move_up_entry,
        "j" | "down" => move_down_entry,
        "r" => reload_entries,
        "enter" => open_entry,
        "g" => {
           "g" => move_entry_first,
           "e" => move_entry_last,
        },
    });
    let subscription = keymap!({
        "a" => prompt_feed_subscription,
        "e" => prompt_feed_edition,
        "d" => prompt_feed_unsubscription,
        "k" | "up" => move_up_subscribed_feed,
        "j" | "down" => move_down_subscribed_feed,
        "r" => reload_subscription,
        "enter" => open_feed,
        "g" => {
            "g" => move_subscribed_feed_first,
            "e" => move_subscribed_feed_last,
        },
    });
    let filter = keymap!({
       "h" | "left" => move_filter_requirement_left,
       "l" | "right" => move_filter_requirement_right,
       "c" => activate_category_filtering,
       "/" => activate_search_filtering,
       "esc" => deactivate_filtering,
    });
    let unsubscribe_popup = keymap!({
        "h" | "left" => move_feed_unsubscription_popup_selection_left,
        "l" | "right" => move_feed_unsubscription_popup_selection_right,
        "enter" => select_feed_unsubscription_popup,
        "esc" => cancel_feed_unsubscription_popup,
    });
    let global = keymap!({
        "q" | "C-c" =>  quit ,
        "S-t" => rotate_theme,
    });

    KeymapsConfig {
        login,
        tabs,
        entries,
        subscription,
        filter,
        unsubscribe_popup,
        global,
    }
}
