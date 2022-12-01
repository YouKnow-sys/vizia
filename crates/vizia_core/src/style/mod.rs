use morphorm::{LayoutType, PositionType, Units};
use std::collections::{HashMap, HashSet};
use vizia_id::GenerationalId;
use vizia_style::{CssRule, Transform, Transition};

use cssparser::{Parser, ParserInput};

use crate::prelude::*;

pub use vizia_style::{
    BorderCornerShape, Color, Display, Length, LengthOrPercentage, LengthValue, Opacity, Overflow,
    ParserOptions, Property, SelectorList, Selectors, StyleSheet, Visibility,
};

// mod color;
// pub use color::Color;

// mod units;
// pub use units::*;

mod rule;
pub use rule::Rule;

// mod display;
// pub use display::*;

mod transform;
pub use transform::*;

// mod parser;
// pub use parser::*;

// mod style_rule;
// pub(crate) use style_rule::StyleRule;

mod selector;
pub use selector::*;

// mod specificity;
// use specificity::*;

// mod property;
// pub use property::*;

mod gradient;
pub use gradient::*;

// mod shadow;
// use shadow::*;

// mod prop;
// pub use prop::*;

use crate::animation::{AnimationState, Interpolator};
use crate::storage::animatable_set::AnimatableSet;
use crate::storage::style_set::StyleSet;
use crate::text::Selection;
use bitflags::bitflags;
use vizia_id::IdManager;
use vizia_storage::SparseSet;

bitflags! {
    /// Describes the capabilities of a view with respect to user interaction.
    ///
    /// This type is part of the prelude.
    pub struct Abilities: u8 {
        const HOVERABLE = 1;
        const FOCUSABLE = 1 << 1;
        const CHECKABLE = 1 << 2;
        const SELECTABLE = 1 << 3;
        /// The element should be focusable in sequential keyboard navigation -
        /// allowing the equivilant of a negative tabindex in html.
        const NAVIGABLE = 1 << 4;
    }
}

impl Default for Abilities {
    fn default() -> Abilities {
        Abilities::HOVERABLE
    }
}

/// Stores the style properties of all entities in the application.
#[derive(Default)]
pub struct Style {
    pub(crate) rule_manager: IdManager<Rule>,

    /// Creates and destroys animation ids
    pub(crate) animation_manager: IdManager<Animation>,

    // pub(crate) rules: Vec<StyleRule>,
    pub selectors: HashMap<Rule, (u32, SelectorList<Selectors>)>,
    pub transitions: HashMap<Rule, Animation>,

    pub default_font: String,

    pub elements: SparseSet<String>,
    pub ids: SparseSet<String>,
    pub classes: SparseSet<HashSet<String>>,
    pub pseudo_classes: SparseSet<PseudoClassFlags>,
    pub disabled: StyleSet<bool>,
    pub abilities: SparseSet<Abilities>,

    // Display
    pub display: StyleSet<Display>,

    // Visibility
    pub visibility: StyleSet<Visibility>,

    // Opacity
    pub opacity: AnimatableSet<Opacity>,

    // Z Order
    pub z_index: StyleSet<i32>,

    // Clipping
    pub clip_widget: SparseSet<Entity>,

    // Transform
    pub transform: AnimatableSet<Vec<Transform>>,
    // pub computed_transform: AnimatableSet<Transform2D>,
    // pub rotate: AnimatableSet<f32>,
    // pub translate: AnimatableSet<(f32, f32)>,
    // pub scale: AnimatableSet<(f32, f32)>,
    pub overflow: StyleSet<Overflow>, // TODO
    //pub scroll: DenseStorage<Scroll>,     // TODO

    // Border
    pub border_width: AnimatableSet<LengthOrPercentage>,
    pub border_color: AnimatableSet<Color>,

    // Border Shape
    pub border_top_left_shape: StyleSet<BorderCornerShape>,
    pub border_top_right_shape: StyleSet<BorderCornerShape>,
    pub border_bottom_left_shape: StyleSet<BorderCornerShape>,
    pub border_bottom_right_shape: StyleSet<BorderCornerShape>,

    // Border Radius
    pub border_top_left_radius: AnimatableSet<LengthOrPercentage>,
    pub border_top_right_radius: AnimatableSet<LengthOrPercentage>,
    pub border_bottom_left_radius: AnimatableSet<LengthOrPercentage>,
    pub border_bottom_right_radius: AnimatableSet<LengthOrPercentage>,

