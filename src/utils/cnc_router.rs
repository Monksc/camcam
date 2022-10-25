#![allow(dead_code)]
use super::*;

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
    gcode_write: T
}

#[derive(Copy, Clone, Debug)]
pub enum ToolType {
    PartialCutBroad,
    FullCutBroad,
    Text,
    Braille
}

#[derive(Copy, Clone, Debug)]
pub enum Smoothness {
    Rough,
    Medium,
    Finish,
}
impl Smoothness {
    pub fn raw_value(&self) -> usize {
        match self {
            Smoothness::Rough => 0,
            Smoothness::Medium => 1,
            Smoothness::Finish => 2,
        }
    }
    pub fn description(&self) -> String {
        match self {
            Smoothness::Rough => String::from("Rough"),
            Smoothness::Medium => String::from("Medium"),
            Smoothness::Finish => String::from("Finish"),
        }
    }
}
impl std::fmt::Display for Smoothness {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "P{}", self.raw_value()+1)
    }
}

#[derive(Clone, Debug)]
pub struct Tool {
    pub name: String,
    pub index_in_machine: usize,
    pub offset_length: f64,
    pub radius: f64, // mm
    pub length: f64,
    pub front_angle: f64,
    pub back_angle: f64,
    pub orientation: f64,
    pub tool_type: ToolType,
    pub smoothness: Smoothness,
}

#[derive(Copy, Clone)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

