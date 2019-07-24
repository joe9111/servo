/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

<%namespace name="helpers" file="/helpers.mako.rs" />
<% from data import ALL_SIDES, maybe_moz_logical_alias %>
<% data.new_style_struct("Margin", inherited=False) %>

% for side in ALL_SIDES:
    <%
        spec = "https://drafts.csswg.org/css-box/#propdef-margin-%s" % side[0]
        if side[1]:
            spec = "https://drafts.csswg.org/css-logical-props/#propdef-margin-%s" % side[1]
    %>
    ${helpers.predefined_type(
        "margin-%s" % side[0],
        "LengthPercentageOrAuto",
        "computed::LengthPercentageOrAuto::zero()",
        alias=maybe_moz_logical_alias(product, side, "-moz-margin-%s"),
        allow_quirks="No" if side[1] else "Yes",
        animation_value_type="ComputedValue",
        logical=side[1],
        logical_group="margin",
        spec=spec,
        flags="GETCS_NEEDS_LAYOUT_FLUSH",
        allowed_in_page_rule=True,
        servo_restyle_damage="reflow"
    )}
% endfor

% for side in ALL_SIDES:
    ${helpers.predefined_type(
        "scroll-margin-%s" % side[0],
        "Length",
        "computed::Length::zero()",
        products="gecko",
        gecko_pref="layout.css.scroll-snap-v1.enabled",
        logical=side[1],
        logical_group="scroll-margin",
        spec="https://drafts.csswg.org/css-scroll-snap-1/#propdef-scroll-margin-%s" % side[0],
        animation_value_type="ComputedValue",
    )}
% endfor