    // Outline
    pub outline_width: AnimatableSet<LengthOrPercentage>,
    pub outline_color: AnimatableSet<Color>,
    pub outline_offset: AnimatableSet<LengthOrPercentage>,

    // Focus Order
    // pub focus_order: SparseSet<FocusOrder>,

    // Background
    pub background_color: AnimatableSet<Color>,
    pub background_image: StyleSet<String>,
    pub background_gradient: StyleSet<LinearGradient>,

    // Outer Shadow
    pub outer_shadow_h_offset: AnimatableSet<LengthOrPercentage>,
    pub outer_shadow_v_offset: AnimatableSet<LengthOrPercentage>,
    pub outer_shadow_blur: AnimatableSet<LengthOrPercentage>,
    pub outer_shadow_color: AnimatableSet<Color>,

    // Inner Shadow (TODO)
    pub inner_shadow_h_offset: AnimatableSet<LengthOrPercentage>,
    pub inner_shadow_v_offset: AnimatableSet<LengthOrPercentage>,
    pub inner_shadow_blur: AnimatableSet<LengthOrPercentage>,
    pub inner_shadow_color: AnimatableSet<Color>,

    // Text & Font
    pub text: StyleSet<String>,
    pub text_wrap: StyleSet<bool>,
    pub font: StyleSet<String>,
    pub font_color: AnimatableSet<Color>,
    pub font_size: AnimatableSet<f32>,
    pub text_selection: StyleSet<Selection>,
    pub caret_color: AnimatableSet<Color>,
    pub selection_color: AnimatableSet<Color>,

    // Image
    pub image: StyleSet<String>,

    pub tooltip: SparseSet<String>,

    // LAYOUT

    // Layout Type
    pub layout_type: StyleSet<LayoutType>,

    // Position Type
    pub position_type: StyleSet<PositionType>,

    // Spacing
    pub left: AnimatableSet<Units>,
    pub right: AnimatableSet<Units>,
    pub top: AnimatableSet<Units>,
    pub bottom: AnimatableSet<Units>,

    // Size
    pub width: AnimatableSet<Units>,
    pub height: AnimatableSet<Units>,

    // Size Constraints
    pub max_width: AnimatableSet<Units>,
    pub max_height: AnimatableSet<Units>,
    pub min_width: AnimatableSet<Units>,
    pub min_height: AnimatableSet<Units>,
    pub content_width: StyleSet<f32>,
    pub content_height: StyleSet<f32>,

    // Spacing Constraints
    pub min_left: AnimatableSet<Units>,
    pub max_left: AnimatableSet<Units>,
    pub min_right: AnimatableSet<Units>,
    pub max_right: AnimatableSet<Units>,
    pub min_top: AnimatableSet<Units>,
    pub max_top: AnimatableSet<Units>,
    pub min_bottom: AnimatableSet<Units>,
    pub max_bottom: AnimatableSet<Units>,

    // Grid
    pub grid_rows: StyleSet<Vec<Units>>,
    pub row_between: AnimatableSet<Units>,
    pub grid_cols: StyleSet<Vec<Units>>,
    pub col_between: AnimatableSet<Units>,

    pub row_index: StyleSet<usize>,
    pub col_index: StyleSet<usize>,
    pub row_span: StyleSet<usize>,
    pub col_span: StyleSet<usize>,

    // Child Spacing
    pub child_left: AnimatableSet<Units>,
    pub child_right: AnimatableSet<Units>,
    pub child_top: AnimatableSet<Units>,
    pub child_bottom: AnimatableSet<Units>,

    pub name: StyleSet<String>,

    pub cursor: StyleSet<CursorIcon>,

    pub needs_restyle: bool,
    pub needs_relayout: bool,
    pub needs_redraw: bool,

    pub dpi_factor: f64,
}

impl Style {
    // pub(crate) fn add_rule(&mut self, style_rule: StyleRule) {
    //     if !self.rules.contains(&style_rule) {
    //         self.rules.push(style_rule);
    //         self.rules.sort_by_key(|rule| rule.specificity());
    //         self.rules.reverse();
    //     }

    //     self.set_style_properties();
    // }