#[derive(Copy, Clone)]
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
        }
    }

    pub fn generate_header(&mut self, use_inches: bool) {
        self.write_gcode_str(
            "(Using G0 which travels along dogleg path.)"
        );
        self.write_gcode_str(
            "O81111 (The Diamond)"
        );
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
            self.write_gcode_string(format!("G20{}", verbose));
        } else {
            let verbose = self.verbose_string(String::from(" (Use millimiters)"));
            self.write_gcode_string(format!("G21{}", verbose));
        }

        self.set_feed_rate_to_units_per_minute();
        self.set_absolute_mode();
        self.write_gcode_string(format!("G54"));
        self.go_home();
    }

    pub fn get_pos(&self) -> Coordinate {
        self.pos
    }

    pub fn get_point(&self) -> lines_and_curves::Point {
        lines_and_curves::Point::from(self.pos.x, self.pos.y)
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
            self.write_gcode_string(format!(
                "G10 L1 P{} axes R{} I{} J{} Q{}{}",
                i+1,
                self.tools[i].radius,
                self.tools[i].front_angle,
                self.tools[i].back_angle,
                self.tools[i].orientation,
                self.verbose_string(String::from(" (Set tool P - tool number R - radius of tool I - front angle (lathe) J - back angle (lathe) Q - orientation (lathe))"))
            ));
        }
    }

    pub fn write_gcode_string(&mut self, str: String) {
        self.gcode_write.write((str+"\n").as_bytes());
    }

    pub fn write_gcode_str(&mut self, line: &str) {
        self.gcode_write.write((String::from(line)+&"\n").as_bytes());
    }

    pub fn write_gcode_comment(&mut self, str: String) {
        if self.verbose {
            self.gcode_write.write((String::from("(") + &str + &")\n").as_bytes());
        }
    }

    pub fn write_gcode_comment_str(&mut self, comment: &str) {
        if self.verbose {
            self.gcode_write.write((String::from("(") + comment + &")\n").as_bytes());
        }
    }

    pub fn get_gcode_writer(&self) -> &T {
        &self.gcode_write
    }

    pub fn program_stop(&mut self) {
        let verbose = self.verbose_string(String::from("( Program Stop )"));
        self.write_gcode_string(
            format!("M00{}",
                verbose
            )
        );
    }

    pub fn end_program(&mut self) {
        self.write_gcode_string(
            format!("M02{}", self.verbose_string(String::from("(End of program)")))
        )
    }

    pub fn end_program2(&mut self) {
        self.write_gcode_string(
            format!("M30{}", self.verbose_string(String::from("(End of program)")))
        )
    }

    pub fn reset_program_and_end(&mut self) {
        self.set_spindle_off();
        self.go_home();
        self.program_stop();
        self.end_program();
        self.end_program2();
    }

    // MARK: 3d printed functions

    pub fn start_extruding_heat(&mut self) {
        self.write_gcode_string(
            format!("M104{}", self.verbose_string(String::from(" (Start extruder heating.)")))
        )
    }

    pub fn wait_until_extruder_reaches(&mut self, to: f64) {
        // TODO: Add in the to function
        self.write_gcode_string(
            format!("M109{}", self.verbose_string(format!(" (Wait until extruder reaches to {}.)", to)))
        )
    }

    pub fn start_bed_heat(&mut self) {
        self.write_gcode_string(
            format!("M190{}", self.verbose_string(String::from(" (Start bed heating.)")))
        )
    }

    pub fn wait_until_bed_reaches(&mut self, to: f64) {
        // TODO: Add in the to function
        self.write_gcode_string(
            format!("M106{}", self.verbose_string(format!(" (Wait until bed reaches to {}.)", to)))
        )
    }

    pub fn set_fan_speed(&mut self, speed: f64) {
        // TODO: Add in the speed function
        self.write_gcode_string(
            format!("M106{}", self.verbose_string(format!(" (Set fan speed to {}.)", speed)))
        )
    }

    pub fn set_spindle_on(&mut self, counter_clockwise: bool, speed: f64) {
        self.spindle_clock_speed = speed;
        self.spindle_state = if counter_clockwise {
            SpindleState::CounterClockwise
        } else {
            SpindleState::Clockwise
        };

        self.write_gcode_string(
            format!(
                "S{} M0{}{}", speed, if counter_clockwise { 4 } else { 3 },
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
        self.write_gcode_string(
            format!("M05{}", self.verbose_string(String::from(" (Turn off the spindle.)")))
        )
    }

    pub fn set_accuracy_control(&mut self, smoothness: Smoothness) {
        self.write_gcode_string(
            format!("G187 {}{}", smoothness,
                self.verbose_string(
                    format!(" (Set accuracy to {}.)", smoothness.description())))
        )
    }

    pub fn set_tool_and_go_home(&mut self, tool_index: usize) {
        self.current_tool_index = tool_index;
        self.go_home();
        self.write_gcode_str("");
        self.write_gcode_string(
            // format!("M06{}", self.verbose_string(String::from(" (Tool change. Not yet implemented.)")))
            format!("N{} T{} M6{}", tool_index+1, self.tools[tool_index].index_in_machine,
                self.verbose_string(String::from(" (Tool change.)")))
                // self.verbose_string(String::from(" (Tool change. Not yet implemented.)")))
        );
        self.set_accuracy_control(self.tools[tool_index].smoothness);
        self.set_tool_offset_positive(
            self.tools[tool_index].index_in_machine,
            self.tools[tool_index].offset_length
        );
        // self.write_gcode_string(format!("G54"));
    }

    pub fn get_tools(&self) -> &Vec<Tool> {
        &self.tools
    }

    pub fn set_flood_colant(&mut self, is_on : bool) {
        self.is_flood_colant_on = is_on;
        self.write_gcode_string(
            format!("M0{}{}", if is_on { 8 } else { 9 }, self.verbose_string(
                format!("Set flood colant {}.", if is_on { "on" } else { "off" })
            ))
        )
    }

    // Non cutting movement
    pub fn move_to_coordinate_rapid(&mut self, pos: &Coordinate) {
        self.pos = *pos;
        self.write_gcode_string(
            format!("G00 X{} Y{} Z{}{}",
                self.pos.x, self.pos.y, self.pos.z,
                self.verbose_str(
                    " (Moves to position specified rapid.)"
                ),
            )
        )
    }

    // Can cut; feed_rate = unit/minute
    pub fn move_to_coordinate(&mut self, pos: &Coordinate,
        feed_rate: f64, can_be_skipped: bool) {
        self.pos = *pos;
        self.write_gcode_string(
            format!("G{} X{} Y{} Z{} F{}{}",
                if can_be_skipped { "31" } else { "01" },
                self.pos.x, self.pos.y, self.pos.z, feed_rate,
                self.verbose_string(
                    String::from(" (Cuts to position specified.)") +
                    if can_be_skipped { " Can be skipped." } else { "" }
                )
            )
        )
    }

    pub fn start_cutting_at(&mut self, pos: &Coordinate, feed_rate: f64, can_be_skipped: bool) {
        // pull out
        self.move_to_coordinate(&Coordinate::from(self.pos.x, self.pos.y, self.home_pos.z),
            feed_rate, can_be_skipped);
        // move over
        self.move_to_coordinate_rapid(&Coordinate::from(pos.x, pos.y, self.home_pos.z));
        // Start Cut
        self.move_to_coordinate(pos,
            feed_rate, can_be_skipped)
    }


    pub fn exact_stop(&mut self, pos: &Coordinate) {
        self.pos = *pos;
        self.write_gcode_string(
            format!("G09 X{} Y{} Z{}", self.pos.x, self.pos.y, self.pos.z)
        )
    }

    pub fn pull_out(&mut self) {
        self.move_to_coordinate(
            &Coordinate::from(self.pos.x, self.pos.y, self.home_pos.z),
            30.0, false
        )
    }

    pub fn go_home(&mut self) {
        // self.referance_pos = self.pos;
        // self.referance_pos.z = self.home_pos.z;
        self.write_gcode_string(
            format!("G00 X{} Y{} Z{}{}",
                self.pos.x, self.pos.y, self.home_pos.z,
                if self.verbose {String::from(" (return home z)")}
                else { String::from("") }
            )
        );
        self.write_gcode_string(
            format!("G00 X{} Y{} Z{}{}",
                self.home_pos.x, self.home_pos.y, self.home_pos.z,
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
        self.write_gcode_string(
            format!("G91 G28 X{} Y{} Z{} G90{}",
                d_pos.x, d_pos.y, d_pos.z,
                if self.verbose {String::from(" (return home through point specified)")}
                else { String::from("") }
            )
        )
    }

    pub fn return_to_referance_position(&mut self, intermediate_pos: Coordinate) {
        self.pos = self.referance_pos;
        self.second_referance_pos = intermediate_pos;
        self.write_gcode_string(
            format!("G29 X{} Y{} Z{}{}",
                intermediate_pos.x, intermediate_pos.y, intermediate_pos.z,
                if self.verbose {String::from(" (return to reference position through point specified.)")}
                else { String::from("") }
            )
        )
    }

    pub fn return_to_second_referance_position(&mut self, intermediate_pos: Coordinate) {
        self.pos = self.second_referance_pos;
        self.write_gcode_string(
            format!("G30 X{} Y{} Z{}{}",
                intermediate_pos.x, intermediate_pos.y, intermediate_pos.z,
                if self.verbose {String::from(" (return to second reference position through point specified.)")}
                else { String::from("") }
            )
        )
    }

    pub fn set_feed_rate_to_units_per_minute(&mut self) {
        self.write_gcode_string(
            format!("G94{}", if self.verbose {String::from(" (Set feed rate to units per minute.)")}
                else {String::new()})
        )
    }

    pub fn set_feed_rate_to_units_per_revelution(&mut self) {
        self.write_gcode_string(
            format!("G95{}", if self.verbose {String::from(" (Set feed rate to units per revelution.)")}
                else {String::new()})
        )
    }

    pub fn set_absolute_mode(&mut self) {
        self.write_gcode_string(
            format!("G90{}", if self.verbose {String::from(" (Set to absolute mode.)")}
                else {String::new()})
        )
    }

    pub fn set_relative_mode(&mut self) {
        self.write_gcode_string(
            String::from(" (G91 changing to relative mode not yet implemented)"),
        )
    }

    pub fn set_polar_coordinates(&mut self) {
        self.write_gcode_string(
            String::from(" (G15 changing to relative mode not yet implemented)"),
        )
    }

    pub fn set_stroke_limit(&mut self, is_on: bool) {
        self.write_gcode_string(
            format!("G2{}{}", 
                if is_on { 2 } else { 3 },
                if self.verbose {
                    format!(" (Stored stroke check {}.)", if is_on { "on" } else { "off" })
                } else { String::new() }
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
            self.write_gcode_string(
                format!("G27{}", self.verbose_str(" (Machine zero position check.)"))
            );
            self.write_gcode_string(
                commands[i].clone()
            );
        }
    }


    pub fn circular_interpolation_offset_midpoint(&mut self,
        is_clock_wise: bool,
        end_pos: &Coordinate, offset: &Coordinate) {
        self.write_gcode_string(
            format!("G0{} X{} Y{} I{} J{}",
                if is_clock_wise {2} else {3},
                end_pos.x, end_pos.y, offset.x, offset.y)
        );
        self.pos = *end_pos;
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

    pub fn circular_interpolation_exact_midpoint_with_radius(
        &mut self,
        is_clock_wise: bool,
        center_pos: &Coordinate, radius: f64, feed_rate: f64
    ) {
        self.pull_out();
        let mut left_pos = *center_pos;
        left_pos.x -= radius;
        let mut right_pos = *center_pos;
        right_pos.x += radius;
        self.move_to_coordinate_rapid(&right_pos);
        self.move_to_coordinate(&right_pos, feed_rate, false);
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
        self.write_gcode_string(
            format!("G04 {}{}", milliseconds, self.verbose_string(
                format!(" (Dewel for {} milliseconds.)", milliseconds)
            ))
        )
    }

    pub fn cancel_cutter_radius_offset(&mut self) {
        self.write_gcode_string(
            format!("G40{}", self.verbose_str(
                " (Cutter radius offset cancel.)"
            ))
        )
    }

    pub fn cancel_cutter_radius_offset_left(&mut self) {
        self.write_gcode_string(
            format!("G41{}", self.verbose_str(
                " (Cutter radius offset left.)"
            ))
        )
    }

    pub fn cancel_cutter_radius_offset_right(&mut self) {
        self.write_gcode_string(
            format!("G42{}", self.verbose_str(
                " (Cutter radius offset right.)"
            ))
        )
    }

    pub fn set_tool_offset_positive(&mut self, tool_index: usize, offset_value: f64) {
        self.write_gcode_string(
            format!("G43 Z{} H{}{}",
                offset_value, tool_index,
                self.verbose_string(format!(" (Set tool offset for tool {}.)", tool_index))
            )
        )
    }

    pub fn set_tool_offset_negative(&mut self, tool_index: usize) {
        self.write_gcode_string(
            format!("G44 H{}{}",
                tool_index,
                self.verbose_string(format!(" (Set tool negative offset for tool {}.)", tool_index))
            )
        )
    }

    pub fn set_tool_offset(&mut self, offset: f64) {
        self.write_gcode_string(
            format!("G43.1 Z{}{}",
                offset,
                self.verbose_string(format!(" (Set current tool offset to be {}.)", offset))
            )
        )
    }

    pub fn cancel_tool_length_compensation(&mut self) {
        self.write_gcode_string(
            format!("G49{}",
                self.verbose_string(format!(" (Cancel tool length compensation.)"))
            )
        )
    }

    pub fn scaling_factor_off(&mut self) {
        self.write_gcode_string(
            format!("G50{}",
                self.verbose_string(format!(" (Cancel scaling factor.)"))
            )
        )
    }

    pub fn scaling_factor_on(&mut self, scaling_pivot: Coordinate, scalor_value: f64) {
        self.write_gcode_string(
            format!("G51 I{} J{} K{} P{}{}",
                scaling_pivot.x, scaling_pivot.y, scaling_pivot.z,
                scalor_value,
                self.verbose_string(format!(" (Cancel scaling factor.)"))
            )
        )
    }

    pub fn local_coordinate_system_set(&mut self, coordinate: Coordinate) {
        self.write_gcode_string(
            format!("G52 X{} Y{} Z{}{}",
                coordinate.x, coordinate.y, coordinate.z,
                self.verbose_string(format!(" (Sets local coordinate system.)"))
            )
        )
    }

    pub fn machine_coordinate_system_set(&mut self, coordinate: Coordinate) {
        self.write_gcode_string(
            format!("G53 X{} Y{} Z{}{}",
                coordinate.x, coordinate.y, coordinate.z,
                self.verbose_string(format!(" (Sets machine coordinate system.)"))
            )
        )
    }

    // work_coordinate is a value in [1:9]
    pub fn set_work_coordinate(&mut self, work_coordinate: usize, coordinate: Coordinate) {
        self.write_gcode_string(
            format!("G{} X{} Y{} Z{}{}",
                if work_coordinate >= 6 {
                    String::from("G59.") + &(work_coordinate-6).to_string()
                } else {
                    (work_coordinate + 53).to_string()
                },
                coordinate.x, coordinate.y, coordinate.z,
                self.verbose_string(format!(" (Sets the work coordinate.)"))
            )
        )
    }

    pub fn single_direction_positioning(&mut self) {
        self.write_gcode_string(
            format!("G60{}",
                self.verbose_string(format!(" (Single direction positioning.)"))
            )
        )
    }

    pub fn turn_on_exact_stop_mode(&mut self) {
        self.write_gcode_string(
            format!("G61{}",
                self.verbose_string(format!(" (Turns on exact stop mode.)"))
            )
        )
    }

    pub fn turn_on_automatic_corner_override_mode(&mut self) {
        self.write_gcode_string(
            format!("G62{}",
                self.verbose_string(format!(" (Automatic corner override mode.)"))
            )
        )
    }

    pub fn tap_make_hole(&mut self, z_distance: f64, spindle_speed: f64) {
        self.write_gcode_string(
            format!("M3 G91 G3 Z{} F{} M4 G32 Z{} G90 {}{}",
                z_distance, spindle_speed, -z_distance,
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

        self.write_gcode_string(format!("G64{}{}",
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
        self.write_gcode_string(
            format!("G68 X{} Y{} R{}{}",
                x, y, degrees,
                if self.verbose {String::from(" (Coordinate system rotating around the x and y axis at r degrees.)")} else {String::new()})
        )
    }

    pub fn coordinate_system_rotation_cancel(&mut self) {
        self.write_gcode_string(
            format!("G69{}",
                if self.verbose {String::from(" (Cancel coordinate system rotating.)")} else {String::new()})
        )
    }

    pub fn high_speed_peck_drilling_cycle(&mut self, pos: Coordinate, margin_depth: f64, depth_of_cut: f64, feed_rate: f64, steps: u64) {
        self.write_gcode_string(
            format!("G73 X{} Y{} Z{} R{} Q{} F{} K{}{}",
                pos.x, pos.y, pos.z, margin_depth, depth_of_cut, feed_rate, steps,
                if self.verbose {String::from(" (High speed peck drilling cycle.)")} else {String::new()})
        )
    }

    // Used to make left handed threads
    pub fn left_hand_threading_cycle(&mut self, pos: Coordinate, depth_of_cut: f64, feed_rate: f64) {
        self.write_gcode_string(
            format!("G74 X{} Y{} Z{} R{} F{}{}",
                pos.x, pos.y, pos.z, depth_of_cut, feed_rate,
                if self.verbose {String::from(" (Left hand threading cycle.")} else {String::new()})
        )
    }

    pub fn fanuc_fine_boring_cycle(&mut self, pos: Coordinate, depth_of_cut: f64, shift_at_hole: f64, dwell_time: f64, feed_rate: f64, steps: u64) {
        self.write_gcode_string(
            format!("G76 X{} Y{} Z{} R{} Q{} P{} F{} K{}{}",
                pos.x, pos.y, pos.z, depth_of_cut, shift_at_hole, dwell_time, feed_rate, steps,
                if self.verbose {String::from(" (Creates hole at position given R depth, Q shift, P time, F feed rate and K steps.)")} else {String::new()})
        )
    }

    pub fn fanuc_boring_cycle(&mut self, pos: Coordinate, starting_position_above_hole: f64, feed_rate: f64, steps: u64) {
        self.write_gcode_string(
            format!("G85 X{} Y{} Z{} R{} F{} K{}{}",
                pos.x, pos.y, pos.z, starting_position_above_hole, feed_rate, steps,
                if self.verbose {String::from(" (Creates hole at position given R depth, Q shift, P time, F feed rate and K steps.)")} else {String::new()})
        )
    }

    // Must use G00 after this
    pub fn fixed_cycle_cancel(&mut self) {
        self.write_gcode_string(
            format!("G80{}",
                if self.verbose {String::from(" (Cancels fixed cycles. Must use G00 after this instruction.)")} else {String::new()})
        )
    }

    pub fn drilling_cycle(&mut self, pos: Coordinate, a: f64, r: f64, l: f64) {
        self.write_gcode_string(
            format!("G81 X{} Y{} Z{} A{} R{} L{}{}",
                pos.x, pos.y, pos.z, a, r, l,
                if self.verbose {String::from(" (Drilling cycle.)")} else {String::new()})
        )
    }

    pub fn stop_drilling_cycle(&mut self) {
        self.write_gcode_string(
            format!("G82{}",
                if self.verbose {String::from(" (Stop drilling cycle.)")} else {String::new()})
        )
    }

    pub fn normal_peck_drilling_canned_cycle(&mut self, e_chip_clean_rpm: Option<f64>,
        feed_rate: f64, i_size_of_first_peck_depth: Option<f64>, j_reduce_peck_depth: Option<f64>,
        k_minimum_depth_of_peck: Option<f64>, l_number_holes: Option<f64>,
        p_pause_at_end_of_last_peck_seconds: Option<f64>, q_peck_depth: Option<f64>,
        r_position_above_the_part: Option<f64>, x: Option<f64>, y: Option<f64>, z: f64) {
        self.write_gcode_string(
            format!("G83{} F{}{}{}{}{}{}{}{}{}{} Z{}{}",
                CNCRouter::<T>::option_param_str(" E", e_chip_clean_rpm),
                feed_rate,
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
        self.write_gcode_string(
            format!("G84 F{}{} S{}{} Z{}{}",
                feed_rate,
                CNCRouter::<T>::option_param_str(" R", r_position_on_r_plane),
                s_rpm,
                CNCRouter::<T>::option_param_str(" X", x_axis_motion),
                z,
                self.verbose_str(" (Right hand threading cycle. Tapping canned cycle.)"),
            )
        )
    }

    pub fn shift_position(&mut self, pos: OptionalCoordinate) {
        self.write_gcode_string(
            format!("G92{}{}{}{}",
                CNCRouter::<T>::option_param_str(" X", pos.x),
                CNCRouter::<T>::option_param_str(" Y", pos.y),
                CNCRouter::<T>::option_param_str(" Z", pos.z),
                self.verbose_str(" (Shift the coordinates over by specified.)"),
            )
        )
    }

    pub fn use_xy_plane(&mut self) {
        self.write_gcode_string(
            format!("G17{}", if self.verbose {String::from(" (Switch to XY plane)")} else {String::new()})
        )
    }


    pub fn use_xz_plane(&mut self) {
        self.write_gcode_string(
            format!("G18{}", if self.verbose {String::from(" (Switch to XZ plane)")} else {String::new()})
        )
    }

    pub fn use_yz_plane(&mut self) {
        self.write_gcode_string(
            format!("G19{}", if self.verbose {String::from(" (Switch to YZ plane)")} else {String::new()})
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
        }
    }

    pub fn is_broad(&self) -> bool {
        if let ToolType::PartialCutBroad = self.tool_type {
            true
        } else if let ToolType::FullCutBroad = self.tool_type {
            true
        } else {
            false
        }
    }

    pub fn is_text(&self) -> bool {
        if let ToolType::Text = self.tool_type {
            true
        } else {
            false
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
            x: Some(0.0),
            y: y,
            z: Some(0.0)
        }
    }

    pub fn from_z(z: Option<f64>) -> OptionalCoordinate {
        OptionalCoordinate {
            x: Some(0.0),
            y: Some(0.0),
            z: z
        }
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
