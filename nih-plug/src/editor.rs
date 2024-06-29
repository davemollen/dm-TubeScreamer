#[path = "./editor/components/param_knob.rs"]
mod param_knob;
use nih_plug::params::Param;
use param_knob::{ParamKnob, ParamKnobSize};
mod ui_data;
use crate::tube_screamer_parameters::TubeScreamerParameters;
use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::{
  model::Model,
  modifiers::{LayoutModifiers, StyleModifiers, TextModifiers},
  prelude::Units::{Pixels, Stretch},
  style::FontWeightKeyword,
  views::{HStack, Label, VStack},
};
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;
use ui_data::{ParamChangeEvent, UiData};

const STYLE: &str = include_str!("./editor/style.css");

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
  ViziaState::new(|| (280, 200))
}

pub(crate) fn create(
  params: Arc<TubeScreamerParameters>,
  editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
  create_vizia_editor(
    editor_state,
    ViziaTheming::Custom,
    move |cx, gui_context| {
      let _ = cx.add_stylesheet(STYLE);

      UiData {
        params: params.clone(),
        gui_context: gui_context.clone(),
      }
      .build(cx);

      VStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
          ParamKnob::new(
            cx,
            params.drive.name(),
            UiData::params,
            params.drive.as_ptr(),
            |params| &params.drive,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            ParamKnobSize::Regular,
          );

          ParamKnob::new(
            cx,
            params.tone.name(),
            UiData::params,
            params.tone.as_ptr(),
            |params| &params.tone,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            ParamKnobSize::Regular,
          );

          ParamKnob::new(
            cx,
            params.level.name(),
            UiData::params,
            params.level.as_ptr(),
            |params| &params.level,
            |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
            ParamKnobSize::Regular,
          );
        })
        .child_space(Stretch(1.0))
        .col_between(Pixels(8.0));

        Label::new(cx, "TUBE SCREAMER")
          .font_size(22.0)
          .font_weight(FontWeightKeyword::Bold)
          .color("#fffafa")
          .child_space(Stretch(1.0))
          .border_color("#fffafa")
          .border_width(Pixels(2.0))
          .border_radius(Pixels(8.0))
          .child_top(Pixels(1.0))
          .child_bottom(Pixels(5.0))
          .width(Pixels(208.0))
          .top(Pixels(32.0))
          .bottom(Pixels(32.0))
          .left(Stretch(1.0))
          .right(Stretch(1.0));
      })
      .child_space(Pixels(16.0))
      .background_color("#528F67");
    },
  )
}
