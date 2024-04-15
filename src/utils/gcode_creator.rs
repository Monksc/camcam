#![allow(dead_code)]
use super::*;
use std::sync::mpsc;

use std::sync::{Arc, Mutex};
use std::thread;

use once_cell::sync::Lazy;
static TOTAL_THREADS: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(8));
static THREADS_IN_USE: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));

pub fn set_threads(mut thread_count: u32) {
    if thread_count < 4 {
        thread_count = 4;
    }
    let mut total_threads = TOTAL_THREADS.lock().unwrap();
    *total_threads = thread_count;
}
pub fn try_get_threads() ->
    std::sync::TryLockResult<std::sync::MutexGuard<'static, u32>>
{
    return THREADS_IN_USE.try_lock();
}

struct ThreadWriter {
    sender: std::sync::mpsc::Sender<String>,
}
impl std::io::Write for ThreadWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        let msg = std::str::from_utf8(buf).unwrap().to_string();
        self.sender.send(msg).unwrap();
        // let msg = std::str::from_utf8(buf).unwrap().to_string();
        // print!("{}", msg);
        return Ok(buf.len());
    }
    fn flush(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
}

pub struct GCodeCreator<T: std::io::Write> {
    cnc_router: cnc_router::CNCRouter<T>,
    start_middle: bool,
    spindle_speed: f64,
    feed_rate: f64,
    z_axis_off_cut: f64,
    depth_of_cut: f64,
}

enum CutBroadSmartPathMethodArguments {
    CanCut(f64, f64),
    MaxY(f64, f64),
    MinY(f64, f64),
}

enum CutBroadSmartPathMethodReturn {
    CanCut(bool),
    MaxY(f64),
    MinY(f64),
}
impl CutBroadSmartPathMethodReturn {
    fn can_cut(&self) -> bool {
        if let CutBroadSmartPathMethodReturn::CanCut(x) = self {
            *x
        } else {
            panic!("Tried to force a CutBroadSmartPathMethodReturn into a CanCut");
        }
    }
    fn max_y(&self) -> f64 {
        if let CutBroadSmartPathMethodReturn::MaxY(x) = self {
            *x
        } else {
            panic!("Tried to force a CutBroadSmartPathMethodReturn into a MaxY");
        }
    }
    fn min_y(&self) -> f64 {
        if let CutBroadSmartPathMethodReturn::MinY(x) = self {
            *x
        } else {
            panic!("Tried to force a CutBroadSmartPathMethodReturn into a MinY");
        }
    }
}

impl<T: std::io::Write> GCodeCreator<T> {
    pub fn from(
        cnc_router: cnc_router::CNCRouter<T>,
        use_inches: bool,
        start_middle: bool,
        spindle_speed: f64,
        feed_rate: f64,
        z_axis_off_cut: f64,
        depth_of_cut: f64,
        name: &str,
        extra_header_message: String,
    ) -> GCodeCreator<T> {
        let mut gc = GCodeCreator {
            cnc_router: cnc_router,
            start_middle: start_middle,
            spindle_speed: spindle_speed,
            feed_rate: feed_rate,
            z_axis_off_cut: z_axis_off_cut,
            depth_of_cut: depth_of_cut,
        };

        gc.cnc_router.generate_header(use_inches, &name, extra_header_message);

        return gc;
    }

    pub fn to_new_write<W: std::io::Write>(&self, w: W) -> GCodeCreator<W> {
        GCodeCreator::<W> {
            cnc_router: self.cnc_router.to_new_write(w),
            start_middle: self.start_middle,
            spindle_speed: self.spindle_speed,
            feed_rate: self.feed_rate,
            z_axis_off_cut: self.z_axis_off_cut,
            depth_of_cut: self.depth_of_cut,
        }
    }

    pub fn is_down(&self, tool_length: f64) -> bool {
        let z = self.cnc_router.get_pos().z + tool_length;
        (z > self.z_axis_off_cut && self.depth_of_cut > 0.0)
            || (z < self.z_axis_off_cut && self.depth_of_cut < 0.0)
    }

