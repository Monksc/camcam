// #![allow(dead_code)]
use super::*;
use serde::{Serialize, Deserialize};

static ERROR_MSG_COULD_NOT_WRITE: &str = "Could not write in cnc_router.";

fn format_float(x: f64) -> String {
    if x < 0.0 {
        format!("{:.5}", x)
    } else {
        format!("{:.6}", x)
    }
}

#[derive(Copy, Clone, Debug)]
enum SpindleState {
    Off,
    Clockwise,
    CounterClockwise,
}

pub struct CNCRouter<T: std::io::Write> {
    tools: Vec<Tool>,
    current_tool_index: usize,
    pos: Coordinate,
    verbose: bool,
    home_pos: Coordinate,
    referance_pos: Coordinate,
    second_referance_pos: Coordinate,
    spindle_state: SpindleState,
    spindle_clock_speed: f64, // in RPM
    is_flood_colant_on: bool,
    gcode_write: T,
    last_command: String,
    feed_rate: f64,
    exact_stop_change_y: bool,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShapeType(u8);
impl ShapeType {
    const NONE : u8 = 0;
    const TEXT : u8 = 1<<0;
    const BRAILLE : u8 = 1<<1;
    const ALL : u8 = (1<<2) - 1;

    pub fn new() -> Self {
        Self(ShapeType::NONE)
    }

    pub fn text() -> Self {
        Self(Self::TEXT)
    }
    pub fn braille() -> Self {
        Self(Self::BRAILLE)
    }
    pub fn all() -> Self {
        Self(Self::ALL)
    }

    fn is_type(&self, bitmap: u8) -> bool {
        (self.0 & bitmap) == bitmap
    }
    fn set_type(&mut self, bitmap: u8, is_on: bool) {
        if is_on {
            self.0 |= bitmap
        } else {
            self.0 &= self.0 ^ bitmap
        }
    }
    pub fn is_text(&self) -> bool {
        self.is_type(ShapeType::TEXT)
    }
    pub fn is_braille(&self) -> bool {
        self.is_type(ShapeType::BRAILLE)
    }

    pub fn set_text(&mut self, value: bool) {
        self.set_type(ShapeType::TEXT, value)
    }
    pub fn set_braille(&mut self, value: bool) {
        self.set_type(ShapeType::BRAILLE, value)
    }

    pub fn subset_of(&self, other: &Self) -> bool {
        (self.0 & other.0) == self.0
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ToolType {
    FullCutBroad(f64, bool),   // How far can go in the x directions before starting over.
                               // is_contour
    PartialCutBroad,
    SpaceBetweenCutBroad(f64, f64, f64), // bigger radius, extra shrink by, grow by smaller
                                         // (Should be = step over or 0)
    DontAddCutBroad,
    FullContour(ShapeType, f64), // shape then shrink by (used for debug)
    PartialContourAngle(f64, f64, ShapeType), // angle, bigger radius
    PartialContourRadius(f64, f64, ShapeType),  // bigger radius, extra shrink by
    PartialContourRadiusOrAngle(f64, f64, f64, ShapeType),  // bigger radius, extra shrink by, angle,
}

impl Default for ToolType {
    fn default() -> Self {
        Self::FullCutBroad(99999.9, false)
    }
}

impl PartialEq for ToolType {
    fn eq(&self, other: &Self) -> bool {
        self.raw_value() == other.raw_value()
    }
}
impl Eq for ToolType {}
impl std::hash::Hash for ToolType {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        state.write_i32(self.raw_value() as i32);
        state.finish();
    }
}

impl ToolType {
    pub fn full_text() -> Self {
        Self::FullContour(
            ShapeType::text(),
            0.0,
        )
    }
    pub fn full_braille() -> Self {
        Self::FullContour(
            ShapeType::braille(),
            0.0,
        )
    }
    pub fn full_contour_all() -> Self {
        Self::FullContour(
            ShapeType::all(),
            0.0,
        )
    }

    pub fn description(&self) -> String {
        match self {
            ToolType::FullCutBroad(_, _) => String::from("Full Cut Broad"),
            ToolType::PartialCutBroad => String::from("Partial Cut Broad"),
            ToolType::SpaceBetweenCutBroad(_, _, _) => String::from("Space Between Cut Broad"),
            ToolType::DontAddCutBroad => String::from("Don't Add Cut Broad"),
            ToolType::FullContour(_, _) => String::from("Full Contour"),
            ToolType::PartialContourAngle(_, _, _) => String::from("Partial Contour Angle"),
            ToolType::PartialContourRadius(_, _, _) => String::from("Partial Contour Radius"),
            ToolType::PartialContourRadiusOrAngle(_, _, _, _) =>
                String::from("Partial Contour Radius Or Angle"),
        }
    }
    pub fn raw_value(&self) -> u32 {
        match self {
            ToolType::FullCutBroad(_, _) => 0,
            ToolType::PartialCutBroad => 1,
            ToolType::SpaceBetweenCutBroad(_, _, _) => 2,
            ToolType::DontAddCutBroad => 3,
            ToolType::FullContour(_, _) => 4,
            ToolType::PartialContourAngle(_, _, _) => 5,
            ToolType::PartialContourRadius(_, _, _) => 6,
            ToolType::PartialContourRadiusOrAngle(_, _, _, _) => 7,
        }
    }
    pub fn full_cut(self) -> bool {
        if let ToolType::FullCutBroad(_, _) = self {
            true
        } else if let ToolType::FullContour(_, _) = self {
            true
        } else {
            false
        }
    }

    pub fn dont_add_cut(self) -> bool {
        self == ToolType::DontAddCutBroad
    }

    pub fn is_text(self) -> bool {
        if let ToolType::FullContour(shape_type, _) = self {
            shape_type.is_text()
        } else if let ToolType::PartialContourAngle(_, _, shape_type) = self {
            shape_type.is_text()
        } else if let ToolType::PartialContourRadius(_, _, shape_type) = self {
            shape_type.is_text()
        } else if let ToolType::PartialContourRadiusOrAngle(_, _, _, shape_type) = self {
            shape_type.is_text()
        } else {
            false
        }
    }

    pub fn is_braille(self) -> bool {
        if let ToolType::FullContour(shape_type, _) = self {
            shape_type.is_braille()
        } else if let ToolType::PartialContourAngle(_, _, shape_type) = self {
            shape_type.is_braille()
        } else if let ToolType::PartialContourRadius(_, _, shape_type) = self {
            shape_type.is_braille()
        } else if let ToolType::PartialContourRadiusOrAngle(_, _, _, shape_type) = self {
            shape_type.is_braille()
        } else {
            false
        }
    }

    pub fn is_broad(self) -> bool {
        if let ToolType::FullCutBroad(_, _) = self {
            true
        } else if ToolType::PartialCutBroad == self {
            false
        } else if let ToolType::SpaceBetweenCutBroad(_, _, _) = self {
            true
        } else {
            ToolType::DontAddCutBroad == self
        }
    }

    pub fn is_same_type(&self, other: &ToolType) -> bool {
        (self.is_text() && other.is_text()) ||
        (self.is_braille() && other.is_braille()) ||
        (self.is_broad() && other.is_broad())
    }

    pub fn to_shape_type(&self) -> ShapeType {
        let mut shape_type = ShapeType::new();
        if self.is_text() {
            shape_type.set_text(true);
        }
        if self.is_braille() {
            shape_type.set_braille(true);
        }

        return shape_type;
    }

