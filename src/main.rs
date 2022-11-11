#![allow(dead_code)]
mod utils;
use utils::*;

pub struct StringHolder(String);

impl StringHolder {
    fn new() -> StringHolder {
        StringHolder(String::new())
    }
}

impl std::io::Write for StringHolder {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        self.0.push_str(&std::str::from_utf8(buf).unwrap().to_string());
        return Ok(buf.len());
    }
    fn flush(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
}

pub struct Output();

impl Output {
    fn new() -> Output {
        Output()
    }
}

impl std::io::Write for Output {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        print!("{}", &std::str::from_utf8(buf).unwrap().to_string());
        return Ok(buf.len());
    }
    fn flush(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
}


fn main() {
    let feed_rate_of_cut = 108.0; // * 10000.0;
    let feed_rate_of_drill = 50.0; // * 10000.0;

    let cnc = cnc_router::CNCRouter::from(
        vec![
            cnc_router::Tool {
                name: String::from("Quarter Inch Bit"),
                index_in_machine: 4,
                offset_length: 0.5,
                radius: 0.25/2.0,
                length: 0.008,
                front_angle: 0.0,
                back_angle: 0.0,
                orientation: 0.0,
                tool_type: cnc_router::ToolType::PartialCutBroad,
                // smoothness: cnc_router::Smoothness::Medium,
                smoothness:        cnc_router::Smoothness::Finish,
                feed_rate_of_cut: feed_rate_of_cut,
                feed_rate_of_drill: feed_rate_of_drill,
                offset: 0.2,
            },
            cnc_router::Tool {
                name:              String::from("1/8 Inch Bit"),
                index_in_machine:  3,
                offset_length:     0.5,
                radius:            0.125/2.0,
                length:            0.0,
                front_angle:       0.0,
                back_angle:        0.0,
                orientation:       0.0,
                tool_type:         cnc_router::ToolType::FullCutBroad,
                // smoothness:        cnc_router::Smoothness::Medium,
                smoothness:        cnc_router::Smoothness::Finish,
                feed_rate_of_cut: feed_rate_of_cut,
                feed_rate_of_drill: feed_rate_of_drill,
                offset: 0.2,
            },
            // cnc_router::Tool {
            //     name:              String::from("1/16 Inch Bit"),
            //     index_in_machine:  5,
            //     offset_length:     0.5,
            //     radius:            0.0625/2.0,
            //     length:            0.0,
            //     front_angle:       0.0,
            //     back_angle:        0.0,
            //     orientation:       0.0,
            //     tool_type:         cnc_router::ToolType::FullCutBroad,
            //     smoothness:        cnc_router::Smoothness::Finish,
            //     feed_rate_of_cut: feed_rate_of_cut,
            //     feed_rate_of_drill: feed_rate_of_drill,
            //     offset: 1.0,
            // },
            // cnc_router::Tool {
            //     name:              String::from("0.02 Inch Bit"),
            //     index_in_machine:  2,
            //     offset_length:     0.5,
            //     radius:            0.02/2.0,
            //     length:            0.0,
            //     front_angle:       0.0,
            //     back_angle:        0.0,
            //     orientation:       0.0,
            //     tool_type:         cnc_router::ToolType::FullCutBroad,
            //     smoothness:        cnc_router::Smoothness::Finish,
            //     feed_rate_of_cut:  feed_rate_of_cut,
            //     feed_rate_of_drill:feed_rate_of_drill,
            //     offset: 1.0,
            // },
            // cnc_router::Tool {
            //     name:              String::from("Braille Bit"),
            //     index_in_machine:  6,
            //     offset_length:     0.5,
            //     radius:            0.02/2.0,
            //     length:            0.0,
            //     front_angle:       0.0,
            //     back_angle:        0.0,
            //     orientation:       0.0,
            //     tool_type:         cnc_router::ToolType::Braille,
            //     smoothness:        cnc_router::Smoothness::Finish,
            //     feed_rate_of_cut:  feed_rate_of_cut,
            //     feed_rate_of_drill:feed_rate_of_drill,
            //     offset: 1.0,
            // },
            cnc_router::Tool {
                name:              String::from("Text / Lettering Bit"),
                index_in_machine:  7,
                offset_length:     0.6,
                radius:            0.005/2.0,
                length:            0.0,
                front_angle:       0.0,
                back_angle:        0.0,
                orientation:       0.0,
                tool_type:         cnc_router::ToolType::Text,
                smoothness:        cnc_router::Smoothness::Finish,
                feed_rate_of_cut:  feed_rate_of_cut,
                feed_rate_of_drill:feed_rate_of_drill,
                offset: 1.0,
            },
        ],
        false, // verbose
        cnc_router::Coordinate::from(0.0, 0.0, 10.0),
        // StringHolder::new()
        Output::new()
    );

    let mut gc = gcode_creator::GCodeCreator::from(
        cnc,
        true, // use_inches
        true, // start middle
        12000.0, // spindle_speed
        108.0 * 10000.0, // Feed Rate
        // 50.0, // feed_rate
        0.1, // z_axis_off_cut
        -0.155, // depth_of_cut
    );

    gc.build_gcode(
        false,
        bit_path::Path::spiral_in_out,
        // bit_path::Path::path_x_then_y,
        &mut vec![
            sign::Sign::from(
                lines_and_curves::Rectangle::from(
                    lines_and_curves::Point::from(1.0, 1.0),
                    lines_and_curves::Point::from(7.0, 7.0)
                ),
                vec![
                    sign::Shape::from(
                        lines_and_curves::LineSegment::create_path(&vec![
                            lines_and_curves::Point::from(4.0, 2.0), // Top
                            lines_and_curves::Point::from(6.0, 4.0), // Right
                            lines_and_curves::Point::from(4.0, 6.0), // Bottom
                            lines_and_curves::Point::from(2.0, 4.0), // Left
                        ], true)
                        .iter()
                        .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                        .collect(),
                    ),
                    sign::Shape::from(
                        lines_and_curves::Rectangle::from(
                            lines_and_curves::Point::from(2.0, 2.0),
                            lines_and_curves::Point::from(6.0, 6.0),
                        )
                        .to_lines()
                        .iter()
                        .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                        .collect(),
                    ),
                ],
            ),
        ],
    );
}
