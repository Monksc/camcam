#![allow(dead_code)]
mod utils;
use utils::*;
use utils::lines_and_curves::*;

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
            // cnc_router::Tool {
            //     name: String::from("Quarter Inch Floor"),
            //     index_in_machine: 2,
            //     offset_length: 0.5,
            //     radius: 0.10/2.0,
            //     length: 0.0,
            //     front_angle: 0.0,
            //     back_angle: 0.0,
            //     orientation: 0.0,
            //     tool_type: cnc_router::ToolType::FullCutBroad,
            //     // smoothness: cnc_router::Smoothness::Medium,
            //     smoothness:        cnc_router::Smoothness::Finish,
            //     feed_rate_of_cut: feed_rate_of_cut,
            //     feed_rate_of_drill: feed_rate_of_drill,
            //     offset: 0.5,
            // },
            // cnc_router::Tool {
            //     name: String::from("Quarter Inch Text"),
            //     index_in_machine: 2,
            //     offset_length: 0.5,
            //     radius: 0.10/2.0,
            //     length: 0.0,
            //     front_angle: 0.0,
            //     back_angle: 0.0,
            //     orientation: 0.0,
            //     tool_type: cnc_router::ToolType::FullCutText,
            //     // smoothness: cnc_router::Smoothness::Medium,
            //     smoothness:        cnc_router::Smoothness::Finish,
            //     feed_rate_of_cut: feed_rate_of_cut,
            //     feed_rate_of_drill: feed_rate_of_drill,
            //     offset: 0.5,
            // },
            // cnc_router::Tool {
            //     name: String::from("Fill Up"),
            //     index_in_machine: 3,
            //     offset_length: 0.0,
            //     radius: 0.05/2.0,
            //     length: 0.0,
            //     front_angle: 0.0,
            //     back_angle: 0.0,
            //     orientation: 0.0,
            //     tool_type: cnc_router::ToolType::SpaceBetweenCutBroad,
            //     // tool_type: cnc_router::ToolType::FullCutBroad,
            //     // smoothness: cnc_router::Smoothness::Medium,
            //     smoothness:        cnc_router::Smoothness::Finish,
            //     feed_rate_of_cut: feed_rate_of_cut,
            //     feed_rate_of_drill: feed_rate_of_drill,
            //     offset: 0.5,
            // },
            // cnc_router::Tool {
            //     name: String::from("Fill Up Text"),
            //     index_in_machine: 3,
            //     offset_length: 0.0,
            //     radius: 0.05/2.0,
            //     length: 0.0,
            //     front_angle: 0.0,
            //     back_angle: 0.0,
            //     orientation: 0.0,
            //     tool_type: cnc_router::ToolType::FullCutText,
            //     smoothness:        cnc_router::Smoothness::Finish,
            //     feed_rate_of_cut: feed_rate_of_cut,
            //     feed_rate_of_drill: feed_rate_of_drill,
            //     offset: 0.5,
            // },
            cnc_router::Tool {
                name: String::from("Full Cut Broad"),
                index_in_machine: 1,
                offset_length: 0.0,
                radius: 0.50 / 2.0,
                length: 0.0,
                front_angle: 0.0,
                back_angle: 0.0,
                orientation: 0.0,
                tool_type: cnc_router::ToolType::FullCutBroad,
                // smoothness: cnc_router::Smoothness::Medium,
                smoothness:        cnc_router::Smoothness::Finish,
                feed_rate_of_cut: feed_rate_of_cut,
                feed_rate_of_drill: feed_rate_of_drill,
                offset: 0.5
            },
            cnc_router::Tool {
                name: String::from("Space Between Small"),
                index_in_machine: 2,
                offset_length: 0.0,
                radius: 0.05 / 2.0,
                length: 0.0,
                front_angle: 0.0,
                back_angle: 0.0,
                orientation: 0.0,
                tool_type: cnc_router::ToolType::SpaceBetweenCutBroad(0.0),
                // smoothness: cnc_router::Smoothness::Medium,
                smoothness:        cnc_router::Smoothness::Finish,
                feed_rate_of_cut: feed_rate_of_cut,
                feed_rate_of_drill: feed_rate_of_drill,
                offset: 0.1
            },
            cnc_router::Tool {
                name: String::from("Broad Text Contour"),
                index_in_machine: 1,
                offset_length: 0.0,
                radius: 0.50 / 2.0,
                length: 0.0,
                front_angle: 0.0,
                back_angle: 0.0,
                orientation: 0.0,
                tool_type: cnc_router::ToolType::FullCutText,
                // smoothness: cnc_router::Smoothness::Medium,
                smoothness:        cnc_router::Smoothness::Finish,
                feed_rate_of_cut: feed_rate_of_cut,
                feed_rate_of_drill: feed_rate_of_drill,
                offset: 1.0,
            },
            cnc_router::Tool {
                name: String::from("Exact Text Contour"),
                index_in_machine: 3,
                offset_length: 0.0,
                radius: 0.0,
                length: 0.0,
                front_angle: 0.0,
                back_angle: 0.0,
                orientation: 0.0,
                tool_type: cnc_router::ToolType::PartialCutText(0.0, 0.5 / 2.0),
                smoothness:        cnc_router::Smoothness::Finish,
                feed_rate_of_cut: feed_rate_of_cut,
                feed_rate_of_drill: feed_rate_of_drill,
                offset: 1.0,
            },
            cnc_router::Tool {
                name: String::from("Small Text Contour"),
                index_in_machine: 2,
                offset_length: 0.0,
                radius: 0.05 / 2.0,
                length: 0.0,
                front_angle: 0.0,
                back_angle: 0.0,
                orientation: 0.0,
                tool_type: cnc_router::ToolType::FullCutText,
                // smoothness: cnc_router::Smoothness::Medium,
                smoothness:        cnc_router::Smoothness::Finish,
                feed_rate_of_cut: feed_rate_of_cut,
                feed_rate_of_drill: feed_rate_of_drill,
                offset: 1.0,
            },
            // cnc_router::Tool {
            //     name: String::from("Exact Bit"),
            //     index_in_machine: 1,
            //     offset_length: 0.5,
            //     // radius: 0.0625/2.0,
            //     radius: 0.25/2.0,
            //     length: 0.0,
            //     front_angle: 0.0,
            //     back_angle: 0.0,
            //     orientation: 0.0,
            //     tool_type: cnc_router::ToolType::FullCutText,
            //     // smoothness: cnc_router::Smoothness::Medium,
            //     smoothness:        cnc_router::Smoothness::Finish,
            //     feed_rate_of_cut: feed_rate_of_cut,
            //     feed_rate_of_drill: feed_rate_of_drill,
            //     offset: 1.0,
            // },
        ],
        false, // verbose
        cnc_router::Coordinate::from(0.0, 0.0, 10.0),
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

    // Tight inner line
    /*
    let shapes = &mut vec![
        sign::Sign::from(
            lines_and_curves::Rectangle::from(
                lines_and_curves::Point::from(1.0, 1.0),
                lines_and_curves::Point::from(7.0, 7.0)
            ),
            vec![
                sign::Shape::from(
                    cnc_router::ToolType::FullCutText,
                    lines_and_curves::LineSegment::create_path(&vec![
                        lines_and_curves::Point::from(2.0, 6.0), // Top Left
                        lines_and_curves::Point::from(5.0, 6.0), // Top Right
                        lines_and_curves::Point::from(5.0, 4.8), // move down
                        lines_and_curves::Point::from(3.0, 4.8), // move left
                        lines_and_curves::Point::from(3.0, 4.6), // move down
                        lines_and_curves::Point::from(5.0, 4.6), // move right
                        lines_and_curves::Point::from(5.0, 2.0), // Bottom Right
                        lines_and_curves::Point::from(2.0, 2.0), // Bottom Left
                    ], true)
                    .iter()
                    .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                    .collect(),
                ),
            ],
        ),
    ];
    */


    /*
    // V
    let shapes = &mut vec![
        sign::Sign::from(
            lines_and_curves::Rectangle::from(
                lines_and_curves::Point::from(1.0, 1.0),
                lines_and_curves::Point::from(7.0, 7.0)
            ),
            vec![
                sign::Shape::from(
                    cnc_router::ToolType::FullCutText,
                    lines_and_curves::LineSegment::create_path(&vec![
                        lines_and_curves::Point::from(2.0, 6.0), // Top Left Outer V
                        lines_and_curves::Point::from(3.0, 6.0), // Top Left Inner V
                        lines_and_curves::Point::from(4.0, 3.0), // Bottom Center Inner V
                        lines_and_curves::Point::from(5.0, 6.0), // Top Right Inner V
                        lines_and_curves::Point::from(6.0, 6.0), // Top Right Outside V
                        lines_and_curves::Point::from(4.0, 2.0), // Bottom Center Outer V
                    ], true)
                    .iter()
                    .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                    .collect(),
                ),
            ],
        ),
    ];
    */

    /*
    // Tight V
    let shapes = &mut vec![
        sign::Sign::from(
            lines_and_curves::Rectangle::from(
                lines_and_curves::Point::from(1.0, 1.0),
                lines_and_curves::Point::from(7.0, 9.0)
            ),
            vec![
                sign::Shape::from(
                    cnc_router::ToolType::FullCutText,
                    lines_and_curves::LineSegment::create_path(&vec![
                        lines_and_curves::Point::from(2.0,  6.0), // Top Left Outer V
                        lines_and_curves::Point::from(3.7,  6.0), // Top Left Inner V
                        // lines_and_curves::Point::from(3.7,  3.0), // Bottom Center Inner V
                        lines_and_curves::Point::from(4.0,  3.0), // Bottom Center Inner V
                        // lines_and_curves::Point::from(4.3,  3.0), // Bottom Center Inner V
                        lines_and_curves::Point::from(4.3,  6.0), // Top Right Inner V
                        lines_and_curves::Point::from(6.0,  6.0), // Top Right Outside V
                        lines_and_curves::Point::from(4.0,  2.0), // Bottom Center Outer V
                    ], true)
                    .iter()
                    .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                    .collect(),
                ),
                sign::Shape::from(
                    cnc_router::ToolType::FullCutText,
                    lines_and_curves::LineSegment::create_path(&vec![
                        lines_and_curves::Point::from(1.0,  9.0), // Top Left
                        lines_and_curves::Point::from(7.0,  9.0), // Top Right
                        lines_and_curves::Point::from(7.0,  1.0), // Bottom Right
                        lines_and_curves::Point::from(1.0,  1.0), // Bottom Left
                    ], true)
                    .iter()
                    .map(|x| lines_and_curves::AllIntersections::SoftLineSegment(x.clone()))
                    .collect(),
                ),
            ],
        ),
    ];
    */

    // Tight right V
    let shapes = &mut vec![
        sign::Sign::from(
            lines_and_curves::Rectangle::from(
                lines_and_curves::Point::from(1.0, 1.0),
                lines_and_curves::Point::from(7.0, 7.0)
            ),
            vec![
                sign::Shape::from(
                    cnc_router::ToolType::FullCutText,
                    lines_and_curves::LineSegment::create_path(&vec![
                        lines_and_curves::Point::from(6.0,  6.0), // Top Left Outer V
                        lines_and_curves::Point::from(6.0,  5.0), // Top Left Inner V
                        lines_and_curves::Point::from(3.0,  4.0), // Bottom Center Inner V
                        lines_and_curves::Point::from(6.0,  3.0), // Top Right Inner V
                        lines_and_curves::Point::from(6.0,  2.0), // Top Right Outside V
                        lines_and_curves::Point::from(2.0,  4.0), // Bottom Center Outer V
                    ], true)
                    .iter()
                    .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                    .collect(),
                ),
            ],
        ),
    ];

    // Tight Left V
    /*
    let shapes = &mut vec![
        sign::Sign::from(
            lines_and_curves::Rectangle::from(
                lines_and_curves::Point::from(1.0, 1.0),
                lines_and_curves::Point::from(7.0, 7.0)
            ),
            vec![
                sign::Shape::from(
                    cnc_router::ToolType::FullCutText,
                    lines_and_curves::LineSegment::create_path(&vec![
                        lines_and_curves::Point::from(2.0,  6.0), // Top Left Outer V
                        lines_and_curves::Point::from(2.0,  5.0), // Top Left Inner V
                        lines_and_curves::Point::from(5.0,  4.0), // Bottom Center Inner V
                        lines_and_curves::Point::from(2.0,  3.0), // Top Right Inner V
                        lines_and_curves::Point::from(2.0,  2.0), // Top Right Outside V
                        lines_and_curves::Point::from(6.0,  4.0), // Bottom Center Outer V
                    ], true)
                    .iter()
                    .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                    .collect(),
                ),
            ],
        ),
    ];
    */

    // Tight Downward V
    /*
    let shapes = &mut vec![
        sign::Sign::from(
            lines_and_curves::Rectangle::from(
                lines_and_curves::Point::from(1.0, 1.0),
                lines_and_curves::Point::from(7.0, 7.0)
            ),
            vec![
                sign::Shape::from(
                    cnc_router::ToolType::FullCutText,
                    lines_and_curves::LineSegment::create_path(&vec![
                        lines_and_curves::Point::from(2.0,  8.0-6.0), // Top Left Outer V
                        lines_and_curves::Point::from(3.8,  8.0-6.0), // Top Left Inner V
                        lines_and_curves::Point::from(3.9,  8.0-3.0), // Bottom Center Inner V
                        lines_and_curves::Point::from(4.1,  8.0-3.0), // Bottom Center Inner V
                        lines_and_curves::Point::from(4.2,  8.0-6.0), // Top Right Inner V
                        lines_and_curves::Point::from(6.0,  8.0-6.0), // Top Right Outside V
                        lines_and_curves::Point::from(4.0,  8.0-2.0), // Bottom Center Outer V
                    ], true)
                    .iter()
                    .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                    .collect(),
                ),
            ],
        ),
    ];
    */

    // let shapes = &mut vec![
    //     sign::Sign::from(
    //             lines_and_curves::Rectangle::from(
    //                     lines_and_curves::Point::from(1.0, 1.0),
    //                     lines_and_curves::Point::from(7.0, 7.0)
    //             ),
    //             vec![
    //             sign::Shape::from(
    //                     cnc_router::ToolType::FullCutText,
    //                     lines_and_curves::LineSegment::create_path(&vec![
    //                             lines_and_curves::Point::from(1.7697503566741943, 1.5),
    //                             lines_and_curves::Point::from(1.696648359298706, 1.5),
    //                             lines_and_curves::Point::from(1.696648359298706, 2.060875415802002),
    //                             lines_and_curves::Point::from(1.5, 2.060875415802002),
    //                             lines_and_curves::Point::from(1.5, 2.125),
    //                             lines_and_curves::Point::from(1.9655437469482422, 2.125),
    //                             lines_and_curves::Point::from(1.9655437469482422, 2.060875415802002),
    //                             lines_and_curves::Point::from(1.7697503566741943, 2.060875415802002),
    //                             lines_and_curves::Point::from(1.7697503566741943, 1.5),
    //             ], true)
    //                     .iter()
    //                     .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
    //                     .collect(),
    //             ),
    //             sign::Shape::from(
    //                     cnc_router::ToolType::FullCutText,
    //                     lines_and_curves::LineSegment::create_path(&vec![
    //                             lines_and_curves::Point::from(1.0, 2.5), // Softy
    //                             lines_and_curves::Point::from(2.5, 2.5), // Softy
    //                             lines_and_curves::Point::from(2.5, 1.0), // Softy
    //                             lines_and_curves::Point::from(1.0, 1.0), // Softy
    //             ], true)
    //                     .iter()
    //                     .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
    //                     .collect(),
    //             ),
    //             ],
    //     )
    // ];


    gc.build_gcode_smart_path(
        false,
        &shapes,
        &vec![
            (cnc_router::ToolType::FullCutText, 0.009),
            (cnc_router::ToolType::Braille, 0.0),
        ],
    );
}
