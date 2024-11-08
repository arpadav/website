// static & technicals
// --------------------------------------------
// bools
var rotated = false;
var click_enable = true;
var tabs_first_invisible = true;
var first_load = true;

// transforms, do not edit
var transforms = ['-moz-transform', '-webkit-transform', '-o-transform', '-ms-transform', 'transform'];

// resize timeout
var resize_timeout;

// current and new font-sizes for main name
var fs_cur;
var fs_new;
// --------------------------------------------

// adjustables
// --------------------------------------------
// animation length
var anim_len = 0.8;

// scale before/after click
var name_scale_in = [1];
var name_scale_out = [1];
var tabs_scale_in = [1];
var tabs_scale_out = [1];

// scale of various segments
var h2_scale = 0.55;
var h3_scale = 0.425;
var p_scale = 0.275;

// name rotation specifications
var name_anim_begin_rot = ["0deg"];
var name_anim_end_rot = ["-90deg"];
var name_anim_begin_pos = ["0%", "0%"];
var name_anim_end_pos = ["-105%", "0%"];
var name_pivot = ["center"];

// tabs rotation specifications
var tabs_anim_begin_rot = ["90deg"];
var tabs_anim_end_rot = ["0deg"];
var tabs_anim_begin_pos = ["85%", "0%"];
var tabs_anim_end_pos = ["10%", "0%"];
var tabs_pivot = ["left center"];
// --------------------------------------------

init();

function init() {
    // resize on load
    resize_divs();

    // name clicking listener
    let name_div = document.getElementById('name');
    name_div.style.cursor = "pointer";
    name_div.addEventListener("click", name_click);

    // tabs head clicking listener
    let tabs_title = document.getElementsByClassName('head');
    for (let i = 0; i < tabs_title.length; i++) {
        tabs_title[i].style.cursor = "pointer";
        tabs_title[i].addEventListener("click", toggle_collapse);
    }

    // add listener for resizing the window
    window.addEventListener("resize", function () {
        clearTimeout(resize_timeout);
        resize_timeout = setTimeout(resize_divs, 100);
    });

    // key event triggers name_click
    // keys: enter, space, numbers, letters
    document.body.onkeyup = function (e) {
        // if(e.keyCode == 13 || e.keyCode == 32) name_click();
        if (e.key == " " || e.key == "Enter") name_click();
    }

    // testing ---------------------------------------------
    // // debug
    // document.body.onkeyup = function(e) {
    //   if(e.keyCode == 65) {
    //     smooth_scrollTo(document.body, 0, 100, function(){
    //       console.log("huh1");
    //     });
    //     // document.body.scrollTop = 0;
    //     console.log("pressed A"); }
    // }
    // testing ---------------------------------------------
}


// =======================================================================
// =                           VISUALS                                   =
// =======================================================================
// toggles visibility of 'tabs'
function set_tab_visibility(visible) {
    let tabs = document.getElementsByClassName("tab");
    if (visible) for (let i = 0; i < tabs.length; i++) tabs[i].style.visibility = "visible";
    else for (let i = 0; i < tabs.length; i++) tabs[i].style.visibility = "hidden";
}

// single collapsing body, used in anchors
function single_tab_open(id) {
    let headers = document.getElementById(id);
    for (let i = 0; i < headers.children.length; i++) {
        if (headers.children[i].className == 'body') {
            headers.children[i].style.maxHeight = headers.children[i].scrollHeight + "px";
        }
    }
}

// reset the height of tabs
// called upon resizing window
function reset_tabs_height() {
    let bodies = document.getElementsByClassName('body');
    for (let i = 0; i < bodies.length; i++) if (bodies[i].style.maxHeight) bodies[i].style.maxHeight = bodies[i].scrollHeight + "px";
}

// collapse bodies upon clicking title or name
// links do not cause collapse animations
function toggle_collapse(force_collapse) {
    let bodies = document.getElementsByClassName('body');
    if (force_collapse == true) for (let i = 0; i < bodies.length; i++) bodies[i].style.maxHeight = null;
    else {
        // "this" refers to click event
        if (this.children[0].tagName != 'A') {
            this.classList.toggle("active");
            let content = this.nextElementSibling;
            for (let i = 0; i < bodies.length; i++) if (bodies[i] != content) bodies[i].style.maxHeight = null;
            if (content.style.maxHeight) {
                content.style.maxHeight = null;
                window.history.replaceState(' ', '', ' ');
            }
            else {
                content.style.maxHeight = content.scrollHeight + "px";
                window.history.replaceState("#" + this.parentNode.id, '', "#" + this.parentNode.id);
            }
        }
    }
}