    pub fn is_text_or_braille(&self) -> bool {
        self.is_text() || self.is_braille()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Smoothness {
    Rough,
    Medium,
    Finish,
    NoChange,
}

impl Default for Smoothness {
    fn default() -> Self {
        Self::NoChange
    }
}

impl Smoothness {
    pub fn raw_value(&self) -> usize {
        match self {
            Smoothness::Rough => 0,
            Smoothness::Medium => 1,
            Smoothness::Finish => 2,
            Smoothness::NoChange => 3,
        }
    }
    pub fn description(&self) -> String {
        match self {
            Smoothness::Rough => String::from("Rough"),
            Smoothness::Medium => String::from("Medium"),
            Smoothness::Finish => String::from("Finish"),
            Smoothness::NoChange => String::from("No Change"),
        }
    }
}
impl std::fmt::Display for Smoothness {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Smoothness::NoChange = self {
            Ok(())
        } else {
            write!(f, "P{}", self.raw_value()+1)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tool {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub index_in_machine: usize,
    #[serde(default)]
    pub offset_length: f64,
    #[serde(default)]
    pub radius: f64, // mm
    #[serde(default)]
    pub length: f64,
    #[serde(default)]
    pub front_angle: f64,
    #[serde(default)]
    pub back_angle: f64,
    #[serde(default)]
    pub orientation: f64,
    #[serde(default)]
    pub tool_type: ToolType,
    #[serde(default)]
    pub smoothness: Smoothness,
    #[serde(default)]
    pub feed_rate_of_cut: f64,
    #[serde(default)]
    pub feed_rate_of_drill: f64,
    #[serde(default)]
    pub offset: f64,
    #[serde(default)]
    pub pre_cut_gcode: String,
    #[serde(default)]
    pub force_retouch_off: bool,
    #[serde(default)]
    pub suggested_length: f64,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptionalCoordinate {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>
}

impl <T: std::io::Write> CNCRouter<T> {
    pub fn from(tools: Vec<Tool>, verbose: bool, home_pos: Coordinate,
        gcode_write: T) -> CNCRouter<T> {
        CNCRouter {
            gcode_write: gcode_write,
            tools: tools,
            current_tool_index: 0,
            pos: home_pos,
            verbose: verbose,
            home_pos: home_pos,
            referance_pos: home_pos,
            second_referance_pos: home_pos,
            spindle_state: SpindleState::Off,
            is_flood_colant_on: true,
            spindle_clock_speed: 0.0,
            last_command: String::new(),
            feed_rate: 0.0,
            exact_stop_change_y: false,
        }
    }

    pub fn to_new_write<W: std::io::Write>(&self, w : W) -> CNCRouter<W> {
        CNCRouter::<W> {
            tools: self.tools.clone(),
            current_tool_index: self.current_tool_index,
            pos: self.pos.clone(),
            verbose: self.verbose,
            home_pos: self.home_pos.clone(),
            referance_pos: self.referance_pos.clone(),
            second_referance_pos: self.second_referance_pos.clone(),
            spindle_state: self.spindle_state,
            spindle_clock_speed: self.spindle_clock_speed,
            is_flood_colant_on: self.is_flood_colant_on,
            gcode_write: w,
            last_command: self.last_command.clone(),
            feed_rate: self.feed_rate,
            exact_stop_change_y: self.exact_stop_change_y,
        }
    }

    fn format_float(&self, x: f64) -> String {
        format_float(x)
    }

    fn format_command(&mut self, new_command: String) -> String {
        if self.last_command == new_command &&
            (new_command == "G00" || new_command == "G01") {
            return String::new();
        }
        self.last_command = new_command;
        return self.last_command.clone() + " ";
    }
    fn format_command_str(&mut self, new_command: &str) -> String {
        self.format_command(String::from(new_command))
    }

    pub fn generate_header(&mut self, use_inches: bool, name: &str, extra_header_message: String) {
        self.write_gcode_str(
            "%"
        );
        self.write_gcode_str(
            &name
        );

        self.write_gcode_string(format!(
            "(Generated by camcam from Asante Sign Group)",
        ));
        self.write_gcode_string(extra_header_message);

        for i in 0..self.tools.len() {
            self.write_gcode_string(format!(
                "({} D={} CR=0. - ZMIN={} - flat end mill)",
                self.tools[i].name,
                self.tools[i].radius*2.0,
                self.tools[i].length,
            ));
        }
        if use_inches {
            let verbose = self.verbose_string(String::from(" (Use inches)"));
            self.write_gcode_command("G20", verbose);
        } else {
            let verbose = self.verbose_string(String::from(" (Use millimiters)"));
            self.write_gcode_command("G21", verbose);
        }

        self.set_feed_rate_to_units_per_minute();
        self.set_absolute_mode();
        // self.turn_on_exact_stop_mode();
        self.set_exact_stop_on_y_change(true);
        self.write_gcode_command("G54", self.verbose_str(" (Change 0 coordinate)"));
        self.go_home();
    }

    pub fn get_pos(&self) -> Coordinate {
        self.pos
    }

    pub fn get_point(&self) -> lines_and_curves::Point {
        lines_and_curves::Point::from(self.pos.x, self.pos.y)
    }

    pub fn get_feed_rate(&self) -> f64 {
        self.feed_rate
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    pub fn get_verbose(&self) -> bool {
        self.verbose
    }

    fn verbose_string(&self, str: String) -> String {
        if self.verbose {
            str
        } else {
            String::new()
        }
    }

    fn verbose_str<'a>(&self, str: &'a str) -> &'a str {
        if self.verbose {
            str
        } else {
            ""
        }
    }

    fn option_param<P: ToString>(param_name: String, param: Option<P>) -> String {
        if let Some(p) = param {
            param_name + &p.to_string()
        } else {
            String::new()
        }
    }

    fn option_param_str<P: ToString>(param_name: &str, param: Option<P>) -> String {
        CNCRouter::<T>::option_param(String::from(param_name), param)
    }

    fn set_stat_on_off(&mut self, on_message: String, off_message: String, is_on: bool) -> String {
        if is_on {
            on_message
        } else {
            off_message
        }
    }

    fn set_tool_table_gcode(&mut self) {
        for i in 0..self.tools.len() {
            self.write_gcode_command("G10", format!(
                "L1 P{} axes R{} I{} J{} Q{}{}",
                i+1,
                self.format_float(self.tools[i].radius),
                self.format_float(self.tools[i].front_angle),
                self.format_float(self.tools[i].back_angle),
                self.format_float(self.tools[i].orientation),
                self.verbose_string(String::from(" (Set tool P - tool number R - radius of tool I - front angle (lathe) J - back angle (lathe) Q - orientation (lathe))"))
            ));
        }
    }

    pub fn write_gcode_string_no_line(&mut self, str: String) {
        self.gcode_write.write((str).as_bytes())
            .expect(ERROR_MSG_COULD_NOT_WRITE);
    }

    pub fn write_gcode_string(&mut self, str: String) {
        self.gcode_write.write((str+"\n").as_bytes())
            .expect(ERROR_MSG_COULD_NOT_WRITE);
    }

    pub fn write_gcode_str(&mut self, line: &str) {
        self.gcode_write.write((String::from(line)+&"\n").as_bytes())
            .expect(ERROR_MSG_COULD_NOT_WRITE);
    }

    pub fn write_gcode_command<W: std::fmt::Display>(&mut self, command: &str, line: W) {
        let c = self.format_command_str(command);
        self.write_gcode_string(
            format!("{}{}", c, line)
        )
    }

    // Used if settings become unknown
    // Like call a sub program
    pub fn reset_settings(&mut self) {
        self.last_command = String::new();
    }

    pub fn force_flush_gcode(&mut self) {
        self.gcode_write.flush();
    }


    pub fn clear_gcode_command(&mut self) {
        self.last_command = String::new();
    }

    pub fn write_gcode_comment(&mut self, str: String) {
        if self.verbose {
            self.gcode_write.write((String::from("(") + &str + &")\n").as_bytes())
                .expect(ERROR_MSG_COULD_NOT_WRITE);
        }
    }

    pub fn write_gcode_comment_str(&mut self, comment: &str) {
        if self.verbose {
            self.gcode_write.write((String::from("(") + comment + &")\n").as_bytes())
                .expect(ERROR_MSG_COULD_NOT_WRITE);
        }
    }

    pub fn get_gcode_writer(&self) -> &T {
        &self.gcode_write
    }

    pub fn run_subprogram(&mut self, program: &str) {
        self.write_gcode_command(
            "M98",
            format!(
                "P{}{}",
                program,
                self.verbose_string(
                    String::from(" ( TOOL MSR )")
                )
            )
        )
    }

    pub fn program_stop(&mut self) {
        let verbose = self.verbose_string(String::from("( Program Stop )"));
        self.write_gcode_command(
            "M00",
            verbose
        );
    }

    pub fn reset_home_and_return(&mut self) {
        self.write_gcode_command(
            "G53",
            "G0 Z0."
        );
        self.write_gcode_command(
            "X4.3125",
            "",
        );
        self.write_gcode_command(
            "G53",
            "G0 Y0."
        );
    }

    pub fn end_program(&mut self) {
        self.write_gcode_command(
            "M02",
            self.verbose_string(String::from("(End of program)"))
        );
        self.gcode_write.flush();
    }

    pub fn end_program2(&mut self) {
        self.write_gcode_command(
            "M30",
            self.verbose_string(String::from("(End of program)"))
        );
        self.write_gcode_str(
            "%"
        );
        self.gcode_write.flush();
    }

    pub fn reset_program_and_end(&mut self) {
        // self.set_spindle_off();
        self.go_home();
        // self.program_stop();
        // self.end_program();
        self.reset_home_and_return();
        self.end_program2();
    }

    // MARK: 3d printed functions

    pub fn start_extruding_heat(&mut self) {
        self.write_gcode_command(
            "M104",
            self.verbose_string(String::from(" (Start extruder heating.)"))
        )
    }

    pub fn wait_until_extruder_reaches(&mut self, to: f64) {
        // TODO: Add in the to function
        self.write_gcode_command(
            "M109",
            self.verbose_string(
                format!(
                    " (Wait until extruder reaches to {}.)",
                    self.format_float(to)
                )
            )
        )
    }

    pub fn start_bed_heat(&mut self) {
        self.write_gcode_command(
            "M190",
            self.verbose_string(String::from(" (Start bed heating.)"))
        )
    }

    pub fn wait_until_bed_reaches(&mut self, to: f64) {
        // TODO: Add in the to function
        self.write_gcode_command(
            "M106",
            self.verbose_string(format!(" (Wait until bed reaches to {}.)", to))
        )
    }

    pub fn set_fan_speed(&mut self, speed: f64) {
        // TODO: Add in the speed function
        self.write_gcode_command(
            "M106", self.verbose_string(format!(" (Set fan speed to {}.)", speed))
        )
    }

    pub fn set_spindle_on(&mut self, counter_clockwise: bool, speed: f64) {
        self.spindle_clock_speed = speed;
        self.spindle_state = if counter_clockwise {
            SpindleState::CounterClockwise
        } else {
            SpindleState::Clockwise
        };

        self.format_command_str("");
        self.write_gcode_string(
            format!(
                "S{} M0{}{}", self.format_float(speed), if counter_clockwise { 4 } else { 3 },
                self.verbose_string(String::from(
                    if counter_clockwise {
                        " (Set spindle on to counter clockwise.)"
                    } else {
                        " (Set spindle on to clockwise.)"
                    }
                ))
            )
        )
    }

    pub fn set_spindle_off(&mut self) {
        self.spindle_state = SpindleState::Off;
        self.write_gcode_command(
            "M05", self.verbose_string(String::from(" (Turn off the spindle.)"))
        )
    }

    pub fn set_accuracy_control(&mut self, smoothness: Smoothness) {
        // Have to add in E{} Max corner rounding value
        self.write_gcode_command(
            "G187",
            format!("{}{}", smoothness,
                self.verbose_string(
                    format!(" (Set accuracy to {}.)", smoothness.description())))
        )
    }

    pub fn touch_off_tool(&mut self, suggested_length: f64) {
        self.write_gcode_str("G00 G17 G40 G49 G80 G90");
        self.write_gcode_str("#[10400+#4120]=#[10200+#4120] (Copy from 10201 to 10401 )");
        self.write_gcode_str("#[10200+#4120]=#[10000+#4120] (Copy from 10001 to 10201 )");
        self.write_gcode_string(
            format!(
                "G65 P9995 A0. B1. C2. T{}. E{:.3} D0.( Msr Length )",
                self.tools[self.current_tool_index].index_in_machine,
                suggested_length,
            ));
        self.write_gcode_str("#[10000+#4120]=#[2000+#4120] (Copy H Geom. to 10001 )");
        self.write_gcode_str("M01");
    }

    pub fn set_tool_and_go_home(
        &mut self,
        tool_index: usize,
        feed_rate: f64,
        pre_cut_gcode: &str,
        should_touch_off_tool: bool,
        suggested_length: f64,
    ) {
        self.reset_settings();
        self.current_tool_index = tool_index;
        self.go_home();
        self.write_gcode_str("");
        self.write_gcode_string(
            format!(
                "({})",
                self.tools[tool_index].name,
            )
        );
        self.write_gcode_string(
            format!("T{} M6{}", self.tools[tool_index].index_in_machine,
                self.verbose_string(String::from(" (Tool change.)")))
        );
        self.set_accuracy_control(self.tools[tool_index].smoothness);
        self.set_tool_offset_positive(
            self.tools[tool_index].index_in_machine,
            self.tools[tool_index].offset_length,
            feed_rate
        );
        if should_touch_off_tool {
            self.touch_off_tool(suggested_length);
        }
        self.write_gcode_str(pre_cut_gcode);
        self.write_gcode_string(
            format!("T{} M6{}", self.tools[tool_index].index_in_machine,
                self.verbose_string(String::from(" (Tool change.)")))
        );
        self.set_tool_offset_positive(
            self.tools[tool_index].index_in_machine,
            self.tools[tool_index].offset_length,
            feed_rate
        );
        self.write_gcode_command("G54", self.verbose_str(" (Change 0 coordinate)"));
        self.go_home();
    }

    pub fn get_tools(&self) -> &Vec<Tool> {
        &self.tools
    }

    pub fn set_flood_colant(&mut self, is_on : bool) {
        self.is_flood_colant_on = is_on;
        self.write_gcode_command(
            if is_on { "M08" } else { "M09" },
            self.verbose_string(
                format!("Set flood colant {}.", if is_on { "on" } else { "off" })
            )
        )
    }

    // Non cutting movement
    pub fn move_to_coordinate_rapid(&mut self, pos: &Coordinate) {
        self.pos = *pos;
        self.write_gcode_command(
            "G00",
            format!("X{} Y{} Z{}{}",
                self.format_float(self.pos.x),
                self.format_float(self.pos.y),
                self.format_float(self.pos.z),
                self.verbose_str(
                    " (Moves to position specified rapid.)"
                ),
            )
        )
    }

    // Can cut; feed_rate = unit/minute
    pub fn move_to_coordinate(&mut self, pos: &Coordinate,
        feed_rate: Option<f64>, can_be_skipped: bool) {
        self.pos = *pos;
        let f = if let Some(f) = feed_rate {
            if self.feed_rate == f {
                String::new()
            } else {
                self.feed_rate = f;
                format!(" F{}", self.format_float(self.feed_rate))
            }
        } else {
            String::new()
        };
        self.write_gcode_command(
            if can_be_skipped { "G31" } else { "G01" },
            format!("G09 X{} Y{} Z{}{}{}",
                self.format_float(self.pos.x),
                self.format_float(self.pos.y),
                self.format_float(self.pos.z),
                f,
                self.verbose_string(
                    String::from(" (Cuts to position specified.)") +
                    if can_be_skipped { " Can be skipped." } else { "" }
                )
            )
        )
    }

    pub fn move_to_optional_coordinate(
        &mut self, pos: &OptionalCoordinate,
        feed_rate: Option<f64>, can_be_skipped: bool
    ) {
        let mut exact_cut = "";
        if let Some(x) = pos.x {
            self.pos.x = x;
        }
        if let Some(y) = pos.y {
            self.pos.y = y;
        }
        if let Some(z) = pos.z {
            self.pos.z = z;
            exact_cut = "G09 ";
        }
        let f = if let Some(f) = feed_rate {
            if self.feed_rate == f {
                String::new()
            } else {
                self.feed_rate = f;
                format!(" F{}", self.format_float(self.feed_rate))
            }
        } else {
            String::new()
        };
        self.write_gcode_command(
            if can_be_skipped { "G31" } else { "G01" },
            format!("{}{}{}{}",
                exact_cut,
                pos,
                f,
                self.verbose_string(
                    String::from(" (Cuts to position specified.)") +
                    if can_be_skipped { " Can be skipped." } else { "" }
                )
            )
        )
    }

    pub fn exact_stop(&mut self, pos: &Coordinate) {
        self.pos = *pos;
        self.write_gcode_command(
            "G09",
            format!(
                "X{} Y{} Z{}",
                self.format_float(self.pos.x),
                self.format_float(self.pos.y),
                self.format_float(self.pos.z)
            )
        )
    }

    pub fn exact_stop_next_command(&mut self) {
        if self.verbose {
            self.write_gcode_str("(G09 makes the line exact stop)");
        }
        self.gcode_write.write("G09 ".as_bytes())
            .expect(ERROR_MSG_COULD_NOT_WRITE);
    }

    pub fn pull_out(&mut self, feed_rate: Option<f64>) {
        self.move_to_coordinate(
            &Coordinate::from(
                self.pos.x, self.pos.y, self.home_pos.z),
            feed_rate, false
        )
    }

    pub fn go_home(&mut self) {
        // self.referance_pos = self.pos;
        // self.referance_pos.z = self.home_pos.z;
        self.reset_settings();
        self.write_gcode_command(
            "G00",
            format!("X{} Y{} Z{}{}",
                self.format_float(self.pos.x),
                self.format_float(self.pos.y),
                self.format_float(self.home_pos.z),
                if self.verbose {String::from(" (return home z)")}
                else { String::from("") }
            )
        );
        self.write_gcode_command(
            "G00",
            format!("X{} Y{} Z{}{}",
                self.format_float(self.home_pos.x),
                self.format_float(self.home_pos.y),
                self.format_float(self.home_pos.z),
                if self.verbose {String::from(" (return home)")}
                else { String::from("") }
            )
            // format!("G00 G28 X{} Y{} Z{}{}",
            //     self.pos.x, self.pos.y, self.home_pos.z,
            //     if self.verbose {String::from(" (return home through point specified)")}
            //     else { String::from("") }
            // )
        );
        self.pos = self.home_pos;
    }

    pub fn go_home_incremental(&mut self, d_pos: Coordinate) {
        self.pos = self.home_pos;
        self.referance_pos = d_pos;
        self.clear_gcode_command();
        self.write_gcode_string(
            format!("G91 G28 X{} Y{} Z{} G90{}",
                self.format_float(d_pos.x),
                self.format_float(d_pos.y),
                self.format_float(d_pos.z),
                if self.verbose {String::from(" (return home through point specified)")}
                else { String::from("") }
            )
        )
    }

    pub fn return_to_referance_position(&mut self, intermediate_pos: Coordinate) {
        self.pos = self.referance_pos;
        self.second_referance_pos = intermediate_pos;
        self.write_gcode_command(
            "G29",
            format!("X{} Y{} Z{}{}",
                self.format_float(intermediate_pos.x),
                self.format_float(intermediate_pos.y),
                self.format_float(intermediate_pos.z),
                if self.verbose {String::from(" (return to reference position through point specified.)")}
                else { String::from("") }
            )
        )
    }

    pub fn return_to_second_referance_position(&mut self, intermediate_pos: Coordinate) {
        self.pos = self.second_referance_pos;
        self.write_gcode_command(
            "G30",
            format!("X{} Y{} Z{}{}",
                self.format_float(intermediate_pos.x),
                self.format_float(intermediate_pos.y),
                self.format_float(intermediate_pos.z),
                if self.verbose {String::from(" (return to second reference position through point specified.)")}
                else { String::from("") }
            )
        )
    }

    pub fn set_feed_rate_to_units_per_minute(&mut self) {
        self.write_gcode_command(
            "G94",
            self.verbose_str(" (Set feed rate to units per minute.)")
        )
    }

    pub fn set_feed_rate_to_units_per_revelution(&mut self) {
        self.write_gcode_command(
            "G95",
            self.verbose_str(" (Set feed rate to units per revelution.)")
        )
    }

    pub fn set_absolute_mode(&mut self) {
        self.write_gcode_command(
            "G90",
            self.verbose_str(" (Set to absolute mode.)")
        )
    }

    pub fn set_relative_mode(&mut self) {
        self.write_gcode_command(
            "G91",
            " (G91 changing to relative mode)",
        )
    }

    pub fn set_polar_coordinates(&mut self) {
        self.write_gcode_command(
            "G15",
            self.verbose_str("Set to polar coordinates"),
        )
    }

    pub fn set_stroke_limit(&mut self, is_on: bool) {
        self.write_gcode_command(
            if is_on { "G22" } else { "G23" },
            self.verbose_string(
                format!(" (Stored stroke check {}.)", if is_on { "on" } else { "off" })
            )
        )
    }

    pub fn set_spindle_speed_fluctuation_detect(&mut self, is_on: bool) {
        self.set_stat_on_off(
            format!("G25{}",
                self.verbose_string(String::from(" (Spindle speed fluctuation detect on.)"))),
            format!("G26{}",
                self.verbose_string(String::from(" (Spindle speed fluctuation detect off.)"))),
            is_on,
        );
    }

    pub fn check_for_zero_position(&mut self, commands: Vec<String>) {
        for i in 0..commands.len() {
            self.write_gcode_command(
                "G27",
                self.verbose_str(" (Machine zero position check.)")
            );
            self.write_gcode_string(
                commands[i].clone()
            );
        }
        self.clear_gcode_command()
    }


    pub fn circular_interpolation_offset_midpoint(&mut self,
        is_clock_wise: bool,
        end_pos: &Coordinate, offset: &Coordinate) {
        self.write_gcode_command(
            if is_clock_wise {"G02"} else {"G03"},
            format!("X{} Y{} I{} J{}",
                self.format_float(end_pos.x),
                self.format_float(end_pos.y),
                self.format_float(offset.x),
                self.format_float(offset.y)
            )
        );
        self.pos.x = end_pos.x;
        self.pos.y = end_pos.y;
    }

    pub fn circular_interpolation_exact_midpoint(&mut self,
        is_clock_wise: bool,
        end_pos: &Coordinate, center_pos: &Coordinate) {
        self.circular_interpolation_offset_midpoint(
            is_clock_wise,
            end_pos,
            &(center_pos - (&self.pos))
        );
    }

    pub fn circular_interpolation_around_midpoint(
        &mut self,
        is_clock_wise: bool,
        center_pos: &Coordinate,
    ) {
        let og_pos = self.pos;
        self.circular_interpolation_exact_midpoint(
            is_clock_wise,
            &(2.0 * *center_pos - self.pos), center_pos
        );
        self.circular_interpolation_exact_midpoint(
            is_clock_wise,
            &og_pos, center_pos
        );
    }

    pub fn circular_interpolation_around_change_midpoint(
        &mut self,
        is_clock_wise: bool,
        feed_rate: Option<f64>,
        dx: f64,
        dy: f64
    ) {
        let verbose = self.verbose_str(
            " (Draw a circle with center point of current position + I, J)"
        );
        let feed_rate_msg = if let Some(f) = feed_rate {
            if f == self.feed_rate {
                String::new()
            } else {
                self.feed_rate = f;
                format!("F{} ", f)
            }
        } else {
            String::new()
        };

        self.write_gcode_command(
            if is_clock_wise { "G02" } else { "G03" },
            format!(
                "{}I{} J{}{}", 
                feed_rate_msg,
                dx,
                dy,
                verbose
            )
        );
    }


    pub fn circular_interpolation_exact_midpoint_with_radius(
        &mut self,
        is_clock_wise: bool,
        center_pos: &Coordinate, radius: f64, feed_rate: f64
    ) {
        self.pull_out(Some(feed_rate));
        let mut left_pos = *center_pos;
        left_pos.x -= radius;
        let mut right_pos = *center_pos;
        right_pos.x += radius;
        self.move_to_coordinate_rapid(&right_pos);
        self.move_to_coordinate(&right_pos, Some(feed_rate), false);
        self.circular_interpolation_exact_midpoint(
            is_clock_wise,
            &left_pos, &Coordinate::from_x(-radius),
        );
        self.circular_interpolation_exact_midpoint(
            is_clock_wise,
            &right_pos, &Coordinate::from_x(radius),
        );
    }

    // Pause in code to ensure proper cuts. Use X or U for seconds.
    //      Use P for milliseconds.
    pub fn dewel(&mut self, milliseconds: u64) {
        self.write_gcode_command(
            "G04",
            format!("P{}{}", milliseconds, self.verbose_string(
                format!(" (Dewel for {} milliseconds.)", milliseconds)
            ))
        )
    }

    pub fn cancel_cutter_radius_offset(&mut self) {
        self.write_gcode_command(
            "G40",
            self.verbose_str(
                " (Cutter radius offset cancel.)"
            )
        )
    }

    pub fn cancel_cutter_radius_offset_left(&mut self) {
        self.write_gcode_command(
            "G41",
            self.verbose_str(
                " (Cutter radius offset left.)"
            )
        )
    }

    pub fn cancel_cutter_radius_offset_right(&mut self) {
        self.write_gcode_command(
            "G42",
            self.verbose_str(
                " (Cutter radius offset right.)"
            )
        )
    }

    pub fn set_tool_offset_positive(
        &mut self, tool_index: usize,
        _offset_value: f64, _feed_rate: f64,
    ) {
        self.write_gcode_command(
            "G43",
            format!("H{} {}",
                tool_index,
                self.verbose_string(format!(" (Set tool offset for tool {}.)", tool_index))
            )
        )
    }

    pub fn set_tool_offset_negative(&mut self, tool_index: usize) {
        self.write_gcode_command(
            "G44",
            format!("H{}{}",
                tool_index,
                self.verbose_string(format!(" (Set tool negative offset for tool {}.)", tool_index))
            )
        )
    }

    pub fn set_tool_offset(&mut self, offset: f64) {
        self.write_gcode_command(
            "G43.1",
            format!("Z{}{}",
                self.format_float(offset),
                self.verbose_string(format!(" (Set current tool offset to be {}.)", offset))
            )
        )
    }

    pub fn cancel_tool_length_compensation(&mut self) {
        self.write_gcode_command(
            "G49",
            self.verbose_str(" (Cancel tool length compensation.)")
        )
    }

    pub fn scaling_factor_off(&mut self) {
        self.write_gcode_command(
            "G50",
            self.verbose_string(format!(" (Cancel scaling factor.)"))
        )
    }

    pub fn scaling_factor_on(&mut self, scaling_pivot: Coordinate, scalor_value: f64) {
        self.write_gcode_command(
            "G51",
            format!("I{} J{} K{} P{}{}",
                self.format_float(scaling_pivot.x),
                self.format_float(scaling_pivot.y),
                self.format_float(scaling_pivot.z),
                self.format_float(scalor_value),
                self.verbose_string(format!(" (Cancel scaling factor.)"))
            )
        )
    }

    pub fn local_coordinate_system_set(&mut self, coordinate: Coordinate) {
        self.write_gcode_command(
            "G52",
            format!("X{} Y{} Z{}{}",
                self.format_float(coordinate.x),
                self.format_float(coordinate.y),
                self.format_float(coordinate.z),
                self.verbose_string(format!(" (Sets local coordinate system.)"))
            )
        )
    }

    pub fn machine_coordinate_system_set(&mut self, coordinate: Coordinate) {
        self.write_gcode_command(
            "G53",
            format!("X{} Y{} Z{}{}",
                self.format_float(coordinate.x),
                self.format_float(coordinate.y),
                self.format_float(coordinate.z),
                self.verbose_string(format!(" (Sets machine coordinate system.)"))
            )
        )
    }

    // work_coordinate is a value in [1:9]
    pub fn set_work_coordinate(&mut self, work_coordinate: usize, coordinate: Coordinate) {
        self.write_gcode_command(
            &(if work_coordinate >= 6 {
                String::from("G59.") + &(work_coordinate-6).to_string()
            } else {
                (work_coordinate + 53).to_string()
            }),
            format!("X{} Y{} Z{}{}",
                self.format_float(coordinate.x),
                self.format_float(coordinate.y),
                self.format_float(coordinate.z),
                self.verbose_string(format!(" (Sets the work coordinate.)"))
            )
        )
    }

    pub fn single_direction_positioning(&mut self, pos: &OptionalCoordinate) {
        self.write_gcode_command(
            "G60",
            format!(
                "{}{}",
                pos,
                self.verbose_string(format!(" (Single direction positioning.)"))
            )
        )
    }

    pub fn turn_on_exact_stop_mode(&mut self) {
        self.write_gcode_command(
            "G61",
            self.verbose_string(format!(" (Turns on exact stop mode.)"))
        )
    }

    pub fn set_exact_stop_on_y_change(&mut self, value: bool) {
        self.exact_stop_change_y = value;
    }

    pub fn turn_on_automatic_corner_override_mode(&mut self) {
        self.write_gcode_command(
            "G62",
            format!("G62{}",
                self.verbose_string(format!(" (Automatic corner override mode.)"))
            )
        )
    }

    pub fn tap_make_hole(&mut self, z_distance: f64, spindle_speed: f64) {
        self.clear_gcode_command();
        self.write_gcode_string(
            format!("M3 G91 G3 Z{} F{} M4 G32 Z{} G90 {}{}",
                self.format_float(z_distance),
                self.format_float(spindle_speed),
                self.format_float(-z_distance),
                if let SpindleState::Clockwise = self.spindle_state {
                    format!("M03 S{}", self.spindle_clock_speed)
                } else if let SpindleState::CounterClockwise = self.spindle_state {
                    format!("M04 S{}", self.spindle_clock_speed)
                } else {
                    String::from("M05")
                },
                self.verbose_string(format!(" (Create a hole size {} .)", z_distance))
            )
        );
    }

    // Used to make curves
    pub fn path_blending(&mut self, motion_blending_tolerance: Option<f64>,
        naive_cam_tolerance: Option<f64>) {

        self.write_gcode_command(
            "G64",
            format!("{}{}",
                match (motion_blending_tolerance, naive_cam_tolerance) {
                    (Some(p), Some(q)) => format!(" P{} Q{}", p, q),
                    (Some(p), _) => format!(" P{}", p),
                    (_, _) => String::new(),
                },
                self.verbose_str(" (Used to make curves.)")
        ));
    }

    // TODO: implement custom macros G65-G67

    pub fn coordinate_system_rotation(&mut self, x: f64, y: f64, degrees: f64) {
        self.write_gcode_command(
            "G68",
            format!("X{} Y{} R{}{}",
                self.format_float(x),
                self.format_float(y),
                self.format_float(degrees),
                if self.verbose {String::from(" (Coordinate system rotating around the x and y axis at r degrees.)")} else {String::new()})
        )
    }

    pub fn coordinate_system_rotation_cancel(&mut self) {
        self.write_gcode_command(
            "G69",
            self.verbose_str(" (Cancel coordinate system rotating.)")
        )
    }

    pub fn high_speed_peck_drilling_cycle(&mut self, pos: Coordinate, margin_depth: f64, depth_of_cut: f64, feed_rate: f64, steps: u64) {
        self.feed_rate = feed_rate;
        self.write_gcode_command(
            "G73",
            format!(
                "X{} Y{} Z{} R{} Q{} F{} K{}{}",
                self.format_float(pos.x),
                self.format_float(pos.y),
                self.format_float(pos.z),
                self.format_float(margin_depth),
                self.format_float(depth_of_cut),
                self.format_float(feed_rate),
                steps,
                self.verbose_str(" (High speed peck drilling cycle.)")
            )
        )
    }

    // Used to make left handed threads
    pub fn left_hand_threading_cycle(&mut self, pos: Coordinate, depth_of_cut: f64, feed_rate: f64) {
        self.feed_rate = feed_rate;
        self.write_gcode_command(
            "G74",
            format!(
                "X{} Y{} Z{} R{} F{}{}",
                self.format_float(pos.x),
                self.format_float(pos.y),
                self.format_float(pos.z),
                self.format_float(depth_of_cut),
                self.format_float(feed_rate),
                self.verbose_str(" (Left hand threading cycle.")
            )
        )
    }

    pub fn fanuc_fine_boring_cycle(&mut self, pos: Coordinate, depth_of_cut: f64, shift_at_hole: f64, dwell_time: f64, feed_rate: f64, steps: u64) {
        self.feed_rate = feed_rate;
        self.write_gcode_command(
            "G76",
            format!(
                "X{} Y{} Z{} R{} Q{} P{} F{} K{}{}",
                self.format_float(pos.x),
                self.format_float(pos.y),
                self.format_float(pos.z),
                self.format_float(depth_of_cut),
                self.format_float(shift_at_hole),
                self.format_float(dwell_time),
                self.format_float(feed_rate),
                steps,
                self.verbose_str(" (Creates hole at position given R depth, Q shift, P time, F feed rate and K steps.)")
            )
        )
    }

    pub fn fanuc_boring_cycle(&mut self, pos: Coordinate, starting_position_above_hole: f64, feed_rate: f64, steps: u64) {
        self.feed_rate = feed_rate;
        self.write_gcode_command(
            "G85",
            format!("X{} Y{} Z{} R{} F{} K{}{}",
                self.format_float(pos.x),
                self.format_float(pos.y),
                self.format_float(pos.z),
                self.format_float(starting_position_above_hole),
                self.format_float(feed_rate),
                steps,
                self.verbose_str(" (Creates hole at position given R depth, Q shift, P time, F feed rate and K steps.)")
            )
        )
    }

    // Must use G00 after this
    pub fn fixed_cycle_cancel(&mut self) {
        self.write_gcode_command(
            "G80",
            self.verbose_str(" (Cancels fixed cycles. Must use G00 after this instruction.)")
        )
    }

    pub fn drilling_cycle(&mut self, pos: Coordinate, a: f64, r: f64, l: f64) {
        self.write_gcode_command(
            "G81",
            format!(
                "X{} Y{} Z{} A{} R{} L{}{}",
                self.format_float(pos.x),
                self.format_float(pos.y),
                self.format_float(pos.z),
                self.format_float(a),
                self.format_float(r),
                self.format_float(l),
                self.verbose_str(" (Drilling cycle.)")
            )
        )
    }

    pub fn turn_fan(&mut self, is_on: bool) {
        let verbose = self.verbose_string(
            format!(
                " (Turn {} fan.)",
                if is_on {
                    "on"
                } else {
                    "off"
                }
            )
        );
        self.write_gcode_command(
            if is_on {
                "M83"
            } else {
                "M84"
            },
            verbose,
        )
    }

    pub fn stop_drilling_cycle(&mut self) {
        self.write_gcode_command(
            "G82",
            self.verbose_str(" (Stop drilling cycle.)")
        )
    }

    pub fn normal_peck_drilling_canned_cycle(&mut self, e_chip_clean_rpm: Option<f64>,
        feed_rate: f64, i_size_of_first_peck_depth: Option<f64>, j_reduce_peck_depth: Option<f64>,
        k_minimum_depth_of_peck: Option<f64>, l_number_holes: Option<f64>,
        p_pause_at_end_of_last_peck_seconds: Option<f64>, q_peck_depth: Option<f64>,
        r_position_above_the_part: Option<f64>, x: Option<f64>, y: Option<f64>, z: f64) {
        self.feed_rate = feed_rate;
        self.write_gcode_command(
            "G83",
            format!("{} F{}{}{}{}{}{}{}{}{}{} Z{}{}",
                CNCRouter::<T>::option_param_str(" E", e_chip_clean_rpm),
                self.format_float(feed_rate),
                CNCRouter::<T>::option_param_str(" I", i_size_of_first_peck_depth),
                CNCRouter::<T>::option_param_str(" J", j_reduce_peck_depth),
                CNCRouter::<T>::option_param_str(" K", k_minimum_depth_of_peck),
                CNCRouter::<T>::option_param_str(" L", l_number_holes),
                CNCRouter::<T>::option_param_str(" P", p_pause_at_end_of_last_peck_seconds),
                CNCRouter::<T>::option_param_str(" Q", q_peck_depth),
                CNCRouter::<T>::option_param_str(" R", r_position_above_the_part),
                CNCRouter::<T>::option_param_str(" X", x),
                CNCRouter::<T>::option_param_str(" Y", y),
                z,
                self.verbose_str(" (Stop drilling cycle.)"),
            )
        )
    }

    pub fn right_hand_threading_cycle(&mut self, feed_rate: f64,
        r_position_on_r_plane: Option<f64>, s_rpm: f64, x_axis_motion: Option<f64>, z: f64) {
        self.feed_rate = feed_rate;
        self.write_gcode_command(
            "G84",
            format!("F{}{} S{}{} Z{}{}",
                self.format_float(feed_rate),
                CNCRouter::<T>::option_param_str(" R", r_position_on_r_plane),
                s_rpm,
                CNCRouter::<T>::option_param_str(" X", x_axis_motion),
                z,
                self.verbose_str(" (Right hand threading cycle. Tapping canned cycle.)"),
            )
        )
    }

    pub fn shift_position(&mut self, pos: OptionalCoordinate) {
        self.write_gcode_command(
            "G92",
            format!("{}{}{}{}",
                CNCRouter::<T>::option_param_str(" X", pos.x),
                CNCRouter::<T>::option_param_str(" Y", pos.y),
                CNCRouter::<T>::option_param_str(" Z", pos.z),
                self.verbose_str(" (Shift the coordinates over by specified.)"),
            )
        )
    }

    pub fn use_xy_plane(&mut self) {
        self.write_gcode_command(
            "G17",
            self.verbose_str(" (Switch to XY plane)")
        )
    }


    pub fn use_xz_plane(&mut self) {
        self.write_gcode_command(
            "G18",
            self.verbose_str(" (Switch to XZ plane)")
        )
    }

    pub fn use_yz_plane(&mut self) {
        self.write_gcode_command(
            "G19",
            self.verbose_str(" (Switch to YZ plane)")
        )
    }

}

impl Tool {
    pub fn from(
        name: String, index_in_machine: usize,
        offset_length: f64, radius: f64,
        length: f64, front_angle: f64,
        back_angle: f64, orientation: f64,
        tool_type: ToolType,
        smoothness: Smoothness,
        feed_rate_of_cut: f64,
        feed_rate_of_drill: f64,
        offset: f64,
        pre_cut_gcode: String,
        force_retouch_off: bool,
        suggested_length: f64,
    ) -> Tool {
        Tool {
            name: name,
            index_in_machine: index_in_machine,
            offset_length: offset_length,
            radius: radius,
            length: length,
            front_angle: front_angle,
            back_angle: back_angle,
            orientation: orientation,
            tool_type: tool_type,
            smoothness: smoothness,
            feed_rate_of_cut: feed_rate_of_cut,
            feed_rate_of_drill: feed_rate_of_drill,
            offset: offset,
            pre_cut_gcode: pre_cut_gcode,
            force_retouch_off: force_retouch_off,
            suggested_length: suggested_length,
        }
    }

    pub fn tool_type(&self) -> ToolType {
        self.tool_type
    }

    pub fn extra_distance_x(&self) -> Option<f64> {
        if let ToolType::FullCutBroad(dx, _) = self.tool_type {
            Some(dx)
        } else {
            None
        }
    }
}

impl Coordinate {
    pub fn zero() -> Coordinate {
        Coordinate {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    pub fn from(x: f64, y: f64, z: f64) -> Coordinate {
        Coordinate {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn from_x(x: f64) -> Coordinate {
        Coordinate {
            x: x,
            y: 0.0,
            z: 0.0
        }
    }

    pub fn from_y(y: f64) -> Coordinate {
        Coordinate {
            x: 0.0,
            y: y,
            z: 0.0
        }
    }

    pub fn from_z(z: f64) -> Coordinate {
        Coordinate {
            x: 0.0,
            y: 0.0,
            z: z
        }
    }

    pub fn to_optional(&self) -> OptionalCoordinate {
        OptionalCoordinate::from(
            Some(self.x),
            Some(self.y),
            Some(self.z),
        )
    }
}

use std::ops::Add;
impl Add for Coordinate {
    type Output = Coordinate;
    fn add(self, rhs: Coordinate) -> Coordinate {
        Coordinate::from(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        )
    }
}
impl Add for &Coordinate {
    type Output = Coordinate;
    fn add(self, rhs: &Coordinate) -> Coordinate {
        Coordinate::from(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        )
    }
}

use std::ops::Sub;
impl Sub for Coordinate {
    type Output = Coordinate;
    fn sub(self, rhs: Coordinate) -> Coordinate {
        Coordinate::from(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        )
    }
}
impl Sub for &Coordinate {
    type Output = Coordinate;
    fn sub(self, rhs: &Coordinate) -> Coordinate {
        Coordinate::from(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        )
    }
}

impl std::ops::Mul<Coordinate> for f64 {
    type Output = Coordinate;

    fn mul(self, rhs: Coordinate) -> Coordinate {
        Coordinate::from(
            rhs.x * self,
            rhs.y * self,
            rhs.z * self
        )
    }
}
impl std::ops::Mul<&Coordinate> for f64 {
    type Output = Coordinate;

    fn mul(self, rhs: &Coordinate) -> Coordinate {
        Coordinate::from(
            rhs.x * self,
            rhs.y * self,
            rhs.z * self
        )
    }
}

// MARK: Coordinate

impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "X{} Y{} Z{}",
            format_float(self.x),
            format_float(self.y),
            format_float(self.z),
        )
    }

}

// MARK: OptionalCoordinate

impl OptionalCoordinate {
    pub fn zero() -> OptionalCoordinate {
        OptionalCoordinate {
            x: Some(0.0),
            y: Some(0.0),
            z: Some(0.0)
        }
    }

    pub fn from(x: Option<f64>, y: Option<f64>, z: Option<f64>) -> OptionalCoordinate {
        OptionalCoordinate {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn from_x(x: Option<f64>) -> OptionalCoordinate {
        OptionalCoordinate {
            x: x,
            y: None,
            z: None
        }
    }

    pub fn from_y(y: Option<f64>) -> OptionalCoordinate {
        OptionalCoordinate {
            x: None,
            y: y,
            z: None,
        }
    }

    pub fn from_z(z: Option<f64>) -> OptionalCoordinate {
        OptionalCoordinate {
            x: None,
            y: None,
            z: z
        }
    }
}

impl std::fmt::Display for OptionalCoordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut space = String::new();
        if let Some(x) = self.x {
            if let Err(e) = write!(f, "X{}", format_float(x)) {
                return Err(e);
            }
            space = String::from(" ");
        }
        if let Some(y) = self.y {
            if let Err(e) = write!(f, "{}Y{}", space, format_float(y)) {
                return Err(e);
            }
        }
        if let Some(z) = self.z {
            if let Err(e) = write!(f, "{}Z{}", space, format_float(z)) {
                return Err(e);
            }
        }

        Ok(())
    }
}

fn do_math(default_val: f64, lhs: Option<f64>, rhs: Option<f64>, f: fn(f64, f64) -> f64) -> Option<f64> {
    match (lhs, rhs) {
        (Some(l), Some(r)) => Some(f(l, r)),
        (_, Some(r)) => Some(f(default_val, r)),
        (Some(l), _) => Some(f(l, default_val)),
        (_, _) => None
    }
}

fn do_math_add(lhs: Option<f64>, rhs: Option<f64>) -> Option<f64> {
    do_math(0.0, lhs, rhs, |l, r| { l + r })
}

fn do_math_sub(lhs: Option<f64>, rhs: Option<f64>) -> Option<f64> {
    do_math(0.0, lhs, rhs, |l, r| { l - r })
}

fn do_math_mul(lhs: f64, rhs: Option<f64>) -> Option<f64> {
    do_math(1.0, Some(lhs), rhs, |l, r| { l * r })
}

impl Add for OptionalCoordinate {
    type Output = OptionalCoordinate;
    fn add(self, rhs: OptionalCoordinate) -> OptionalCoordinate {
        OptionalCoordinate::from(
            do_math_add(self.x, rhs.x),
            do_math_add(self.y, rhs.y), do_math_add(self.z, rhs.z),
        )
    }
}
impl Add for &OptionalCoordinate {
    type Output = OptionalCoordinate;
    fn add(self, rhs: &OptionalCoordinate) -> OptionalCoordinate {
        OptionalCoordinate::from(
            do_math_add(self.x, rhs.x),
            do_math_add(self.y, rhs.y),
            do_math_add(self.z, rhs.z),
        )
    }
}

impl Sub for OptionalCoordinate {
    type Output = OptionalCoordinate;
    fn sub(self, rhs: OptionalCoordinate) -> OptionalCoordinate {
        OptionalCoordinate::from(
            do_math_sub(self.x, rhs.x),
            do_math_sub(self.y, rhs.y),
            do_math_sub(self.z, rhs.z),
        )
    }
}
impl Sub for &OptionalCoordinate {
    type Output = OptionalCoordinate;
    fn sub(self, rhs: &OptionalCoordinate) -> OptionalCoordinate {
        OptionalCoordinate::from(
            do_math_sub(self.x, rhs.x),
            do_math_sub(self.y, rhs.y),
            do_math_sub(self.z, rhs.z),
        )
    }
}

impl std::ops::Mul<OptionalCoordinate> for f64 {
    type Output = OptionalCoordinate;

    fn mul(self, rhs: OptionalCoordinate) -> OptionalCoordinate {
        OptionalCoordinate::from(
            do_math_mul(self, rhs.x),
            do_math_mul(self, rhs.y),
            do_math_mul(self, rhs.z),
        )
    }
}
impl std::ops::Mul<&OptionalCoordinate> for f64 {
    type Output = OptionalCoordinate;

    fn mul(self, rhs: &OptionalCoordinate) -> OptionalCoordinate {
        OptionalCoordinate::from(
            do_math_mul(self, rhs.x),
            do_math_mul(self, rhs.y),
            do_math_mul(self, rhs.z),
        )
    }
}


pub trait CNCPath {
    fn to_path(
        &self
    ) -> Vec<OptionalCoordinate> {
        Vec::new()
    }

    fn to_path_vec(
        items: &Vec<Self>,
    ) -> Vec<OptionalCoordinate> where Self : Sized {
        items.iter().map(|item| item.to_path()).flatten().collect()
    }

    fn is_connected(&self) -> bool;

    fn start_path(&self) -> Option<Coordinate>;

    fn follow_path<T: std::io::Write>(
        &self,
        cnc_router: &mut CNCRouter<T>,
        feed_rate: Option<f64>,
    ) {
        for pos in self.to_path() {
            cnc_router.move_to_optional_coordinate(
                &pos, feed_rate, false,
            )
        }
    }

    fn cut_till<T: std::io::Write>(
        items: &Vec<Self>,
        x: Option<f64>,
        y: Option<f64>,
        cnc_router: &mut CNCRouter<T>,
        feed_rate: Option<f64>,
        force_drill: bool, // can only be true if x and y is none
        feed_rate_of_drill: f64, // only in effect iff force_drill
        z_axis_of_cut: f64, // only in effect iff force_drill
        depth_of_cut: f64, // only in effect iff force_drill
        tool_type: &ToolType,
        tool_radius: f64,
        tool_offset: f64,
        cut_inside: bool,
        mut can_cut: Box::<impl FnMut(f64, f64) -> bool>
    ) -> bool where Self : Sized {
        let points = CNCPath::to_path_vec(items);

        let mut new_points = Vec::new();
        for p in points {
            let (Some(x), Some(y)) = (p.x, p.y) else {
                return false;
            };
            new_points.push(lines_and_curves::Point::from(x, y));
        }

        if new_points.len() < 3 {
            return false;
        }
        // new_points.push(new_points[0]);
        if let Some(x) = x {
            let start_pos = cnc_router.get_pos();
            let mut start_index = 0;
            let mut closest_distance = 0.1; // if no closer we might as well take index 0
            for i in 0..new_points.len() {
                let j = (i+1) % new_points.len();
                let line = lines_and_curves::LineSegment::from(
                    lines_and_curves::Point::from(
                        new_points[i].x,
                        new_points[i].y,
                    ),
                    lines_and_curves::Point::from(
                        new_points[j].x,
                        new_points[j].y,
                    ),
                );

                let distance = line.distance_to_point(&lines_and_curves::Point::from(
                    start_pos.x,
                    start_pos.y
                ));
                if distance < closest_distance {
                    closest_distance = distance;
                    start_index = i;
                }
            }

            let mut higher_distance = 0.0;
            'findhigherindex: for h in 0..new_points.len() {
                let i = (h+start_index) % new_points.len();
                let j = (h+start_index+1) % new_points.len();

                let x_values = [x, start_pos.x];
                for z in 0..(if h == 0 { 1 } else { 2 }) {
                    let x = x_values[z];
                    if (x >= new_points[j].x && new_points[i].x > x) ||
                        (x <= new_points[j].x && new_points[i].x < x)
                    {
                        break 'findhigherindex;
                    }
                }
                higher_distance += new_points[i].distance_to(&new_points[j]);
            }

            let mut lower_distance = 0.0;
            'findlowerindex: for h in 0..new_points.len() {
                let i = (3*new_points.len()-h+start_index+1) % new_points.len();
                let j = (3*new_points.len()-h+start_index) % new_points.len();

                let x_values = [x, start_pos.x];
                for z in 0..(if h == 0 { 1 } else { 2 }) {
                    let x = x_values[z];
                    if (x >= new_points[j].x && new_points[i].x > x) ||
                        (x <= new_points[j].x && new_points[i].x < x)
                    {
                        break 'findlowerindex;
                    }
                }
                lower_distance += new_points[i].distance_to(&new_points[j]);
            }

            let should_go_higher = higher_distance <= lower_distance;

            for h in 0..new_points.len() {
                let (new_h, addition) = if should_go_higher {
                    (h, 1)
                } else {
                    (1+new_points.len()-h, new_points.len()-1)
                };
                let i = (new_h+start_index) % new_points.len();
                let j = (new_h+start_index+addition) % new_points.len();

                let x_values = [x, start_pos.x];
                for z in 0..(if h == 0 { 1 } else { 2 }) {
                    let x = x_values[z];
                    if (x >= new_points[j].x && new_points[i].x > x) ||
                        (x <= new_points[j].x && new_points[i].x < x)
                    {
                        let line = lines_and_curves::LineSegment::from(
                            new_points[i],
                            new_points[j]
                        );

                        cnc_router.move_to_optional_coordinate(
                            &OptionalCoordinate::from(
                                Some(x),
                                line.y(x),
                                None,
                            ),
                            feed_rate, false,
                        );
                        return true;
                    }
                }
                // let index = if should_go_higher { j } else { i };
                cnc_router.move_to_optional_coordinate(
                    &OptionalCoordinate::from(
                        Some(new_points[j].x),
                        Some(new_points[j].y),
                        None,
                    ),
                    feed_rate, false,
                );
                if !can_cut(new_points[j].x, new_points[j].y) {
                    return true;
                }
            }
        } else if let Some(y) = y {

        } else if let ToolType::PartialContourAngle(max_angle, previous_radius, _) = tool_type {
            let mut is_up = true;
            for i in 0..new_points.len() {
                let j = (i+1) % new_points.len();
                let k = (i+2) % new_points.len();

                let angle = lines_and_curves::Point::right_angle(
                    &new_points[i],
                    &new_points[j],
                    &new_points[k],
                );

                let angle = if cut_inside {
                    2.0 * std::f64::consts::PI - angle
                } else {
                    angle
                };

                if angle.is_nan() ||
                    angle >= std::f64::consts::PI ||
                    (*max_angle > 0.0 && angle > *max_angle) {
                    if !is_up {
                        cnc_router.move_to_optional_coordinate(
                            &cnc_router::OptionalCoordinate::from_z(
                                Some(z_axis_of_cut)
                            ),
                            Some(feed_rate_of_drill), false,
                        );
                        is_up = true;
                    }
                    continue;
                }

                // TODO: Maybe it should be (*previous_radius - tool_radius)
                // or length - tool_radius
                // let length_from_point_j = (*previous_radius - tool_radius) / (angle / 2.0).tan();
                let length_from_point_j = *previous_radius / (angle / 2.0).tan();

                let dji = (new_points[i] - new_points[j]).normalize();
                let djk = (new_points[k] - new_points[j]).normalize();

                let ij_point = if length_from_point_j >
                    new_points[i].distance_to(&new_points[j]) {
                    dji * new_points[i].distance_to(&new_points[j]) + new_points[j]
                } else {
                    dji * length_from_point_j + new_points[j]
                };
                let jk_point = if length_from_point_j >
                    new_points[k].distance_to(&new_points[j]) {
                    djk * new_points[k].distance_to(&new_points[j]) + new_points[j]
                } else {
                    djk * length_from_point_j + new_points[j]
                };

                if !is_up && cnc_router.get_point().distance_to(
                    &ij_point
                ) * feed_rate.unwrap_or(cnc_router.get_feed_rate()) > (
                    2.0 * feed_rate_of_drill * depth_of_cut
                ).abs() {
                    cnc_router.move_to_optional_coordinate(
                        &cnc_router::OptionalCoordinate::from_z(
                            Some(z_axis_of_cut)
                        ),
                        Some(feed_rate_of_drill), false,
                    );
                    is_up = true;
                }

                if is_up {
                    cnc_router.move_to_coordinate_rapid(
                        &cnc_router::Coordinate::from(
                            ij_point.x, ij_point.y, z_axis_of_cut
                        )
                    );

                    cnc_router.move_to_optional_coordinate(
                        &cnc_router::OptionalCoordinate::from_z(
                            Some(z_axis_of_cut + depth_of_cut)
                        ),
                        Some(feed_rate_of_drill), false,
                    );
                    is_up = false;
                }

                cnc_router.move_to_optional_coordinate(
                    &OptionalCoordinate::from(
                        Some(new_points[j].x),
                        Some(new_points[j].y),
                        None,
                    ),
                    feed_rate, false,
                );
                cnc_router.move_to_optional_coordinate(
                    &OptionalCoordinate::from(
                        Some(jk_point.x),
                        Some(jk_point.y),
                        None,
                    ),
                    feed_rate, false,
                );
            }
        } else if let ToolType::PartialContourRadiusOrAngle(previous_radius, _, max_angle, _) = tool_type {
            let mut is_up = true;
            for i in 0..new_points.len() {
                let j = (i+1) % new_points.len();
                let k = (i+2) % new_points.len();

                let angle = lines_and_curves::Point::right_angle(
                    &new_points[i],
                    &new_points[j],
                    &new_points[k],
                );

                let angle = if cut_inside {
                    2.0 * std::f64::consts::PI - angle
                } else {
                    angle
                };

                let mut find_farthest_point_needs_to_be_cut = |
                    distance_line : f64,
                    farther_point: lines_and_curves::Point,
                | {
                    let mut farthest_point_needs_to_be_cut = None;
                    let n = (distance_line / (2.0 * tool_offset * tool_radius)).ceil() as usize;
                    for li in 0..=n {
                        let prob = 0.5 - (li as f64 / n as f64) / 2.0;
                        let x = farther_point.x * prob + new_points[j].x * (1.0 - prob);
                        let y = farther_point.y * prob + new_points[j].y * (1.0 - prob);
                        if can_cut(
                            x, y,
                        ) {
                            let prob = 0.5 - (
                                if li > 0 {
                                    li - 1
                                } else {
                                    li
                                } as f64
                                / n as f64
                            ) / 2.0;

                            let x = farther_point.x * prob + new_points[j].x * (1.0 - prob);
                            let y = farther_point.y * prob + new_points[j].y * (1.0 - prob);

                            farthest_point_needs_to_be_cut = Some(
                                lines_and_curves::Point::from(
                                    x, y,
                                )
                            );
                            break;
                        }
                    }

                    return farthest_point_needs_to_be_cut;
                };

                let distance_ij = new_points[i].distance_to(&new_points[j]);
                let farthest_point_needs_to_be_cut_ij = find_farthest_point_needs_to_be_cut(
                    distance_ij,
                    new_points[i],
                );

                let distance_jk = new_points[j].distance_to(&new_points[k]);
                let farthest_point_needs_to_be_cut_jk = find_farthest_point_needs_to_be_cut(
                    distance_jk,
                    new_points[k],
                );

                let needs_cut_tight_angle = !(
                    angle.is_nan() ||
                    angle >= std::f64::consts::PI ||
                    (*max_angle > 0.0 && angle > *max_angle)
                );

                if !(
                    if let Some(_) = farthest_point_needs_to_be_cut_ij { true } else { false } ||
                    if let Some(_) = farthest_point_needs_to_be_cut_jk { true } else { false } ||
                    needs_cut_tight_angle
                ) {
                    if !is_up {
                        cnc_router.move_to_optional_coordinate(
                            &cnc_router::OptionalCoordinate::from_z(
                                Some(z_axis_of_cut)
                            ),
                            Some(feed_rate_of_drill), false,
                        );
                        is_up = true;
                    }
                    continue;
                }

                // TODO: Maybe it should be (*previous_radius - tool_radius)
                // or length - tool_radius
                // let length_from_point_j = (*previous_radius - tool_radius) / (angle / 2.0).tan();
                let length_from_point_j = *previous_radius / (angle / 2.0).tan();

                let dji = (new_points[i] - new_points[j]).normalize();
                let djk = (new_points[k] - new_points[j]).normalize();

                let get_point = |
                    max_distance: f64,
                    farthest_point_from_j: Option<lines_and_curves::Point>,
                    dj: lines_and_curves::Point,
                    end_point: lines_and_curves::Point,
                | {
                    if
                        length_from_point_j <
                        max_distance / 2.0
                        && if let Some(_) = farthest_point_from_j { true } else { false }
                    {
                        if let Some(point) = farthest_point_from_j {
                            let d = point.distance_to(&new_points[j]);
                            if length_from_point_j > d {
                                dj * length_from_point_j + new_points[j]
                            } else {
                                point
                            }
                        } else {
                            (new_points[i] + new_points[j]) / 2.0
                        }
                    } else if needs_cut_tight_angle && (
                        length_from_point_j >
                        max_distance
                    ) {
                        end_point
                    } else if needs_cut_tight_angle {
                        dj * length_from_point_j + new_points[j]
                    } else {
                        new_points[j]
                    }
                };


                let ij_point = get_point(
                    distance_ij,
                    farthest_point_needs_to_be_cut_ij,
                    dji,
                    new_points[i],
                );

                let jk_point = get_point(
                    distance_jk,
                    farthest_point_needs_to_be_cut_jk,
                    djk,
                    new_points[k],
                );

                if !is_up && cnc_router.get_point().distance_to(
                    &ij_point
                ) * feed_rate.unwrap_or(cnc_router.get_feed_rate()) > (
                    2.0 * feed_rate_of_drill * depth_of_cut
                ).abs() {
                    cnc_router.move_to_optional_coordinate(
                        &cnc_router::OptionalCoordinate::from_z(
                            Some(z_axis_of_cut)
                        ),
                        Some(feed_rate_of_drill), false,
                    );
                    is_up = true;
                }

                if is_up {
                    cnc_router.move_to_coordinate_rapid(
                        &cnc_router::Coordinate::from(
                            ij_point.x, ij_point.y, z_axis_of_cut
                        )
                    );

                    cnc_router.move_to_optional_coordinate(
                        &cnc_router::OptionalCoordinate::from_z(
                            Some(z_axis_of_cut + depth_of_cut)
                        ),
                        Some(feed_rate_of_drill), false,
                    );
                    is_up = false;
                }

                cnc_router.move_to_optional_coordinate(
                    &OptionalCoordinate::from(
                        Some(new_points[j].x),
                        Some(new_points[j].y),
                        None,
                    ),
                    feed_rate, false,
                );
                cnc_router.move_to_optional_coordinate(
                    &OptionalCoordinate::from(
                        Some(jk_point.x),
                        Some(jk_point.y),
                        None,
                    ),
                    feed_rate, false,
                );
            }
        } else if let ToolType::PartialContourRadius(_, _, _) = tool_type {
            let mut is_up = true;
            for i in 0..new_points.len() {
                let j = (i+1) % new_points.len();
                let m = (i+new_points.len()-1) % new_points.len();

                if  !can_cut(new_points[i].x, new_points[i].y) &&
                    !can_cut(new_points[j].x, new_points[j].y) &&
                    !can_cut(
                        (new_points[i].x + new_points[j].x) / 2.0,
                        (new_points[i].y + new_points[j].y) / 2.0,
                    ) &&
                    !can_cut(new_points[m].x, new_points[m].y)
                {
                    if !is_up {
                        cnc_router.move_to_optional_coordinate(
                            &cnc_router::OptionalCoordinate::from_z(
                                Some(z_axis_of_cut)
                            ),
                            Some(feed_rate_of_drill), false,
                        );
                        is_up = true;
                    }
                    continue
                }

                if is_up {
                    cnc_router.move_to_coordinate_rapid(
                        &cnc_router::Coordinate::from(
                            new_points[i].x, new_points[i].y, z_axis_of_cut
                        )
                    );
                    cnc_router.move_to_optional_coordinate(
                        &cnc_router::OptionalCoordinate::from_z(
                            Some(z_axis_of_cut + depth_of_cut)
                        ),
                        Some(feed_rate_of_drill), false,
                    );
                    is_up = false;
                }

                cnc_router.move_to_optional_coordinate(
                    &OptionalCoordinate::from(
                        Some(new_points[j].x),
                        Some(new_points[j].y),
                        None,
                    ),
                    feed_rate, false,
                );
            }
            if !is_up {
                cnc_router.move_to_optional_coordinate(
                    &cnc_router::OptionalCoordinate::from_z(
                        Some(z_axis_of_cut)
                    ),
                    Some(feed_rate_of_drill), false,
                );
            }
        } else {
            new_points.push(new_points[0]);
            if force_drill {
                cnc_router.move_to_coordinate_rapid(
                    &cnc_router::Coordinate::from(
                        new_points[0].x, new_points[0].y, z_axis_of_cut
                    )
                );

                cnc_router.move_to_optional_coordinate(
                    &cnc_router::OptionalCoordinate::from_z(
                        Some(z_axis_of_cut + depth_of_cut)
                    ),
                    Some(feed_rate_of_drill), false,
                );
            }

            for pos in new_points {
                cnc_router.move_to_optional_coordinate(
                    &OptionalCoordinate::from(
                        Some(pos.x),
                        Some(pos.y),
                        None,
                    ),
                    feed_rate, false,
                )
            }

            if force_drill {
                cnc_router.move_to_optional_coordinate(
                    &cnc_router::OptionalCoordinate::from_z(
                        Some(z_axis_of_cut)
                    ),
                    Some(feed_rate_of_drill), false,
                );
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_shape_type() {
        let mut shape = ShapeType::new();
        assert_eq!(shape.0, 0);

        shape.set_text(true);
        assert_eq!(shape.0, 1);
        shape.set_text(false);
        assert_eq!(shape.0, 0);

        shape.set_braille(true);
        assert_eq!(shape.0, 2);
        shape.set_braille(false);
        assert_eq!(shape.0, 0);
        assert!(!shape.is_braille());
        assert!(!shape.is_text());

        shape.set_text(true);
        shape.set_braille(true);
        assert_eq!(shape.0, 3);

        assert!(shape.is_braille());
        assert!(shape.is_text());
    }
}