    pub fn build_gcode<
        J: lines_and_curves::Intersection + std::fmt::Debug + Clone + cnc_router::CNCPath,
    >(
        &mut self,
        do_cut_on_odd: bool,
        next_path: fn(&mut bit_path::PathItr<f64>) -> bool,
        signs: &mut Vec<sign::Sign<J>>,
    ) {
        let tools = self.cnc_router.get_tools().clone();
        for sign in &mut *signs {
            let mut cuttable_rects = vec![sign.bounding_rect().clone()];
            let mut seen_full_broad = false;

            for (tool_index, tool) in tools.iter().enumerate() {
                if !tool.tool_type().is_broad() {
                    continue;
                }
                let depth_of_cut = self.depth_of_cut;
                let z_axis_off_cut = self.z_axis_off_cut + tool.length;
                if !seen_full_broad {
                    cuttable_rects = vec![sign.bounding_rect().clone()];
                }
                if let cnc_router::ToolType::FullCutBroad(_, _) = tool.tool_type {
                    seen_full_broad = true;
                }
                self.cnc_router
                    .write_gcode_comment(format!("CHANGED TOOL {}", tool_index));

                self.cnc_router.set_tool_and_go_home(
                    tool_index,
                    tool.feed_rate_of_cut,
                    &tool.pre_cut_gcode,
                    tool.force_retouch_off,
                    tool.suggested_length,
                );
                self.cnc_router.set_spindle_on(false, self.spindle_speed);
                let bit_diameter = 2.0 * tool.radius;

                let mut new_cuttable_rects = lines_and_curves::RectangleConnections::from(
                    bit_diameter,
                    bit_diameter,
                    sign.bounding_rect().clone(),
                );

                for cut_rect in cuttable_rects {
                    let sign_width = cut_rect.width(); // sign.bounding_rect().width();
                    let sign_height = cut_rect.height(); // sign.bounding_rect().height();
                    let width = (cut_rect.width() / (bit_diameter * tool.offset)) as usize;
                    let height = (cut_rect.height() / (bit_diameter * tool.offset)) as usize;
                    let min_x = cut_rect.min_x();
                    let min_y = cut_rect.min_y();

                    let mut path = bit_path::Path::from(
                        width,
                        height,
                        Box::from(move |x: usize, y: usize| -> (f64, f64) {
                            (
                                (x as f64 / width as f64) as f64 * sign_width + min_x,
                                (y as f64 / height as f64) as f64 * sign_height + min_y,
                            )
                        }),
                        next_path,
                    );
                    if self.start_middle {
                        path = path.start_middle();
                    }
                    let mut cut_to: Option<lines_and_curves::Point> = None;
                    for (x, y, can_be_down) in path {
                        let seen = sign.y_values_before(x, y);
                        let rect = lines_and_curves::Rectangle::from(
                            lines_and_curves::Point::from(
                                x - bit_diameter / 2.0,
                                y - bit_diameter / 2.0,
                            ),
                            lines_and_curves::Point::from(
                                x + bit_diameter / 2.0,
                                y + bit_diameter / 2.0,
                            ),
                        );

                        let crosses_rect = sign.line_collides_wth_rect(&rect);
                        let is_down =
                            can_be_down && ((seen % 2 == 1) == do_cut_on_odd) && !crosses_rect;
                        if crosses_rect {
                            new_cuttable_rects.add_rect(&lines_and_curves::Point::from(x, y));
                        }

                        if is_down {
                            if let Some(p) = cut_to {
                                assert!(self.is_down(tool.length));
                                if !lines_and_curves::LineSegment::from(
                                    self.cnc_router.get_point(),
                                    lines_and_curves::Point::from(x, y),
                                )
                                .contains_point_endless_line(p)
                                {
                                    new_cuttable_rects
                                        .add_rect(&lines_and_curves::Point::from(p.x, p.y));
                                    self.cnc_router.move_to_optional_coordinate(
                                        &cnc_router::OptionalCoordinate::from(
                                            Some(p.x),
                                            Some(p.y),
                                            None,
                                        ),
                                        Some(tool.feed_rate_of_cut),
                                        false,
                                    );
                                }
                            } else {
                                assert!(!self.is_down(tool.length));
                                // removing this as it adds in unnecessary fillers on paths
                                // with horizontal lines
                                // new_cuttable_rects.add_rect(&lines_and_curves::Point::from(x, y));
                                self.cnc_router.move_to_coordinate_rapid(
                                    &cnc_router::Coordinate::from(x, y, z_axis_off_cut),
                                );
                                self.cnc_router.move_to_optional_coordinate(
                                    &cnc_router::OptionalCoordinate::from_z(Some(
                                        z_axis_off_cut + depth_of_cut,
                                    )),
                                    Some(tool.feed_rate_of_drill),
                                    false,
                                );
                            }

                            cut_to = Some(lines_and_curves::Point::from(x, y));
                        } else {
                            if let Some(p) = cut_to {
                                assert!(self.is_down(tool.length));
                                self.cnc_router.move_to_optional_coordinate(
                                    &cnc_router::OptionalCoordinate::from(
                                        Some(p.x),
                                        Some(p.y),
                                        None,
                                    ),
                                    Some(tool.feed_rate_of_cut),
                                    false,
                                );
                                self.cnc_router.move_to_optional_coordinate(
                                    &cnc_router::OptionalCoordinate::from_z(Some(z_axis_off_cut)),
                                    None,
                                    false,
                                );
                            }
                            cut_to = None;
                        }
                    }

                    if self.is_down(tool.length) {
                        if let Some(p) = cut_to {
                            self.cnc_router.move_to_optional_coordinate(
                                &cnc_router::OptionalCoordinate::from(Some(p.x), Some(p.y), None),
                                Some(tool.feed_rate_of_cut),
                                false,
                            );
                        }
                    }
                    self.cnc_router.move_to_optional_coordinate(
                        &cnc_router::OptionalCoordinate::from_z(Some(z_axis_off_cut)),
                        Some(tool.feed_rate_of_cut),
                        false,
                    );
                }

                let mut new_rects = Vec::new();
                new_cuttable_rects.to_bigger_rect_iter();
                new_rects.extend(new_cuttable_rects);
                cuttable_rects = new_rects;

                self.cnc_router.set_spindle_off();
            }
        }

        self.cnc_router.write_gcode_comment_str("Following Shapes");
        for tool_type in vec![
            cnc_router::ToolType::full_text(),
            cnc_router::ToolType::PartialContourAngle(0.0, 0.0, cnc_router::ShapeType::all()),
            cnc_router::ToolType::full_braille(),
        ] {
            for (tool_index, tool) in tools.iter().enumerate() {
                if tool_type.raw_value() != tool.tool_type.raw_value() {
                    continue;
                }

                let z_axis_off_cut = self.z_axis_off_cut + tool.length;
                self.cnc_router.set_tool_and_go_home(
                    tool_index,
                    tool.feed_rate_of_cut,
                    &tool.pre_cut_gcode,
                    tool.force_retouch_off,
                    tool.suggested_length,
                );
                self.cnc_router.set_spindle_on(false, self.spindle_speed);
                for sign in &mut *signs {
                    for shape in sign.shapes() {
                        let mut first_line = true;
                        for line in shape.lines() {
                            if first_line || !line.is_connected() {
                                first_line = false;
                                let Some(point) = line.start_path() else {
                                    continue;
                                };
                                self.cnc_router.move_to_coordinate_rapid(
                                    &cnc_router::Coordinate::from(point.x, point.y, z_axis_off_cut),
                                );
                                self.cnc_router.move_to_coordinate(
                                    &cnc_router::Coordinate::from(
                                        point.x,
                                        point.y,
                                        z_axis_off_cut + self.depth_of_cut,
                                    ),
                                    Some(tool.feed_rate_of_drill),
                                    false,
                                );
                            }
                            line.follow_path(&mut self.cnc_router, Some(tool.feed_rate_of_cut));
                            if !line.is_connected() {
                                self.cnc_router.move_to_optional_coordinate(
                                    &cnc_router::OptionalCoordinate::from_z(Some(z_axis_off_cut)),
                                    Some(tool.feed_rate_of_drill),
                                    false,
                                );
                            }
                        }
                        if self.is_down(tool.length) {
                            self.cnc_router.move_to_optional_coordinate(
                                &cnc_router::OptionalCoordinate::from_z(Some(z_axis_off_cut)),
                                Some(tool.feed_rate_of_drill),
                                false,
                            );
                        }
                    }
                }
                self.cnc_router.set_spindle_off();
            }
        }

        self.cnc_router.go_home();
        self.cnc_router.set_spindle_off();
        self.cnc_router.reset_program_and_end();
    }