    pub fn remove_rules(&mut self) {
        // for rule in self.rules.iter() {
        //     self.rule_manager.destroy(rule.id);
        // }

        // for (_, animation) in self.transitions.iter() {
        //     self.animation_manager.destroy(*animation);
        // }
    }

    pub fn parse_theme(&mut self, stylesheet: &str) {
        if let Ok(theme) = StyleSheet::parse("test.css", stylesheet, ParserOptions::default()) {
            let rules = theme.rules.0;

            for rule in rules {
                match rule {
                    CssRule::Style(style_rule) => {
                        let rule_id = self.rule_manager.create();

                        //TODO: Store map of selectors
                        let selectors = style_rule.selectors;

                        let specificity = selectors.0.first().unwrap().specificity();

                        self.selectors.insert(rule_id, (specificity, selectors));

                        for property in style_rule.declarations.declarations {
                            match property {
                                Property::Transition(transitions) => {
                                    for transition in transitions.iter() {
                                        self.insert_transition(rule_id, transition);
                                    }
                                }

                                _ => {
                                    self.insert_property(rule_id, property);
                                }
                            }
                        }
                    }

                    _ => {}
                }
            }
        }

        // let mut input = ParserInput::new(stylesheet);
        // let mut parser = Parser::new(&mut input);
        // let rule_parser = parser::RuleParser::new();

        // let rules = {
        //     let rule_list_parser =
        //         cssparser::RuleListParser::new_for_stylesheet(&mut parser, rule_parser);
        //     rule_list_parser.collect::<Vec<_>>()
        // };

        // let mut rule_list: Vec<StyleRule> = rules
        //     .into_iter()
        //     .filter_map(|rule| {
        //         match rule {
        //             Ok(mut style_rule) => {
        //                 style_rule.id = self.rule_manager.create();
        //                 Some(style_rule)
        //             }
        //             Err(parse_error) => {
        //                 let style_parse_error = StyleParseError(parse_error.0);
        //                 println!("{}", style_parse_error);
        //                 None
        //             }
        //         }
        //         //rule.ok()
        //     })
        //     .collect();

        // self.rules.append(&mut rule_list);

        // self.rules.sort_by_key(|rule| rule.specificity());
        // self.rules.reverse();

        // // for rule in self.rules.iter() {
        // //     print!("{}", rule);
        // // }

        // self.clear_style_rules();
        // self.set_style_properties();
    }

    fn insert_transition(&mut self, rule_id: Rule, transition: &Transition) {
        let animation = self.animation_manager.create();
        match transition.property.as_ref() {
            "background-color" => {
                self.background_color.insert_animation(animation, self.add_transition(transition));
                self.background_color.insert_transition(rule_id, animation);
                self.transitions.insert(rule_id, animation);
            }
            "border-color" => {
                self.border_color.insert_animation(animation, self.add_transition(transition));
                self.border_color.insert_transition(rule_id, animation);
                self.transitions.insert(rule_id, animation);
            }
            "outline-color" => {
                self.outline_color.insert_animation(animation, self.add_transition(transition));
                self.outline_color.insert_transition(rule_id, animation);
                self.transitions.insert(rule_id, animation);
            }

            "transform" => {
                self.transform.insert_animation(animation, self.add_transition(transition));
                self.transform.insert_transition(rule_id, animation);
                self.transitions.insert(rule_id, animation);
            }

            _ => {}
        }
    }

    fn insert_property(&mut self, rule_id: Rule, property: Property) {
        match property {
            // Display
            Property::Display(display) => {
                self.display.insert_rule(rule_id, display);
            }

            Property::Visibility(visibility) => {
                self.visibility.insert_rule(rule_id, visibility);
            }

            Property::Opacity(opacity) => {
                self.opacity.insert_rule(rule_id, opacity);
            }

            // Space
            Property::Space(space) => {
                self.left.insert_rule(rule_id, space);
                self.right.insert_rule(rule_id, space);
                self.top.insert_rule(rule_id, space);
                self.bottom.insert_rule(rule_id, space);
            }

            Property::Left(left) => {
                self.left.insert_rule(rule_id, left);
            }

            Property::Right(right) => {
                self.right.insert_rule(rule_id, right);
            }

            Property::Top(top) => {
                self.top.insert_rule(rule_id, top);
            }

            Property::Bottom(bottom) => {
                self.bottom.insert_rule(rule_id, bottom);
            }

            // Size
            Property::Size(size) => {
                self.width.insert_rule(rule_id, size);
                self.height.insert_rule(rule_id, size);
            }

            Property::Width(width) => {
                self.width.insert_rule(rule_id, width);
            }

            Property::Height(height) => {
                self.height.insert_rule(rule_id, height);
            }

            // Background
            Property::BackgroundColor(color) => {
                self.background_color.insert_rule(rule_id, color);
            }

            // Border
            Property::BorderWidth(border_width) => {
                self.border_width.insert_rule(rule_id, border_width.top.0);
            }
            Property::BorderColor(color) => {
                self.border_color.insert_rule(rule_id, color);
            }

            // Transform
            Property::Transform(transforms) => {
                println!("Insert transform: {:?}", transforms);
                self.transform.insert_rule(rule_id, transforms);
            }

            _ => {}
        }
    }