// sets the overflow of the Y direction to hidden/auto
function toggle_scrollbar() {
    let body = document.querySelector('body');
    if (body.style.overflowY == 'hidden' || body.style.overflowY == '') body.style.overflowY = "scroll";
    else body.style.overflowY = "hidden";
}

// resize elements (divs) based on load and window resize
// finds the current fontsize and new fontsize, then
// calls resize name and reset tabs height to resize
function resize_divs() {
    let main_div = document.getElementsByClassName('main');
    // fs_cur = fs_cur ? Number(main_div[0].style.fontSize.match(/(.+?)(?=px)/g)) : 32;
    fs_cur = Number(main_div[0].style.fontSize.match(/(.+?)(?=px)/g));
    fs_new = Math.round(window.innerWidth / 32);

    resize_name();
    reset_tabs_height();
}

// recursive resizing of name
function resize_name() {
    let container_div = document.getElementById('home');
    let main_div = document.getElementsByClassName('main');
    let timeout = 5;

    if (fs_new != fs_cur) {
        if (fs_new > fs_cur) main_div[0].style.fontSize = `${++fs_cur}px`;
        else main_div[0].style.fontSize = `${--fs_cur}px`;
        container_div.style.width = `${14 * fs_cur}px`;
        setTimeout(resize_name, timeout);
    } else {
        // change scale of transform, depending on window height
        name_scale_in[0] = window.innerHeight / Number(container_div.style.width.match(/(.+?)(?=px)/g));
        // update the css animation specs
        update_css_anim();
        // update other font sizes
        let headers = document.getElementsByClassName('head');
        let bodies = document.getElementsByClassName('body');
        for (let i = 0; i < headers.length; i++) recursive_resize_font(headers[i]);
        for (let i = 0; i < bodies.length; i++) recursive_resize_font(bodies[i]);
        if (window.location.hash && first_load) {
            first_load = false;
            name_click();
            setTimeout(call_anchors, anim_len * 1000);
        }
    }
}

// change the font size of other elements, according to the main name
function recursive_resize_font(elements) {
    let root_tag_exceptions = ['P'];
    for (let c = 0; c < elements.children.length; c++) {
        if (elements.children[c].children.length && !root_tag_exceptions.includes(elements.children[c].tagName)) recursive_resize_font(elements.children[c]);
        else {
            switch (elements.children[c].tagName) {
                case 'H2':
                    elements.children[c].style.fontSize = `${h2_scale * fs_new * name_scale_in[0]}px`;
                    break;
                case 'H3':
                    elements.children[c].style.fontSize = `${h3_scale * fs_new * name_scale_in[0]}px`;
                    break;
                case 'P':
                    elements.children[c].style.fontSize = `${p_scale * fs_new * name_scale_in[0]}px`;
                    break;
                case 'LI':
                    elements.children[c].style.fontSize = `${p_scale * fs_new * name_scale_in[0]}px`;
                    break;
            }
        }
    }
}