    pub fn cut_broad_rect<
        J: lines_and_curves::Intersection + std::fmt::Debug + Clone + cnc_router::CNCPath,
    >(
        &mut self,
        do_cut_on_odd: bool,
        sign: &mut sign::Sign<J>,
        tool: &cnc_router::Tool,
        rect: &lines_and_curves::Rectangle,
        range_map: &mut range_map::FillRect,
    ) -> bool {
        let mut did_update = false;
        let z_axis_off_cut = self.z_axis_off_cut + tool.length;

        let increment = 2.0 * tool.radius * tool.offset;
        let mut first_time = true;
        let mut is_going_up = true;
        for x in float_loop(
            rect.min_x() + tool.radius,
            rect.max_x() - tool.radius,
            increment,
            Vec::new(),
        ) {
            if first_time {
                let mut y = rect.min_y() + tool.radius;
                if (sign.y_values_before(x, y) % 2 == 0) == do_cut_on_odd {
                    let mut assigned_y = false;
                    for i in 0..5 {
                        let (new_y, new_is_going_up) = match i {
                            1 => (
                                sign.get_next_y_value_bounds(x, rect.min_y()) + tool.radius,
                                true,
                            ),
                            2 => (
                                sign.get_next_y_value_bounds(x, rect.min_y()) - tool.radius,
                                false,
                            ),
                            3 => (
                                sign.get_prev_y_value_bounds(x, rect.min_y()) + tool.radius,
                                true,
                            ),
                            4 => (
                                sign.get_prev_y_value_bounds(x, rect.min_y()) - tool.radius,
                                false,
                            ),
                            5 => (
                                sign.get_next_y_value_bounds(x, rect.max_y()) + tool.radius,
                                true,
                            ),
                            6 => (
                                sign.get_next_y_value_bounds(x, rect.max_y()) - tool.radius,
                                false,
                            ),
                            7 => (
                                sign.get_prev_y_value_bounds(x, rect.max_y()) + tool.radius,
                                true,
                            ),
                            8 => (
                                sign.get_prev_y_value_bounds(x, rect.max_y()) - tool.radius,
                                false,
                            ),
                            _ => (y, is_going_up),
                        };
                        if (sign.y_values_before(x, y) % 2 == 0) != do_cut_on_odd {
                            y = new_y;
                            is_going_up = new_is_going_up;
                            assigned_y = true;
                            break;
                        }
                    }

                    if !assigned_y {
                        return did_update;
                    }
                }
                self.cnc_router
                    .move_to_coordinate_rapid(&cnc_router::Coordinate::from(x, y, z_axis_off_cut));
                self.cnc_router.move_to_optional_coordinate(
                    &cnc_router::OptionalCoordinate::from_z(Some(
                        z_axis_off_cut + self.depth_of_cut,
                    )),
                    Some(tool.feed_rate_of_drill),
                    false,
                );
                first_time = false;
            } else {
                // follow path tool.radius to left/right
                if (sign.y_values_before(x, self.cnc_router.get_pos().y) % 2 == 0) == do_cut_on_odd
                {
                    self.cnc_router.move_to_optional_coordinate(
                        &cnc_router::OptionalCoordinate::from_z(Some(z_axis_off_cut)),
                        Some(tool.feed_rate_of_cut),
                        false,
                    );
                    return did_update;
                }
                self.cnc_router.move_to_optional_coordinate(
                    &cnc_router::OptionalCoordinate::from_x(Some(x)),
                    Some(tool.feed_rate_of_cut),
                    false,
                );
            }

            let mut y = if is_going_up {
                sign.get_next_y_value_bounds(x, self.cnc_router.get_pos().y) - tool.radius
            } else {
                sign.get_prev_y_value_bounds(x, self.cnc_router.get_pos().y) + tool.radius
            };
            if y < rect.min_y() {
                y = rect.min_y() + tool.radius;
            } else if y > rect.max_y() {
                y = rect.max_y() - tool.radius;
            }

            range_map.fill_rect(
                x - tool.radius,
                self.cnc_router.get_pos().y,
                x + tool.radius,
                y,
            );
            did_update = true;

            if (sign.y_values_before(x, y) % 2 == 0) == do_cut_on_odd {
                self.cnc_router.move_to_optional_coordinate(
                    &cnc_router::OptionalCoordinate::from_z(Some(z_axis_off_cut)),
                    Some(tool.feed_rate_of_cut),
                    false,
                );
                return did_update;
            }
            self.cnc_router.move_to_optional_coordinate(
                &cnc_router::OptionalCoordinate::from_y(Some(y)),
                Some(tool.feed_rate_of_cut),
                false,
            );

            is_going_up = !is_going_up;
        }

        self.cnc_router.move_to_optional_coordinate(
            &cnc_router::OptionalCoordinate::from_z(Some(z_axis_off_cut)),
            Some(tool.feed_rate_of_cut),
            false,
        );

        return did_update;
    }

