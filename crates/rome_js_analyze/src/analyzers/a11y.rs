//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_access_key;
mod no_auto_focus;
mod no_blank_target;
mod no_distracting_elements;
mod no_header_scope;
mod no_redundant_alt;
mod no_svg_without_title;
mod use_alt_text;
mod use_anchor_content;
mod use_html_lang;
mod use_iframe_title;
mod use_key_with_click_events;
mod use_key_with_mouse_events;
mod use_media_caption;
mod use_valid_anchor;
declare_group! { pub (crate) A11y { name : "a11y" , rules : [self :: no_access_key :: NoAccessKey , self :: no_auto_focus :: NoAutoFocus , self :: no_blank_target :: NoBlankTarget , self :: no_distracting_elements :: NoDistractingElements , self :: no_header_scope :: NoHeaderScope , self :: no_redundant_alt :: NoRedundantAlt , self :: no_svg_without_title :: NoSvgWithoutTitle , self :: use_alt_text :: UseAltText , self :: use_anchor_content :: UseAnchorContent , self :: use_html_lang :: UseHtmlLang , self :: use_iframe_title :: UseIframeTitle , self :: use_key_with_click_events :: UseKeyWithClickEvents , self :: use_key_with_mouse_events :: UseKeyWithMouseEvents , self :: use_media_caption :: UseMediaCaption , self :: use_valid_anchor :: UseValidAnchor ,] } }