// =======================================================================
// =                    PARTIAL TECHNICAL/ANIMATION                      =
// =======================================================================
// do a lil animation thang when anchors are called from other pages
function call_anchors() {
    // REGEX LOOKBEHIND BREAKS MOBILE
    // REGEX LOOKBEHIND BREAKS MOBILE
    // REGEX LOOKBEHIND BREAKS MOBILE
    // let anchor = window.location.hash.match(/(?<=#)(.*$)/g)[0];
    let anchor = window.location.hash.match(/(?:#)(.*$)/g)[0].substring(1);
    single_tab_open(anchor);
}

// rotate name on click, disable clicking for animation length
// change visibility of other elements depending on status
function name_click() {
    let name_div = document.getElementById('name');
    let tabs = document.getElementsByClassName('tab');
    // check if clicking on 'name' is enabled
    if (click_enable) {
        // disable for length of animation & start animation
        click_enable = false;
        if (rotated) {
            // scroll to the top, otherwise reset animation looks bad
            smooth_scrollTo(document.body, 0, 3, function () {
                window.history.replaceState(' ', '', ' ');
                // animation for making name centered (horz)
                name_div.style.animation = `name_rotate_horz ${anim_len}s forwards`;
                name_div.style.webkitAnimation = `name_rotate_horz ${anim_len}s forwards`;
                for (let i = 0; i < tabs.length; i++) {
                    tabs[i].style.animation = `tabs_rotate_horz ${anim_len}s forwards fade_out ease ${anim_len}s forwards`;
                    tabs[i].style.webkitAnimation = `tabs_rotate_horz ${anim_len}s forwards, fade_out ease ${anim_len}s forwards`;
                    tabs[i].style.pointerEvents = "none";
                }
                toggle_scrollbar();
                rotated = false;
            });
        } else {
            if (tabs_first_invisible) {
                set_tab_visibility(true);
                tabs_first_invisible = false;
            }
            // collapse all elements before displaying them again
            toggle_collapse(true);
            // animation for making name rotated (vert)
            name_div.style.animation = `name_rotate_vert ${anim_len}s forwards`;
            name_div.style.webkitAnimation = `name_rotate_vert ${anim_len}s forwards`;
            for (let i = 0; i < tabs.length; i++) {
                tabs[i].style.animation = `tabs_rotate_vert ${anim_len}s forwards, fade_in ease ${anim_len}s forwards`;
                tabs[i].style.webkitAnimation = `tabs_rotate_vert ${anim_len}s forwards, fade_in ease ${anim_len}s forwards`;
                tabs[i].style.pointerEvents = "auto";
            }
            rotated = true;
        }
        // disable clicking for length of animation
        setTimeout(function () { click_enable = true; if (rotated) toggle_scrollbar(); }, anim_len * 1000);
    }
}

// initialize css animations with transforms and rules
function update_css_anim() {
    // NAME SIDEWAYS ROTATION (VERT)
    let keyframes = find_keyframes_rule("name_rotate_vert");
    keyframes.deleteRule("0%");
    keyframes.deleteRule("100%");
    keyframes.appendRule(create_css_transform_rule(0,
        "translate", true, name_anim_begin_pos,
        "rotate", true, name_anim_begin_rot,
        "transform-origin", false, name_pivot,
        "scale", true, name_scale_out));
    keyframes.appendRule(create_css_transform_rule(100,
        "translate", true, name_anim_end_pos,
        "rotate", true, name_anim_end_rot,
        "transform-origin", false, name_pivot,
        "scale", true, name_scale_in));

    // NAME CENTERED ROTATION (HORZ)
    keyframes = find_keyframes_rule("name_rotate_horz");
    keyframes.deleteRule("0%");
    keyframes.deleteRule("100%");
    keyframes.appendRule(create_css_transform_rule(0,
        "translate", true, name_anim_end_pos,
        "rotate", true, name_anim_end_rot,
        "transform-origin", false, name_pivot,
        "scale", true, name_scale_in));
    keyframes.appendRule(create_css_transform_rule(100,
        "translate", true, name_anim_begin_pos,
        "rotate", true, name_anim_begin_rot,
        "transform-origin", false, name_pivot,
        "scale", true, name_scale_out));

    // TABS DISPLAYING ROTATION (VERT)
    keyframes = find_keyframes_rule("tabs_rotate_vert");
    keyframes.deleteRule("0%");
    keyframes.deleteRule("100%");
    keyframes.appendRule(create_css_transform_rule(0,
        "translate", true, tabs_anim_begin_pos,
        "rotate", true, tabs_anim_begin_rot,
        "transform-origin", false, tabs_pivot,
        "scale", true, tabs_scale_out));
    keyframes.appendRule(create_css_transform_rule(100,
        "translate", true, tabs_anim_end_pos,
        "rotate", true, tabs_anim_end_rot,
        "transform-origin", false, tabs_pivot,
        "scale", true, tabs_scale_in));

    // TABS DISAPPEARING ROTATION (HORZ)
    keyframes = find_keyframes_rule("tabs_rotate_horz");
    keyframes.deleteRule("0%");
    keyframes.deleteRule("100%");
    keyframes.appendRule(create_css_transform_rule(0,
        "translate", true, tabs_anim_end_pos,
        "rotate", true, tabs_anim_end_rot,
        "transform-origin", false, tabs_pivot,
        "scale", true, tabs_scale_in));
    keyframes.appendRule(create_css_transform_rule(100,
        "translate", true, tabs_anim_begin_pos,
        "rotate", true, tabs_anim_begin_rot,
        "transform-origin", false, tabs_pivot,
        "scale", true, tabs_scale_out));
}

// mainly taken from: https://gist.github.com/andjosh/6764939
// with adjustments
// =======================================================================
// smooth scrolls to some top position
// rather than duration, speed is an input. all will scroll at the same
// speed, so variable duration depending on what is starting scroll position
function smooth_scrollTo(element, to, speed, callback) {
    let start = element.scrollTop,
        delta = to - start,
        cur_time = 0,
        inc = 5,
        dur = Math.round(element.scrollTop / speed);

    var animateScroll = function () {
        cur_time += inc;
        let val = easeInOutQuad(cur_time, start, delta, dur);
        element.scrollTop = Math.round(val);

        if (cur_time < dur) setTimeout(animateScroll, inc);
        else return callback();
    };
    animateScroll();
}

/**
 * Returns a value between 'b' and 'c' (inclusive) at time 't',
 * based on the value of 'd'. The curve is an in-out quad, meaning
 * that the curve is faster at the beginning and end, and slower
 * in the middle.
 *
 * @param {number} t - time, should be between 0 and 'd'
 * @param {number} b - starting value
 * @param {number} c - ending value
 * @param {number} d - duration, should be greater than 0
 * @returns {number} value between 'b' and 'c' at time 't'
 */
function easeInOutQuad(t, b, c, d) {
    t /= (d / 2);
    if (t < 1) return (c / 2) * (t * t) + b;
    t--;
    return -(c / 2) * (t * (t - 2) - 1) + b;
};
// =======================================================================


// =======================================================================
// =                           FULL TECHNICAL                            =
// =======================================================================
// find any CSS property from text
// STRING:
// TYPE.cssText.match(/(?<=PROP:)(\s*?)(.*?)(?=\"\s*?\;)/g)[0].match(/(?<=\")(.*?)$/g)[0];
// TYPE.cssText.match(RegExp(`(?<=${PROP}:)(\\s*?)(.*?)(?=\\"\\s*?\\;)`, 'g'))[0].match(/(?<=\")(.*?)$/g)[0];
// RAW:
// TYPE.cssText.match(/(?<=PROP:)(\s*?)(.*?)(?=\s*?\;)/g)[0].trim();
// TYPE.cssText.match(RegExp(`(?<=${PROP}:)(\\s*?)(.*?)(?=\\s*?\\;)`, 'g'))[0].trim();

// REGEX LOOKBEHIND BREAKS MOBILE
// REGEX LOOKBEHIND BREAKS MOBILE
// REGEX LOOKBEHIND BREAKS MOBILE
/*
function get_css_property(type, prop, str) {
  if (str) {
    try { return type.cssText.match(RegExp(`(?<=${prop}:)(\\s*?)(.*?)(?=\\"\\s*?\\;)`, 'g'))[0].match(/(?<=\")(.*?)$/g)[0]; }
    catch(err) { return null; }
  } else {
    try { return type.cssText.match(RegExp(`(?<=${prop}:)(\\s*?)(.*?)(?=\\s*?\\;)`, 'g'))[0].trim(); }
    catch(err) { return null; }
  }
}
*/

// used in adding a rule for transformations
// used in initialization, and (somewhat) beyond
// returns a string (rule) to be added to the CSS rules
// input format: (0-100 percent, string func1, bool trans_dep1, array params1, string func2, ...
function create_css_transform_rule() {
    // save arguments
    let this_args = arguments;

    // get percent, begin 'rule' string to return
    let percent = this_args[0];
    let rule = percent + "% { ";

    // initialize arrays for transform dependent and independent functions
    trans_dep_idx = [];
    trans_indep_idx = [];

    // populate said arrays
    for (let i = 2; i < this_args.length; i += 3) {
        if (this_args[i]) trans_dep_idx.push(i - 1); else trans_indep_idx.push(i - 1);
    }

    // append 'rule' string with transform functions first
    transforms.forEach(function (t_item) {
        let more_trans_dep = 0;
        trans_dep_idx.forEach(function (idx) {
            let function_name = this_args[idx];
            let args = this_args[idx + 2];

            rule = (!more_trans_dep) ? rule + t_item + ": " : rule + " ";
            more_trans_dep = 1;
            rule = rule + function_name + "(";

            args.forEach(function (item, idx) {
                rule = (idx != args.length - 1) ? rule + item + ", " : rule + item + ")";
            });
        });
        rule = rule + "; ";
    });

    // then, append independent functions/properties
    trans_indep_idx.forEach(function (idx) {
        let function_name = this_args[idx];
        let args = this_args[idx + 2];

        rule = rule + function_name + ": ";
        args.forEach(function (item, idx) {
            rule = (idx != args.length - 1) ? rule + item + ", " : rule + item;
        });
        rule = rule + "; ";
    });

    return rule + " }";
}

// uses CSS DOM to look through CSS rules
// used in finding CSS styles
function find_selectortext_rule(rule) {
    let ss = document.styleSheets;
    for (let i = 0; i < ss.length; ++i) {
        for (let j = 0; j < ss[i].cssRules.length; ++j) {
            if (ss[i].cssRules[j].selectorText == rule) {
                return ss[i].cssRules[j];
            }
        }
    }
    return null;
}

// uses CSS DOM to look through CSS rules
// used in initialization of keyframe animations
function find_keyframes_rule(rule) {
    let ss = document.styleSheets;
    for (let i = 0; i < ss.length; ++i) {
        for (let j = 0; j < ss[i].cssRules.length; ++j) {
            if (ss[i].cssRules[j].type == window.CSSRule.KEYFRAMES_RULE && ss[i].cssRules[j].name == rule) {
                return ss[i].cssRules[j];
            }
        }
    }
    return null;
}