    pub fn broad_smart_path<
        J: lines_and_curves::Intersection + std::fmt::Debug + Clone + cnc_router::CNCPath,
    >(
        &mut self,
        do_cut_on_odd: bool,
        sign: sign::Sign<J>,
        tool: &cnc_router::Tool,
    ) {
        let mut sign = sign;

        let mut fill_rect = range_map::FillRect::from(
            sign.bounding_rect().min_x(),
            sign.bounding_rect().min_y(),
            sign.bounding_rect().max_x(),
            sign.bounding_rect().max_y(),
        );

        let mut cleared = false;
        while !cleared {
            cleared = true;
            for (min_x, min_y, max_x, max_y) in fill_rect.get_open_rects() {
                if (max_x - min_x) * (max_y - min_y) < 0.1 {
                    continue;
                }
                let rect = lines_and_curves::Rectangle::from(
                    lines_and_curves::Point::from(min_x, min_y),
                    lines_and_curves::Point::from(max_x, max_y),
                );
                cleared = false;
                if !self.cut_broad_rect(do_cut_on_odd, &mut sign, tool, &rect, &mut fill_rect) {
                    fill_rect.fill_rect(min_x, min_y, max_x, max_y);
                }
            }
        }
    }

    fn build_gcode_smart_path_helepr<
        J: lines_and_curves::Intersection + std::fmt::Debug + Clone + cnc_router::CNCPath,
    >(
        &mut self,
        do_cut_on_odd: bool,
        signs: Vec<sign::Sign<J>>,
        add_padding_to: Vec<(cnc_router::ShapeType, f64)>,
        tool_index: usize,
        tool: cnc_router::Tool,
        thinnest_radius_seen: f64,
    ) {
        use std::time::Instant;
        let tool_time = Instant::now();

        self.cnc_router.set_tool_and_go_home(
            tool_index,
            tool.feed_rate_of_cut,
            &tool.pre_cut_gcode,
            tool.force_retouch_off,
            tool.suggested_length,
        );
        self.cnc_router.set_spindle_on(false, self.spindle_speed);
        self.cnc_router.turn_fan(true);

        if tool.tool_type().is_broad() {
            for sign in signs {
                let mut sign = sign;
                let increment = 2.0 * tool.radius * tool.offset;
                sign.add_xs_layers(sign.bounding_rect().min_x(), increment);

                let mut fill_rect = range_map::FillRect::from(
                    sign.bounding_rect().min_x(),
                    sign.bounding_rect().min_y(),
                    sign.bounding_rect().max_x(),
                    sign.bounding_rect().max_y(),
                );

                let mut thick_sign = sign.clone();
                if let cnc_router::ToolType::SpaceBetweenCutBroad(
                    override_thinest_radius,
                    shrink_by_radius,
                    smaller_grow_by_radius,
                ) = tool.tool_type()
                {
                    // let increment = 2.0 * tool.radius * tool.offset;
                    let bigger_radius = if override_thinest_radius == 0.0 {
                        thinnest_radius_seen
                    } else {
                        override_thinest_radius
                    }; // + 8.0 * increment;

                    thick_sign =
                        thick_sign.expand_lines(bigger_radius, do_cut_on_odd, &add_padding_to);
                    thick_sign = thick_sign.expand_lines(
                        // tool.radius - bigger_radius - 1.1 * tool.radius * tool.offset,
                        // do_cut_on_odd,
                        -(bigger_radius - tool.radius + shrink_by_radius),
                        do_cut_on_odd,
                        &Vec::new(),
                    );

                    if smaller_grow_by_radius != 0.0 {
                        sign =
                            sign.expand_lines(smaller_grow_by_radius, do_cut_on_odd, &Vec::new());
                    }
                }

                let mut zero_range = range_map::FillRect::from(
                    sign.bounding_rect().min_x(),
                    sign.bounding_rect().min_y(),
                    sign.bounding_rect().max_x(),
                    sign.bounding_rect().max_y(),
                );

                self.broad_smart_path2(
                    do_cut_on_odd,
                    &mut sign.expand_lines(tool.radius, do_cut_on_odd, &add_padding_to),
                    if let cnc_router::ToolType::SpaceBetweenCutBroad(_, _, _) = tool.tool_type() {
                        Some(&mut thick_sign)
                    } else {
                        None
                    },
                    &tool,
                    if let cnc_router::ToolType::SpaceBetweenCutBroad(_, _, _) = tool.tool_type() {
                        &mut zero_range
                    } else {
                        &mut fill_rect
                    },
                    !tool.tool_type().full_cut(),
                );
            }
        } else if tool.tool_type().is_text_or_braille() {
            self.cut_text(do_cut_on_odd, &signs, &add_padding_to, &tool, &mut None);
        }
        self.cnc_router.set_spindle_off();
        self.cnc_router.turn_fan(false);
        self.cnc_router.force_flush_gcode();

        eprintln!(
            "Tool: {}, \tRadius: {}, \tTime: {:?}",
            tool.name,
            tool.radius,
            tool_time.elapsed(),
        );
    }

