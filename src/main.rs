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
    let cnc = cnc_router::CNCRouter::from(
        vec![
            cnc_router::Tool::from(
                String::from("Quarter Inch Bit"),
                4,
                0.5,
                0.25/2.0,
                // -0.047,
                0.008,
                0.0,
                0.0,
                0.0,
                cnc_router::ToolType::PartialCutBroad,
                cnc_router::Smoothness::Medium,
            ),
            cnc_router::Tool::from(
                String::from("Half Inch Bit"),
                3,
                0.5,
                0.125/2.0,
                // -0.055,
                0.0,
                0.0,
                0.0,
                0.0,
                cnc_router::ToolType::FullCutBroad,
                cnc_router::Smoothness::Medium,
            ),
            cnc_router::Tool::from(
                String::from("1/8 Inch Bit"),
                5,
                0.5,
                0.0625/2.0,
                // -0.055,
                0.0,
                0.0,
                0.0,
                0.0,
                cnc_router::ToolType::FullCutBroad,
                cnc_router::Smoothness::Finish,
            ),
            cnc_router::Tool::from(
                String::from("0.02 Inch Bit"),
                2,
                0.5,
                0.02/2.0,
                // -0.055,
                0.0,
                0.0,
                0.0,
                0.0,
                cnc_router::ToolType::FullCutBroad,
                cnc_router::Smoothness::Finish,
            ),
            cnc_router::Tool::from(
                String::from("Braille Bit"),
                6,
                0.5,
                0.02/2.0,
                // -0.055,
                0.0,
                0.0,
                0.0,
                0.0,
                cnc_router::ToolType::Braille,
                cnc_router::Smoothness::Finish,
            ),
            cnc_router::Tool::from(
                String::from("Text / Lettering Bit"),
                7,
                0.6,
                0.005/2.0,
                // -0.055,
                0.0,
                0.0,
                0.0,
                0.0,
                cnc_router::ToolType::Text,
                cnc_router::Smoothness::Finish,
            ),
        ],
        true,
        cnc_router::Coordinate::from(0.0, 0.0, 1.0),
        // StringHolder::new()
        Output::new()
    );

    let mut gc = gcode_creator::GCodeCreator::from(
        cnc,
        true, // use_inches
        false, // start middle
        12000.0, // spindle_speed
        108.0 * 10000.0, // Feed Rate
        // 50.0, // feed_rate
        0.1, // z_axis_off_cut
        -0.155, // depth_of_cut
    );

    gc.build_gcode(
        // bit_path::Path::spiral_in_out,
        bit_path::Path::path_x_then_y,
        &mut vec![
            sign::Sign::from(
                lines_and_curves::Rectangle::from(
                    lines_and_curves::Point::from(1.0, 1.0),
                    lines_and_curves::Point::from(7.0, 7.0)
                ),
                vec![
                    sign::Shape::from(
                        // lines_and_curves::LineSegment::create_path(&vec![
                        //     lines_and_curves::Point::from(15.0, 15.0),
                        //     lines_and_curves::Point::from(15.0, 20.0),
                        //     lines_and_curves::Point::from(20.0, 20.0),
                        //     lines_and_curves::Point::from(20.0, 15.0),
                        // ], true),
                        lines_and_curves::LineSegment::create_path(&vec![
                            lines_and_curves::Point::from(4.0, 2.0), // Top
                            lines_and_curves::Point::from(6.0, 4.0), // Right
                            lines_and_curves::Point::from(4.0, 6.0), // Bottom
                            lines_and_curves::Point::from(2.0, 4.0), // Left
                        ], true),
                    ),
                ],
            ),
        ],
        &Box::from(|line: &lines_and_curves::LineSegment| -> lines_and_curves::Point {
            line.point1()
        }),
        &Box::from(|router: &mut cnc_router::CNCRouter<Output>,
            line: &lines_and_curves::LineSegment, feed_rate: f64| {
            let p = line.point2();
            router.move_to_coordinate(
                &cnc_router::Coordinate::from(
                    p.x, p.y,
                    router.get_pos().z
                ),
                feed_rate,
                false,
            );
        })
    );
}