    /*
    fn set_style_properties(&mut self) {
        for rule in self.rules.iter() {
            let rule_id = rule.id;

            for property in rule.properties.clone() {
                match property {
                    Property::Display(value) => {
                        self.display.insert_rule(rule_id, value);
                    }

                    Property::Visibility(value) => {
                        self.visibility.insert_rule(rule_id, value);
                    }

                    Property::Opacity(value) => {
                        self.opacity.insert_rule(rule_id, Opacity(value));
                    }

                    Property::Overflow(value) => {
                        self.overflow.insert_rule(rule_id, value);
                    }

                    // Property::BackgroundGradient(value) => {
                    //     self.background_gradient.insert_rule(rule_id, value);
                    // }
                    Property::PositionType(value) => {
                        self.position_type.insert_rule(rule_id, value);
                    }

                    Property::Space(value) => {
                        self.left.insert_rule(rule_id, value);
                        self.right.insert_rule(rule_id, value);
                        self.top.insert_rule(rule_id, value);
                        self.bottom.insert_rule(rule_id, value);
                    }

                    Property::Left(value) => {
                        self.left.insert_rule(rule_id, value);
                    }

                    Property::Right(value) => {
                        self.right.insert_rule(rule_id, value);
                    }

                    Property::Top(value) => {
                        self.top.insert_rule(rule_id, value);
                    }

                    Property::Bottom(value) => {
                        self.bottom.insert_rule(rule_id, value);
                    }

                    // Position Constraints
                    Property::MinLeft(value) => {
                        self.min_left.insert_rule(rule_id, value);
                    }

                    Property::MaxLeft(value) => {
                        self.max_left.insert_rule(rule_id, value);
                    }

                    Property::MinRight(value) => {
                        self.min_right.insert_rule(rule_id, value);
                    }

                    Property::MaxRight(value) => {
                        self.max_right.insert_rule(rule_id, value);
                    }

                    Property::MinTop(value) => {
                        self.min_top.insert_rule(rule_id, value);
                    }

                    Property::MaxTop(value) => {
                        self.max_top.insert_rule(rule_id, value);
                    }

                    Property::MinBottom(value) => {
                        self.min_left.insert_rule(rule_id, value);
                    }

                    Property::MaxBottom(value) => {
                        self.max_left.insert_rule(rule_id, value);
                    }

                    // Size
                    Property::Width(value) => {
                        self.width.insert_rule(rule_id, value);
                    }

                    Property::Height(value) => {
                        self.height.insert_rule(rule_id, value);
                    }

                    // Size Constraints
                    Property::MaxWidth(value) => {
                        self.max_width.insert_rule(rule_id, value);
                    }

                    Property::MinWidth(value) => {
                        self.min_width.insert_rule(rule_id, value);
                    }

                    Property::MaxHeight(value) => {
                        self.max_height.insert_rule(rule_id, value);
                    }

                    Property::MinHeight(value) => {
                        self.min_height.insert_rule(rule_id, value);
                    }

                    // Border
                    Property::BorderWidth(value) => {
                        self.border_width.insert_rule(rule_id, value);
                    }

                    Property::BorderColor(value) => {
                        self.border_color.insert_rule(rule_id, value);
                    }

                    Property::BorderCornerShape(shape) => {
                        self.border_top_left_shape.insert_rule(rule_id, shape);
                        self.border_top_right_shape.insert_rule(rule_id, shape);
                        self.border_bottom_left_shape.insert_rule(rule_id, shape);
                        self.border_bottom_right_shape.insert_rule(rule_id, shape);
                    }

                    Property::BorderTopLeftShape(shape) => {
                        self.border_top_left_shape.insert_rule(rule_id, shape);
                    }

                    Property::BorderTopRightShape(shape) => {
                        self.border_top_right_shape.insert_rule(rule_id, shape);
                    }

                    Property::BorderBottomLeftShape(shape) => {
                        self.border_bottom_left_shape.insert_rule(rule_id, shape);
                    }

                    Property::BorderBottomRightShape(shape) => {
                        self.border_bottom_right_shape.insert_rule(rule_id, shape);
                    }

                    // Border Radius
                    Property::BorderRadius(value) => {
                        self.border_radius_top_left.insert_rule(rule_id, value);
                        self.border_radius_top_right.insert_rule(rule_id, value);
                        self.border_radius_bottom_left.insert_rule(rule_id, value);
                        self.border_radius_bottom_right.insert_rule(rule_id, value);
                    }

                    Property::BorderTopLeftRadius(value) => {
                        self.border_radius_top_left.insert_rule(rule_id, value);
                    }

                    Property::BorderTopRightRadius(value) => {
                        self.border_radius_top_right.insert_rule(rule_id, value);
                    }

                    Property::BorderBottomLeftRadius(value) => {
                        self.border_radius_bottom_left.insert_rule(rule_id, value);
                    }

                    Property::BorderBottomRightRadius(value) => {
                        self.border_radius_bottom_right.insert_rule(rule_id, value);
                    }

                    Property::OutlineWidth(value) => {
                        self.outline_width.insert_rule(rule_id, value);
                    }

                    Property::OutlineColor(value) => {
                        self.outline_color.insert_rule(rule_id, value);
                    }

                    Property::OutlineOffset(value) => {
                        self.outline_offset.insert_rule(rule_id, value);
                    }

                    // Font
                    Property::FontSize(value) => {
                        self.font_size.insert_rule(rule_id, value);
                    }

                    Property::FontColor(value) => {
                        self.font_color.insert_rule(rule_id, value);
                    }

                    Property::Font(value) => {
                        self.font.insert_rule(rule_id, value);
                    }

                    Property::TextWrap(value) => {
                        self.text_wrap.insert_rule(rule_id, value);
                    }

                    Property::SelectionColor(value) => {
                        self.selection_color.insert_rule(rule_id, value);
                    }

                    Property::CaretColor(value) => {
                        self.caret_color.insert_rule(rule_id, value);
                    }

                    // Background
                    Property::BackgroundColor(value) => {
                        self.background_color.insert_rule(rule_id, value);
                    }

                    Property::BackgroundImage(value) => {
                        self.background_image.insert_rule(rule_id, value);
                    }

                    // Layout
                    Property::LayoutType(value) => {
                        self.layout_type.insert_rule(rule_id, value);
                    }

                    Property::ZIndex(value) => {
                        self.z_order.insert_rule(rule_id, value);
                    }

                    // Outer Shadow
                    Property::OuterShadow(box_shadow) => {
                        self.outer_shadow_h_offset
                            .insert_rule(rule_id, box_shadow.horizontal_offset);
                        self.outer_shadow_v_offset.insert_rule(rule_id, box_shadow.vertical_offset);
                        self.outer_shadow_blur.insert_rule(rule_id, box_shadow.blur_radius);
                        self.outer_shadow_color.insert_rule(rule_id, box_shadow.color);
                    }

                    Property::OuterShadowColor(color) => {
                        self.outer_shadow_color.insert_rule(rule_id, color);
                    }

                    // Inner Shadow
                    Property::InnerShadow(box_shadow) => {
                        self.inner_shadow_h_offset
                            .insert_rule(rule_id, box_shadow.horizontal_offset);
                        self.inner_shadow_v_offset.insert_rule(rule_id, box_shadow.vertical_offset);
                        self.inner_shadow_blur.insert_rule(rule_id, box_shadow.blur_radius);
                        self.inner_shadow_color.insert_rule(rule_id, box_shadow.color);
                    }

                    // Child Spacing
                    Property::ChildLeft(value) => {
                        self.child_left.insert_rule(rule_id, value);
                    }

                    Property::ChildRight(value) => {
                        self.child_right.insert_rule(rule_id, value);
                    }

                    Property::ChildTop(value) => {
                        self.child_top.insert_rule(rule_id, value);
                    }

                    Property::ChildBottom(value) => {
                        self.child_bottom.insert_rule(rule_id, value);
                    }

                    Property::ChildSpace(value) => {
                        self.child_left.insert_rule(rule_id, value);
                        self.child_right.insert_rule(rule_id, value);
                        self.child_top.insert_rule(rule_id, value);
                        self.child_bottom.insert_rule(rule_id, value);
                    }

                    Property::RowBetween(value) => {
                        self.row_between.insert_rule(rule_id, value);
                    }

                    Property::ColBetween(value) => {
                        self.col_between.insert_rule(rule_id, value);
                    }

                    Property::Cursor(cursor) => {
                        self.cursor.insert_rule(rule_id, cursor);
                    }

                    // TODO
                    // Property::Translate(value) => {
                    //     self.translate.insert_rule(rule_id, value);
                    // }

                    // Property::Rotate(value) => {
                    //     self.rotate.insert_rule(rule_id, value);
                    // }

                    // Property::Scale(value) => {
                    //     self.scale.insert_rule(rule_id, value.0);
                    // }

                    // Transitions
                    Property::Transition(transitions) => {
                        for transition in transitions {
                            match transition.property.as_ref() {
                                "background-color" => {
                                    let animation = self.animation_manager.create();
                                    self.background_color.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.background_color.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "color" => {
                                    let animation = self.animation_manager.create();
                                    self.font_color.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.font_color.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "left" => {
                                    let animation = self.animation_manager.create();
                                    self.left.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.left.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "top" => {
                                    let animation = self.animation_manager.create();
                                    self.top.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.top.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "right" => {
                                    let animation = self.animation_manager.create();
                                    self.right.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.right.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "bottom" => {
                                    let animation = self.animation_manager.create();
                                    self.bottom.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.bottom.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "min-left" => {
                                    let animation = self.animation_manager.create();
                                    self.min_left.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.min_left.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "max-left" => {
                                    let animation = self.animation_manager.create();
                                    self.max_left.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.max_left.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "min-right" => {
                                    let animation = self.animation_manager.create();
                                    self.min_right.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.min_right.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "max-right" => {
                                    let animation = self.animation_manager.create();
                                    self.max_right.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.max_right.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "min-top" => {
                                    let animation = self.animation_manager.create();
                                    self.min_top.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.min_top.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "max-top" => {
                                    let animation = self.animation_manager.create();
                                    self.max_top.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.max_top.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "min-bottom" => {
                                    let animation = self.animation_manager.create();
                                    self.min_bottom.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.min_bottom.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "max-bottom" => {
                                    let animation = self.animation_manager.create();
                                    self.max_bottom.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.max_bottom.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "width" => {
                                    let animation = self.animation_manager.create();
                                    self.width.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.width.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "height" => {
                                    let animation = self.animation_manager.create();
                                    self.height.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.height.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "min-width" => {
                                    let animation = self.animation_manager.create();
                                    self.min_width.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.min_width.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "max-width" => {
                                    let animation = self.animation_manager.create();
                                    self.max_width.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.max_width.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "min-height" => {
                                    let animation = self.animation_manager.create();
                                    self.min_height.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.min_height.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "max-height" => {
                                    let animation = self.animation_manager.create();
                                    self.max_height.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.max_height.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "child-left" => {
                                    let animation = self.animation_manager.create();
                                    self.child_left.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.child_left.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "child-right" => {
                                    let animation = self.animation_manager.create();
                                    self.child_right.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.child_right.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "child-top" => {
                                    let animation = self.animation_manager.create();
                                    self.child_top.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.child_top.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "child-bottom" => {
                                    let animation = self.animation_manager.create();
                                    self.child_bottom.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.child_bottom.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "col-between" => {
                                    let animation = self.animation_manager.create();
                                    self.col_between.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.col_between.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "row-between" => {
                                    let animation = self.animation_manager.create();
                                    self.row_between.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.row_between.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "opacity" => {
                                    let animation = self.animation_manager.create();
                                    self.opacity.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.opacity.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "outer-shadow-color" => {
                                    let animation = self.animation_manager.create();
                                    self.outer_shadow_color.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.outer_shadow_color.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "outline-width" => {
                                    let animation = self.animation_manager.create();
                                    self.outline_width.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.outline_width.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "outline-color" => {
                                    let animation = self.animation_manager.create();
                                    self.outline_color.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.outline_color.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                "outline-offset" => {
                                    let animation = self.animation_manager.create();
                                    self.outline_offset.insert_animation(
                                        animation,
                                        self.add_transition(transition),
                                    );
                                    self.outline_offset.insert_transition(rule_id, animation);
                                    self.transitions.insert(rule_id, animation);
                                }

                                _ => {}
                            }
                        }
                    }

                    Property::Unknown(s, _) => {
                        println!("Unknown style property: {}", s)
                    }

                    _ => {}
                }
            }
        }
        // println!("{:?}", self.rules);
        // println!("{:?}", self.child_left.shared_data);
    }
    */