    pub fn build_gcode_smart_path<
        J: lines_and_curves::Intersection
            + std::fmt::Debug
            + Clone
            + cnc_router::CNCPath
            + Sync
            + Send
            + 'static,
    >(
        &mut self,
        do_cut_on_odd: bool,
        signs: &Vec<sign::Sign<J>>,
        add_padding_to: &Vec<(cnc_router::ShapeType, f64)>,
    ) {
        use std::time::Instant;
        let begining = Instant::now();

        let mut fill_rects = Vec::new();

        for sign in signs {
            fill_rects.push(range_map::FillRect::from(
                sign.bounding_rect().min_x(),
                sign.bounding_rect().min_y(),
                sign.bounding_rect().max_x(),
                sign.bounding_rect().max_y(),
            ));
        }

        let tools = self.cnc_router.get_tools().clone();
        let mut handlers = Vec::new();

        let mut thinnest_radius_seen = 10.0;
        for (tool_index, tool) in tools.iter().enumerate() {
            let (tx, rx) = mpsc::channel();

            {
                let mut copy_self = self.to_new_write(ThreadWriter { sender: tx });
                let tool = tool.clone();
                let add_padding_to = add_padding_to.clone();
                let signs = signs.clone();

                // Make sure we have empty threads
                let mut times_looped = 0;
                loop {
                    {
                        let total_threads = TOTAL_THREADS.lock().unwrap();
                        let mut threads_in_use = THREADS_IN_USE.lock().unwrap();

                        if *threads_in_use < *total_threads {
                            *threads_in_use += 1;
                            break;
                        }
                    }
                    times_looped += 1;

                    use rand::prelude::*;
                    let mut rng = rand::thread_rng();
                    let r = rng.gen::<f64>();
                    thread::sleep(std::time::Duration::from_millis(
                        (250. * r * if times_looped < 3 {
                            1
                        } else {
                            1 << if times_looped > 8 { 8 } else { times_looped }
                        } as f64) as u64
                    ));
                }

                let handle = thread::spawn(move || {
                    copy_self.build_gcode_smart_path_helepr(
                        do_cut_on_odd,
                        signs,
                        add_padding_to,
                        tool_index,
                        tool,
                        thinnest_radius_seen,
                    );
                    let mut threads_in_use = THREADS_IN_USE.lock().unwrap();
                    *threads_in_use -= 1;
                });
                handlers.push((handle, rx));
            }
            if tool.tool_type().is_broad() {
                if thinnest_radius_seen > tool.radius {
                    thinnest_radius_seen = tool.radius;
                }
            }
        }

        for (handle, rx) in handlers {
            handle.join().unwrap();
            for line in rx {
                self.cnc_router.write_gcode_string_no_line(line);
            }
        }

        // self.cnc_router.reset_program_and_end();
    }

    // MARK: Smart method 2