    fn add_transition<T: Default + Interpolator>(
        &self,
        transition: &Transition,
    ) -> AnimationState<T> {
        AnimationState::new(Animation::null())
            .with_duration(transition.duration)
            .with_delay(transition.delay)
            .with_keyframe((0.0, Default::default()))
            .with_keyframe((1.0, Default::default()))
    }

    // Add style data to an entity
    pub fn add(&mut self, entity: Entity) {
        self.pseudo_classes
            .insert(entity, PseudoClassFlags::default())
            .expect("Failed to add pseudoclasses");
        self.classes.insert(entity, HashSet::new()).expect("Failed to add class list");
        self.abilities.insert(entity, Abilities::default()).expect("Failed to add abilities");
        self.visibility.insert(entity, Default::default());
        //self.focus_order.insert(entity, Default::default()).unwrap();
        self.needs_restyle = true;
        self.needs_relayout = true;
        self.needs_redraw = true;
    }

    pub fn remove(&mut self, entity: Entity) {
        self.elements.remove(entity);
        self.ids.remove(entity);
        self.classes.remove(entity);
        self.pseudo_classes.remove(entity);
        self.disabled.remove(entity);
        self.abilities.remove(entity);
        // Display
        self.display.remove(entity);
        // Visibility
        self.visibility.remove(entity);
        // Opacity
        self.opacity.remove(entity);
        // Z Order
        self.z_index.remove(entity);
        // Clipping
        self.clip_widget.remove(entity);

        // Transform
        self.transform.remove(entity);
        // self.translate.remove(entity);
        // self.rotate.remove(entity);
        // self.scale.remove(entity);

        self.overflow.remove(entity);

        // Border
        self.border_width.remove(entity);
        self.border_color.remove(entity);

        // Border Shape
        self.border_bottom_left_shape.remove(entity);
        self.border_bottom_right_shape.remove(entity);
        self.border_top_left_shape.remove(entity);
        self.border_top_right_shape.remove(entity);

        // Border Radius
        self.border_bottom_left_radius.remove(entity);
        self.border_bottom_right_radius.remove(entity);
        self.border_top_left_radius.remove(entity);
        self.border_top_right_radius.remove(entity);

        self.outline_width.remove(entity);
        self.outline_color.remove(entity);
        self.outline_offset.remove(entity);

        //self.focus_order.remove(entity);

        // Background
        self.background_color.remove(entity);
        self.background_image.remove(entity);
        self.background_gradient.remove(entity);

        self.outer_shadow_h_offset.remove(entity);
        self.outer_shadow_v_offset.remove(entity);
        self.outer_shadow_blur.remove(entity);
        self.outer_shadow_color.remove(entity);

        self.inner_shadow_h_offset.remove(entity);
        self.inner_shadow_v_offset.remove(entity);
        self.inner_shadow_blur.remove(entity);
        self.inner_shadow_color.remove(entity);

        self.layout_type.remove(entity);
        self.position_type.remove(entity);

        // Space
        self.left.remove(entity);
        self.right.remove(entity);
        self.top.remove(entity);
        self.bottom.remove(entity);

        // Size
        self.width.remove(entity);
        self.height.remove(entity);

        // Space Constraints
        self.min_left.remove(entity);
        self.max_left.remove(entity);
        self.min_right.remove(entity);
        self.max_right.remove(entity);
        self.min_top.remove(entity);
        self.max_top.remove(entity);
        self.min_bottom.remove(entity);
        self.max_bottom.remove(entity);

        // Size Constraints
        self.min_width.remove(entity);
        self.max_width.remove(entity);
        self.min_height.remove(entity);
        self.max_height.remove(entity);
        self.content_width.remove(entity);
        self.content_height.remove(entity);

        // Child Space
        self.child_left.remove(entity);
        self.child_right.remove(entity);
        self.child_top.remove(entity);
        self.child_bottom.remove(entity);
        self.col_between.remove(entity);
        self.row_between.remove(entity);

        // Grid
        self.grid_cols.remove(entity);
        self.grid_rows.remove(entity);
        self.col_index.remove(entity);
        self.col_span.remove(entity);
        self.row_index.remove(entity);
        self.row_span.remove(entity);

        // Text and Font
        self.text.remove(entity);
        self.text_wrap.remove(entity);
        self.font.remove(entity);
        self.font_color.remove(entity);
        self.font_size.remove(entity);
        self.text_selection.remove(entity);
        self.selection_color.remove(entity);
        self.caret_color.remove(entity);

        self.cursor.remove(entity);

        self.name.remove(entity);

        self.image.remove(entity);
    }

    pub fn clear_style_rules(&mut self) {
        self.disabled.clear_rules();
        // Display
        self.display.clear_rules();
        // Visibility
        self.visibility.clear_rules();
        // Opacity
        self.opacity.clear_rules();
        // Z Order
        self.z_index.clear_rules();

        // Transform
        self.transform.clear_rules();
        // self.translate.clear_rules();
        // self.rotate.clear_rules();
        // self.scale.clear_rules();

        self.overflow.clear_rules();

        // Border
        self.border_width.clear_rules();
        self.border_color.clear_rules();

        // Border Shape
        self.border_bottom_left_shape.clear_rules();
        self.border_bottom_right_shape.clear_rules();
        self.border_top_left_shape.clear_rules();
        self.border_top_right_shape.clear_rules();

        // Border Radius
        self.border_bottom_left_radius.clear_rules();
        self.border_bottom_right_radius.clear_rules();
        self.border_top_left_radius.clear_rules();
        self.border_top_right_radius.clear_rules();

        // Outline
        self.outline_width.clear_rules();
        self.outline_color.clear_rules();
        self.outline_offset.clear_rules();

        // Background
        self.background_color.clear_rules();
        self.background_image.clear_rules();
        self.background_gradient.clear_rules();

        self.outer_shadow_h_offset.clear_rules();
        self.outer_shadow_v_offset.clear_rules();
        self.outer_shadow_blur.clear_rules();
        self.outer_shadow_color.clear_rules();

        self.inner_shadow_h_offset.clear_rules();
        self.inner_shadow_v_offset.clear_rules();
        self.inner_shadow_blur.clear_rules();
        self.inner_shadow_color.clear_rules();

        self.layout_type.clear_rules();
        self.position_type.clear_rules();

        // Space
        self.left.clear_rules();
        self.right.clear_rules();
        self.top.clear_rules();
        self.bottom.clear_rules();

        // Size
        self.width.clear_rules();
        self.height.clear_rules();

        // Space Constraints
        self.min_left.clear_rules();
        self.max_left.clear_rules();
        self.min_right.clear_rules();
        self.max_right.clear_rules();
        self.min_top.clear_rules();
        self.max_top.clear_rules();
        self.min_bottom.clear_rules();
        self.max_bottom.clear_rules();

        // Size Constraints
        self.min_width.clear_rules();
        self.max_width.clear_rules();
        self.min_height.clear_rules();
        self.max_height.clear_rules();
        self.content_width.clear_rules();
        self.content_height.clear_rules();

        // Child Space
        self.child_left.clear_rules();
        self.child_right.clear_rules();
        self.child_top.clear_rules();
        self.child_bottom.clear_rules();
        self.col_between.clear_rules();
        self.row_between.clear_rules();

        // Grid
        self.grid_cols.clear_rules();
        self.grid_rows.clear_rules();
        self.col_index.clear_rules();
        self.col_span.clear_rules();
        self.row_index.clear_rules();
        self.row_span.clear_rules();

        // Text and Font
        self.text.clear_rules();
        self.text_wrap.clear_rules();
        self.font.clear_rules();
        self.font_color.clear_rules();
        self.font_size.clear_rules();
        self.text_selection.clear_rules();
        self.selection_color.clear_rules();
        self.caret_color.clear_rules();

        self.cursor.clear_rules();

        self.name.clear_rules();

        self.image.clear_rules();
    }
}