    fn cut_broad_smart_path2<
        J: lines_and_curves::Intersection + std::fmt::Debug + Clone + cnc_router::CNCPath,
    >(
        &mut self,
        // do_cut_on_odd: bool,
        sign: &mut sign::Sign<J>,
        tool: &cnc_router::Tool,
        fill_rect: &mut range_map::FillRect,
        x: f64,
        y: f64,
        increment: f64,
        only_cleanup: bool,
        methods: &mut Box<
            impl FnMut(CutBroadSmartPathMethodArguments) -> CutBroadSmartPathMethodReturn,
        >,
    ) {
        let too_far_x = if let Some(dx) = tool.extra_distance_x() {
            Some(x + dx)
        } else {
            None
        };

        let mut x = x;
        let mut y = y;

        // let y_before = sign.y_values_before(x, y);
        if
        // ((y_before % 2 == 1) != do_cut_on_odd) ||
        !methods(CutBroadSmartPathMethodArguments::CanCut(x, y)).can_cut()
        // (only_cleanup && sign.y_values_before(x, y) <= 1)
        // (only_cleanup &&
        //     (
        //     (sign.bounding_rect().min_x() - x).abs() < 5.0 * tool.radius ||
        //     (sign.bounding_rect().min_y() - y).abs() < 5.0 * tool.radius ||
        //     (sign.bounding_rect().max_x() - x).abs() < 5.0 * tool.radius ||
        //     (sign.bounding_rect().max_y() - y).abs() < 5.0 * tool.radius ||

        //     (sign.bounding_rect().min_x() - x).abs() < 0.5 ||
        //     (sign.bounding_rect().min_y() - y).abs() < 0.5 ||
        //     (sign.bounding_rect().max_x() - x).abs() < 0.5 ||
        //     (sign.bounding_rect().max_y() - y).abs() < 0.5 ||
        //     if let Some((distance, _, _)) = sign.closest_shape(
        //         &lines_and_curves::Point::from(x, y)
        //     ) {
        //         distance > 0.0 &&
        //             distance < 20.0 * tool.radius &&
        //             distance < 0.5
        //     } else {
        //         true
        //     }
        //  ))
        {
            return;
        }

        let z_axis_off_cut = self.z_axis_off_cut + tool.length;

        let mut moved = false;
        let mut prev_i_move = 9999;
        'main: loop {
            let prev_x = x;
            let prev_y = y;
            let mut found_new_value = false;
            for i in 0..3 {
                if i == prev_i_move && i == 2 {
                    continue;
                }
                if i >= 2 && !moved {
                    continue;
                }
                if i == 2 {
                    if !moved {
                        continue;
                    }
                    if let Some((distance, _, shape)) =
                        sign.closest_shape(&lines_and_curves::Point::from(
                            self.cnc_router.get_pos().x,
                            self.cnc_router.get_pos().y,
                        ))
                    {
                        if distance > 0.1 {
                            continue;
                        }

                        cnc_router::CNCPath::cut_till::<T>(
                            &shape.lines(),
                            Some(
                                (self.cnc_router.get_pos().x + increment)
                                    .min(sign.bounding_rect().max_x()),
                            ),
                            None,
                            &mut self.cnc_router,
                            Some(tool.feed_rate_of_cut),
                            false,
                            tool.feed_rate_of_drill,
                            self.z_axis_off_cut,
                            self.depth_of_cut,
                            &tool.tool_type(),
                            tool.radius,
                            tool.offset,
                            false,
                            Box::from(|x, y| {
                                methods(CutBroadSmartPathMethodArguments::CanCut(x, y)).can_cut()
                            }),
                        );
                        let pos = self.cnc_router.get_pos();
                        x = pos.x;
                        y = pos.y;
                        prev_i_move = i;
                        continue 'main;
                    }
                    continue;
                }

                let (new_x, new_y) = match i {
                    // 0 => (x, sign.get_next_y_value(x, y+0.0001).unwrap_or(max_y(x, y))),
                    // 1 => (x, sign.get_prev_y_value(x, y-0.0001).unwrap_or(min_y(x, y))),
                    0 => (
                        x,
                        methods(CutBroadSmartPathMethodArguments::MaxY(x, y + 0.000001)).max_y(),
                    ),
                    1 => (
                        x,
                        methods(CutBroadSmartPathMethodArguments::MinY(x, y - 0.000001)).min_y(),
                    ),
                    3 => (x + increment, y),
                    4 => (x - increment, y),
                    5 => (x, y + increment),
                    _ => (x, y - increment),
                };

                if (
                        // ((x - new_x).abs() + (y - new_y).abs() + 0.0001) >= increment
                        ((x - new_x).abs() + (y - new_y).abs()) >= increment / 2.0
                    ) &&
                    // sign.sees_even_odd_lines_before(new_x, new_y, do_cut_on_odd, true) &&
                    // sign.sees_even_odd_lines_before((new_x+x)/2.0, (new_y+y)/2.0, do_cut_on_odd, true) &&
                    // (
                    //     (sign.y_values_before(new_x, new_y) % 2 == 1)
                    //     == do_cut_on_odd
                    // ) &&
                    !sign.line_collides_wth_rect(
                        &lines_and_curves::Rectangle::from_rect_add_radius(
                            &lines_and_curves::Rectangle::from(
                                lines_and_curves::Point::from(x, y),
                                lines_and_curves::Point::from(new_x, new_y),
                            ),
                            -0.0001 // so when it touches it appear like it doesnt touch
                            // 0.0
                        )
                    ) &&
                    !fill_rect.is_fill_padding(
                        x, y,
                        new_x, new_y,
                        increment / 2.0, increment / 2.0,
                    ) &&
                    // (
                    //     !only_cleanup ||
                    //     lines_and_curves::Point::from(x, y).distance_to(
                    //         &lines_and_curves::Point::from(new_x, new_y)
                    //     ) < 25.0 * tool.radius
                    // ) &&
                    methods(CutBroadSmartPathMethodArguments::CanCut(new_x, new_y)).can_cut() &&
                    methods(CutBroadSmartPathMethodArguments::CanCut(
                        (new_x+x)/2.0,
                        (new_y+y)/2.0,
                    )).can_cut() &&
                    if let Some(max_x) = too_far_x {
                        new_x <= max_x
                    } else {
                        true
                    }
                {
                    x = new_x;
                    y = new_y;
                    found_new_value = true;
                    prev_i_move = i;
                    break;
                }
            }

            if !found_new_value {
                if moved {
                    self.cnc_router.move_to_optional_coordinate(
                        &cnc_router::OptionalCoordinate::from_z(Some(z_axis_off_cut)),
                        Some(tool.feed_rate_of_drill),
                        false,
                    );
                }
                return;
            }

            if !moved {
                self.cnc_router
                    .move_to_coordinate_rapid(&cnc_router::Coordinate::from(
                        prev_x,
                        prev_y,
                        z_axis_off_cut,
                    ));
                self.cnc_router.move_to_optional_coordinate(
                    &cnc_router::OptionalCoordinate::from_z(Some(
                        z_axis_off_cut + self.depth_of_cut,
                    )),
                    Some(tool.feed_rate_of_drill),
                    false,
                );
                moved = true;
            }

            let min_x = if prev_x < x { prev_x } else { x };
            let max_x = if prev_x > x { prev_x } else { x };
            let min_y = if prev_y < y { prev_y } else { y };
            let max_y = if prev_y > y { prev_y } else { y };

            let epsilon = 0.0000000001;

            fill_rect.fill_rect(
                // min_x-tool.radius, min_y-tool.radius,
                // max_x+tool.radius, max_y+tool.radius,
                min_x - increment + epsilon,
                min_y - increment + epsilon,
                max_x + increment - epsilon,
                max_y + increment - epsilon,
            );
            self.cnc_router.move_to_optional_coordinate(
                &cnc_router::OptionalCoordinate::from(Some(x), Some(y), None),
                Some(tool.feed_rate_of_cut),
                false,
            );
        }
    }

    pub fn broad_smart_path2<
        J: lines_and_curves::Intersection + std::fmt::Debug + Clone + cnc_router::CNCPath,
    >(
        &mut self,
        do_cut_on_odd: bool,
        mut sign: &mut sign::Sign<J>,
        mut bigger_sign: Option<&mut sign::Sign<J>>,
        tool: &cnc_router::Tool,
        mut fill_rect: &mut range_map::FillRect,
        only_cleanup: bool,
    ) {
        let increment = 2.0 * tool.radius * tool.offset;
        let bounding_rect = sign.bounding_rect().clone();

        let mut sign_clone = sign.clone();

        if let Some(bigger_sign) = &mut bigger_sign {
            self.broad_smart_path2_helper(
                do_cut_on_odd,
                &mut sign_clone,
                Some(&mut bigger_sign.clone()),
                &tool,
                &mut fill_rect,
                only_cleanup,
                Box::from(|args| match args {
                    CutBroadSmartPathMethodArguments::CanCut(x, y) => {
                        if x <= sign.bounding_rect().min_x() + 2.0 * tool.radius + 0.01
                            || x >= sign.bounding_rect().max_x() - 2.0 * tool.radius - 0.01
                            || y <= sign.bounding_rect().min_y() + 2.0 * tool.radius + 0.01
                            || y >= sign.bounding_rect().max_y() - 2.0 * tool.radius - 0.01
                        {
                            return CutBroadSmartPathMethodReturn::CanCut(false);
                        }

                        let value = sign.sees_even_odd_lines_before(x, y, do_cut_on_odd, true)
                            && bigger_sign.sees_even_odd_lines_before(x, y, !do_cut_on_odd, false);
                        CutBroadSmartPathMethodReturn::CanCut(value)
                    }
                    CutBroadSmartPathMethodArguments::MaxY(x, y) => {
                        let next_bigger =
                            bigger_sign.get_next_y_value_bounds(x, y) - increment / 2.0;
                        let next_sign = sign.get_next_y_value_bounds(x, y) - increment / 2.0;
                        CutBroadSmartPathMethodReturn::MaxY(next_bigger.min(next_sign).max(y))
                    }
                    CutBroadSmartPathMethodArguments::MinY(x, y) => {
                        let next_bigger =
                            bigger_sign.get_prev_y_value_bounds(x, y) + increment / 2.0;
                        let next_sign = sign.get_prev_y_value_bounds(x, y) + increment / 2.0;
                        CutBroadSmartPathMethodReturn::MinY(next_bigger.max(next_sign).min(y))
                    }
                }),
            );
        } else {
            self.broad_smart_path2_helper(
                do_cut_on_odd,
                &mut sign_clone,
                bigger_sign,
                &tool,
                &mut fill_rect,
                only_cleanup,
                Box::from(|args| match args {
                    CutBroadSmartPathMethodArguments::CanCut(x, y) => {
                        CutBroadSmartPathMethodReturn::CanCut(sign.sees_even_odd_lines_before(
                            x,
                            y,
                            do_cut_on_odd,
                            true,
                        ))
                    }
                    CutBroadSmartPathMethodArguments::MaxY(x, y) => {
                        CutBroadSmartPathMethodReturn::MaxY(sign.get_next_y_value_bounds(x, y))
                    }
                    CutBroadSmartPathMethodArguments::MinY(x, y) => {
                        CutBroadSmartPathMethodReturn::MinY(sign.get_prev_y_value_bounds(x, y))
                    }
                }),
            );
        };
    }

    fn broad_smart_path2_helper<
        J: lines_and_curves::Intersection + std::fmt::Debug + Clone + cnc_router::CNCPath,
    >(
        &mut self,
        do_cut_on_odd: bool,
        sign: &mut sign::Sign<J>,
        mut bigger_sign: Option<&mut sign::Sign<J>>,
        tool: &cnc_router::Tool,
        mut fill_rect: &mut range_map::FillRect,
        only_cleanup: bool,
        mut methods: Box<
            impl FnMut(CutBroadSmartPathMethodArguments) -> CutBroadSmartPathMethodReturn,
        >,
    ) {
        let mut sign_clone = sign.clone();
        let increment = 2.0 * tool.radius * tool.offset;
        let bounding_rect = sign.bounding_rect().clone();

        let mut pockets = Vec::new();

        for x in float_loop(
            bounding_rect.min_x(),
            bounding_rect.max_x(),
            increment,
            if only_cleanup {
                sign.get_significant_xs()
            } else {
                Vec::new()
            },
        ) {
            for y in
                // float_loop(
                //     bounding_rect.min_y(),
                //     bounding_rect.max_y(),
                //     increment,
                //     Vec::new()
                // )
                sign
                    .get_y_values(x)
                    .iter()
                    .chain(
                        if let Some(ys) = fill_rect.get_ys(x) {
                            ys.clone()
                        } else {
                            vec![]
                        }
                        .iter(),
                    )
                    .chain(
                        if let Some(bigger_sign) = &mut bigger_sign {
                            bigger_sign.get_y_values(x)
                        } else {
                            vec![]
                        }
                        .iter(),
                    )
                    // .map(|y| vec![*y, *y-0.1 * increment, *y+0.1 * increment])
                    .map(|y| *y)
                    // .flatten()
                    .collect::<Vec<f64>>()
            {
                if methods(CutBroadSmartPathMethodArguments::CanCut(x, y)).can_cut() {
                    pockets.push(lines_and_curves::Point::from(x, y));
                }
            }
        }

        if only_cleanup {
            while !pockets.is_empty() {
                let mut index = 0;
                let mut current_point = self.cnc_router.get_point();
                current_point.x -= 1.0;
                current_point.y -= 0.5;
                let mut best_distance = current_point.distance_to(&pockets[0]);
                for i in 1..pockets.len() {
                    let distance = current_point.distance_to(&pockets[i]);
                    if distance < best_distance {
                        best_distance = distance;
                        index = i;
                    }
                }

                let x = pockets[index].x;
                let y = pockets[index].y;
                self.cut_broad_smart_path2(
                    // do_cut_on_odd,
                    &mut sign_clone,
                    &tool,
                    &mut fill_rect,
                    x,
                    y,
                    increment,
                    only_cleanup,
                    &mut methods,
                );
                pockets.swap_remove(index);
            }
        } else {
            let mut seen_shapes = vec![std::collections::HashSet::new()];
            let signs = vec![sign.clone()];
            let should_contour = if let cnc_router::ToolType::FullCutBroad(_, is_contour) = tool.tool_type() {
                is_contour
            } else {
                false
            };
            for point in &pockets {
                let x = point.x;
                let y = point.y;
                if should_contour {
                    self.cut_text(
                        do_cut_on_odd,
                        &signs,
                        &vec![],
                        &cnc_router::Tool {
                            radius: 0.0,
                            tool_type: cnc_router::ToolType::full_contour_all(),
                            offset: 1.0,
                            pre_cut_gcode: String::from(""),
                            force_retouch_off: false,
                            ..tool.clone()
                        },
                        &mut Some((&mut seen_shapes, x))
                    );
                }

                self.cut_broad_smart_path2(
                    &mut sign_clone,
                    &tool,
                    &mut fill_rect,
                    x,
                    y,
                    increment,
                    only_cleanup,
                    &mut methods,
                );
            }
        }
    }

    fn cut_text<
        J: lines_and_curves::Intersection + std::fmt::Debug + Clone + cnc_router::CNCPath,
    >(
        &mut self,
        do_cut_on_odd: bool,
        signs: &Vec<sign::Sign<J>>,
        add_padding_to: &Vec<(cnc_router::ShapeType, f64)>,
        tool: &cnc_router::Tool,
        mut dont_cut: &mut Option<(&mut Vec<std::collections::HashSet<usize>>, f64)>
    ) {
        for (sign_index, original_sign) in signs.iter().enumerate() {
            let mut sign = if let Some(_) = dont_cut {
                original_sign.clone()
            } else {
                original_sign.expand_lines(tool.radius, do_cut_on_odd, add_padding_to)
            };

            if let cnc_router::ToolType::FullContour(_, shrink_by) = tool.tool_type {
                if shrink_by != 0.0 {
                    sign = sign.expand_lines(-shrink_by, do_cut_on_odd, &Vec::new());
                }
            }
            let shapes = sign.shapes_cut_inside(do_cut_on_odd);
            for (shape_index, (shape, cut_inside)) in shapes.iter().enumerate() {
                let cut_inside = *cut_inside;
                if let Some(dont_cut) = &mut dont_cut {
                    if let Some(bounding_box) = shape.bounding_box() {
                        if bounding_box.max_x() > dont_cut.1 {
                            continue;
                        }
                    }

                    if dont_cut.0[sign_index].contains(&shape_index) {
                        continue;
                    }
                    dont_cut.0[sign_index].insert(shape_index);
                }

                if !shape
                    .tool_type()
                    .subset_of(&tool.tool_type().to_shape_type())
                {
                    continue;
                }

                if let Some((bigger_radius, shrink_by_radius)) =
                    if let cnc_router::ToolType::PartialContourRadius(
                        bigger_radius,
                        shrink_by_radius,
                        _,
                    ) = tool.tool_type()
                    {
                        Some((bigger_radius, shrink_by_radius))
                    } else if let cnc_router::ToolType::PartialContourRadiusOrAngle(
                        previous_radius,
                        extra_shrink_by,
                        _,
                        _,
                    ) = tool.tool_type()
                    {
                        Some((previous_radius, extra_shrink_by))
                    } else {
                        None
                    }
                {
                    let mut bigger_sign = original_sign
                        .expand_lines(
                            // bigger_radius + 0.2 * tool.radius * tool.offset, do_cut_on_odd, add_padding_to
                            bigger_radius,
                            do_cut_on_odd,
                            add_padding_to,
                        )
                        .expand_lines(
                            -(bigger_radius - tool.radius + shrink_by_radius),
                            do_cut_on_odd,
                            &Vec::new(),
                        );
                    cnc_router::CNCPath::cut_till::<T>(
                        shape.lines(),
                        None,
                        None,
                        &mut self.cnc_router,
                        Some(tool.feed_rate_of_cut),
                        true,
                        tool.feed_rate_of_drill,
                        self.z_axis_off_cut + tool.length,
                        self.depth_of_cut + tool.length,
                        &tool.tool_type(),
                        tool.radius,
                        tool.offset,
                        cut_inside,
                        Box::from(|x: f64, y: f64| {
                            !bigger_sign.sees_even_odd_lines_before(x, y, do_cut_on_odd, true)
                        }),
                    );
                } else {
                    cnc_router::CNCPath::cut_till::<T>(
                        shape.lines(),
                        None,
                        None,
                        &mut self.cnc_router,
                        Some(tool.feed_rate_of_cut),
                        true,
                        tool.feed_rate_of_drill,
                        self.z_axis_off_cut + tool.length,
                        self.depth_of_cut + tool.length,
                        &tool.tool_type(),
                        tool.radius,
                        tool.offset,
                        cut_inside,
                        Box::from(|_, _| true),
                    );
                };

                // Only move up if its down
                if self.cnc_router.get_pos().z < self.z_axis_off_cut + tool.length {
                    self.cnc_router.move_to_optional_coordinate(
                        &cnc_router::OptionalCoordinate::from_z(Some(
                            self.z_axis_off_cut + tool.length,
                        )),
                        Some(tool.feed_rate_of_drill),
                        false,
                    );
                }
            }
        }
    }

    pub fn get_router(&self) -> &cnc_router::CNCRouter<T> {
        &self.cnc_router
    }

    pub fn get_router_mut(&mut self) -> &mut cnc_router::CNCRouter<T> {
        &mut self.cnc_router
    }
}

fn float_loop(
    start: f64,
    threshold: f64,
    step_size: f64,
    significant_values: Vec<f64>,
) -> impl Iterator<Item = f64> {
    let mut index = 0;
    let mut step_index = 1;
    std::iter::successors(Some(start), move |&prev| {
        let next = start + step_size * step_index as f64;
        if prev >= threshold {
            None
        } else if next < threshold {
            if index >= significant_values.len() {
                step_index += 1;
                Some(next)
            } else if significant_values[index] < next {
                index += 1;
                Some(significant_values[index - 1])
            } else {
                step_index += 1;
                Some(next)
            }
        } else {
            Some(threshold)
        }
    })
}
