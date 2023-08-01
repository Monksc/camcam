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
                name: String::from("Broad Text Contour"),
                index_in_machine: 1,
                offset_length: 0.0,
                radius: 0.005 / 2.0,
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
                index_in_machine: 1,
                offset_length: 0.0,
                radius: 0.0,
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
                name: String::from("Full Cut Broad"),
                index_in_machine: 1,
                offset_length: 0.0,
                radius: 0.005 / 2.0,
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
                        lines_and_curves::Point::from(4.0, 4.0), // Top
                        lines_and_curves::Point::from(2.0, 2.0), // Bottom Left
                        lines_and_curves::Point::from(6.0, 2.0), // Bottom Right
                    ], true)
                    .iter()
                    .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                    .collect(),
                ),
                sign::Shape::from(
                    cnc_router::ToolType::FullCutText,
                    lines_and_curves::LineSegment::create_path(&vec![
                        lines_and_curves::Point::from(4.0, 4.0), // Bottom
                        lines_and_curves::Point::from(2.0, 6.0), // Top Left
                        lines_and_curves::Point::from(6.0, 6.0), // Top Right
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
     * Pinball stick
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
                        lines_and_curves::Point::from(2.0, 4.0), // Top Left
                        lines_and_curves::Point::from(5.0, 3.9), // Right
                        lines_and_curves::Point::from(2.0, 3.8), // Bottom Left
                    ], true)
                    .iter()
                    .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                    .collect(),
                ),
                // sign::Shape::from(
                //     cnc_router::ToolType::FullCutText,
                //     lines_and_curves::LineSegment::create_path(&vec![
                //         lines_and_curves::Point::from(4.0, 4.0), // Bottom
                //         lines_and_curves::Point::from(2.0, 6.0), // Top Left
                //         lines_and_curves::Point::from(6.0, 6.0), // Top Right
                //     ], true)
                //     .iter()
                //     .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                //     .collect(),
                // ),
            ],
        ),
    ];
    */

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

    // Tight V
    /*
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
                        lines_and_curves::Point::from(2.0, 7.0),
                        lines_and_curves::Point::from(5.0, 7.0),
                        lines_and_curves::Point::from(5.0, 5.0),
                        lines_and_curves::Point::from(4.8, 6.3),
                        lines_and_curves::Point::from(4.0, 4.4),
                        lines_and_curves::Point::from(3.0, 4.0),
                        lines_and_curves::Point::from(4.0, 3.0),
                        lines_and_curves::Point::from(5.0, 2.2),
                        lines_and_curves::Point::from(5.0, 2.0),
                        lines_and_curves::Point::from(2.0, 2.0),
                    ], true)
                    .iter()
                    .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                    .collect(),
                ),
            ],
        ),
    ];

    /*
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
    */

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

    /*
     * Tight Downward V
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


    // let shapes = &mut vec![
    //     sign::Sign::from(
    //             lines_and_curves::Rectangle::from(
    //                     lines_and_curves::Point::from(1.0, 1.0),
    //                     lines_and_curves::Point::from(15.0, 15.0)
    //             ),
    //             vec![
    //             sign::Shape::from(
    //                     cnc_router::ToolType::FullCutText,
    //                     lines_and_curves::LineSegment::create_path(&vec![
    //                             lines_and_curves::Point::from(1.0, 8.5), // Softy
    //                             lines_and_curves::Point::from(1.0, 1.0), // Softy
    //                             lines_and_curves::Point::from(11.0, 1.0), // Softy
    //                             lines_and_curves::Point::from(11.0, 8.5), // Softy
    //             ], true)
    //                     .iter()
    //                     .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
    //                     .collect(),
    //             ),
    //             sign::Shape::from(
    //                     cnc_router::ToolType::FullCutText,
    //                     lines_and_curves::LineSegment::create_path(&vec![
    //                             lines_and_curves::Point::from(2.0, 2.0),
    //                             lines_and_curves::Point::from(4.0, 2.0),
    //                             lines_and_curves::Point::from(4.0, 4.0),
    //                             lines_and_curves::Point::from(2.0, 4.0),
    //             ], true)
    //                     .iter()
    //                     .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
    //                     .collect(),
    //             ),
    //             sign::Shape::from(
    //                     cnc_router::ToolType::FullCutText,
    //                     lines_and_curves::LineSegment::create_path(&vec![
    //                             lines_and_curves::Point::from(2.1, 2.1),
    //                             lines_and_curves::Point::from(3.9, 2.1),
    //                             lines_and_curves::Point::from(3.9, 3.9),
    //                             lines_and_curves::Point::from(2.1, 3.9),
    //             ], true)
    //                     .iter()
    //                     .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
    //                     .collect(),
    //             ),
    //             sign::Shape::from(
    //                     cnc_router::ToolType::FullCutText,
    //                     lines_and_curves::LineSegment::create_path(&vec![
    //                             lines_and_curves::Point::from(2.0, 8.0),
    //                             lines_and_curves::Point::from(4.0, 8.0),
    //                             lines_and_curves::Point::from(4.0, 5.0),
    //                             lines_and_curves::Point::from(3.0, 6.0),
    //                             lines_and_curves::Point::from(2.0, 5.0),
    //             ], true)
    //                     .iter()
    //                     .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
    //                     .collect(),
    //             ),
    //             sign::Shape::from(
    //                     cnc_router::ToolType::FullCutText,
    //                     lines_and_curves::LineSegment::create_path(&vec![
    //                             lines_and_curves::Point::from(7.620425701141357, 5.811859130859375),
    //                             lines_and_curves::Point::from(7.453274726867676, 5.186859130859375),
    //                             lines_and_curves::Point::from(7.3801727294921875, 5.186859130859375),
    //                             lines_and_curves::Point::from(7.256198406219482, 5.607515335083008),
    //                             lines_and_curves::Point::from(7.245724678039551, 5.643638610839844),
    //                             lines_and_curves::Point::from(7.2365336418151855, 5.6782660484313965),
    //                             lines_and_curves::Point::from(7.229479789733887, 5.707335948944092),
    //                             lines_and_curves::Point::from(7.225419044494629, 5.7263593673706055),
    //                             lines_and_curves::Point::from(7.222212791442871, 5.707549571990967),
    //                             lines_and_curves::Point::from(7.2162275314331055, 5.67890739440918),
    //                             lines_and_curves::Point::from(7.207677841186523, 5.644280433654785),
    //                             lines_and_curves::Point::from(7.197204113006592, 5.607088088989258),
    //                             lines_and_curves::Point::from(7.076650142669678, 5.186859130859375),
    //                             lines_and_curves::Point::from(7.0035481452941895, 5.186859130859375),
    //                             lines_and_curves::Point::from(6.837679386138916, 5.811859130859375),
    //                             lines_and_curves::Point::from(6.913346290588379, 5.811859130859375),
    //                             lines_and_curves::Point::from(7.013807773590088, 5.419844627380371),
    //                             lines_and_curves::Point::from(7.023426532745361, 5.381155967712402),
    //                             lines_and_curves::Point::from(7.031548976898193, 5.34417724609375),
    //                             lines_and_curves::Point::from(7.038175106048584, 5.3091230392456055),
    //                             lines_and_curves::Point::from(7.043732643127441, 5.275777816772461),
    //                             lines_and_curves::Point::from(7.049290180206299, 5.310619354248047),
    //                             lines_and_curves::Point::from(7.056771278381348, 5.347597599029541),
    //                             lines_and_curves::Point::from(7.065748691558838, 5.385644912719727),
    //                             lines_and_curves::Point::from(7.0762224197387695, 5.424119472503662),
    //                             lines_and_curves::Point::from(7.188653945922852, 5.811859130859375),
    //                             lines_and_curves::Point::from(7.263465881347656, 5.811859130859375),
    //                             lines_and_curves::Point::from(7.3805999755859375, 5.421126842498779),
    //                             lines_and_curves::Point::from(7.391715049743652, 5.381369590759277),
    //                             lines_and_curves::Point::from(7.400906085968018, 5.343109130859375),
    //                             lines_and_curves::Point::from(7.408173561096191, 5.307626724243164),
    //                             lines_and_curves::Point::from(7.413944721221924, 5.275777816772461),
    //                             lines_and_curves::Point::from(7.421212196350098, 5.319810390472412),
    //                             lines_and_curves::Point::from(7.431258201599121, 5.368117332458496),
    //                             lines_and_curves::Point::from(7.443869590759277, 5.420271873474121),
    //                             lines_and_curves::Point::from(7.544331073760986, 5.811859130859375),
    //                             lines_and_curves::Point::from(7.620425701141357, 5.811859130859375),
    //             ], true)
    //                     .iter()
    //                     .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
    //                     .collect(),
    //             ),
    //             sign::Shape::from(
    //                     cnc_router::ToolType::FullCutText,
    //                     lines_and_curves::LineSegment::create_path(&vec![
    //                             lines_and_curves::Point::from(8.260815620422363, 5.500213623046875),
    //                             lines_and_curves::Point::from(8.258662700653076, 5.4522905349731445),
    //                             lines_and_curves::Point::from(8.252440452575684, 5.408623695373535),
    //                             lines_and_curves::Point::from(8.242433071136475, 5.368758678436279),
    //                             lines_and_curves::Point::from(8.227899551391602, 5.330462455749512),
    //                             lines_and_curves::Point::from(8.209716796875, 5.296713829040527),
    //                             lines_and_curves::Point::from(8.18792724609375, 5.267014503479004),
    //                             lines_and_curves::Point::from(8.162062168121338, 5.240939617156982),
    //                             lines_and_curves::Point::from(8.132349967956543, 5.219120025634766),
    //                             lines_and_curves::Point::from(8.098366737365723, 5.2013936042785645),
    //                             lines_and_curves::Point::from(8.062059879302979, 5.1888885498046875),
    //                             lines_and_curves::Point::from(8.02099084854126, 5.181052207946777),
    //                             lines_and_curves::Point::from(7.974392890930176, 5.178308963775635),
    //                             lines_and_curves::Point::from(7.926342964172363, 5.181070804595947),
    //                             lines_and_curves::Point::from(7.884374618530273, 5.188925743103027),
    //                             lines_and_curves::Point::from(7.847640037536621, 5.2013936042785645),
    //                             lines_and_curves::Point::from(7.813278675079346, 5.219186305999756),
    //                             lines_and_curves::Point::from(7.783512115478516, 5.241081237792969),
    //                             lines_and_curves::Point::from(7.757865905761719, 5.267228126525879),
    //                             lines_and_curves::Point::from(7.736386775970459, 5.297056198120117),
    //                             lines_and_curves::Point::from(7.718557834625244, 5.330943584442139),
    //                             lines_and_curves::Point::from(7.704428672790527, 5.3694000244140625),
    //                             lines_and_curves::Point::from(7.694770812988281, 5.409379959106445),
    //                             lines_and_curves::Point::from(7.688765048980713, 5.453124046325684),
    //                             lines_and_curves::Point::from(7.686687469482422, 5.501069068908691),
    //                             lines_and_curves::Point::from(7.6888251304626465, 5.549261569976807),
    //                             lines_and_curves::Point::from(7.694991588592529, 5.593020439147949),
    //                             lines_and_curves::Point::from(7.70488977432251, 5.632815361022949),
    //                             lines_and_curves::Point::from(7.718322277069092, 5.669075012207031),
    //                             lines_and_curves::Point::from(7.736441612243652, 5.703609466552734),
    //                             lines_and_curves::Point::from(7.758368968963623, 5.733582496643066),
    //                             lines_and_curves::Point::from(7.784219741821289, 5.759452819824219),
    //                             lines_and_curves::Point::from(7.814295291900635, 5.781506538391113),
    //                             lines_and_curves::Point::from(7.8469743728637695, 5.798443794250488),
    //                             lines_and_curves::Point::from(7.884254455566406, 5.810997009277344),
    //                             lines_and_curves::Point::from(7.926869869232178, 5.81890869140625),
    //                             lines_and_curves::Point::from(7.975675582885742, 5.821691513061523),
    //                             lines_and_curves::Point::from(8.022191047668457, 5.818972587585449),
    //                             lines_and_curves::Point::from(8.063258171081543, 5.811199188232422),
    //                             lines_and_curves::Point::from(8.099623680114746, 5.798788547515869),
    //                             lines_and_curves::Point::from(8.131925582885742, 5.781933784484863),
    //                             lines_and_curves::Point::from(8.161604404449463, 5.760157108306885),
    //                             lines_and_curves::Point::from(8.187382221221924, 5.734492301940918),
    //                             lines_and_curves::Point::from(8.209520816802979, 5.704641342163086),
    //                             lines_and_curves::Point::from(8.22811222076416, 5.6701436042785645),
    //                             lines_and_curves::Point::from(8.241953372955322, 5.633991718292236),
    //                             lines_and_curves::Point::from(8.252188205718994, 5.593953609466553),
    //                             lines_and_curves::Point::from(8.258588790893555, 5.549541473388672),
    //                             lines_and_curves::Point::from(8.260815620422363, 5.500213623046875),
    //             ], true)
    //                     .iter()
    //                     .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
    //                     .collect(),
    //             ),
    //             sign::Shape::from(
    //                     cnc_router::ToolType::FullCutText,
    //                     lines_and_curves::LineSegment::create_path(&vec![
    //                             lines_and_curves::Point::from(7.763209342956543, 5.500213623046875),
    //                             lines_and_curves::Point::from(7.76594877243042, 5.447503566741943),
    //                             lines_and_curves::Point::from(7.7736897468566895, 5.401942253112793),
    //                             lines_and_curves::Point::from(7.785866737365723, 5.362559795379639),
    //                             lines_and_curves::Point::from(7.804069519042969, 5.326083183288574),
    //                             lines_and_curves::Point::from(7.827157497406006, 5.296460151672363),
    //                             lines_and_curves::Point::from(7.855335235595703, 5.272785663604736),
    //                             lines_and_curves::Point::from(7.879222869873047, 5.259444236755371),
    //                             lines_and_curves::Point::from(7.906619071960449, 5.249570369720459),
    //                             lines_and_curves::Point::from(7.938110828399658, 5.243344306945801),
    //                             lines_and_curves::Point::from(7.974392890930176, 5.241150856018066),
    //                             lines_and_curves::Point::from(8.010924816131592, 5.243354797363281),
    //                             lines_and_curves::Point::from(8.04247760772705, 5.249597549438477),
    //                             lines_and_curves::Point::from(8.069780349731445, 5.259472846984863),
    //                             lines_and_curves::Point::from(8.093450546264648, 5.272785663604736),
    //                             lines_and_curves::Point::from(8.121331214904785, 5.296442985534668),
    //                             lines_and_curves::Point::from(8.14413833618164, 5.326061248779297),
    //                             lines_and_curves::Point::from(8.162063598632813, 5.362559795379639),
    //                             lines_and_curves::Point::from(8.174005508422852, 5.401919841766357),
    //                             lines_and_curves::Point::from(8.181603908538818, 5.447482109069824),
    //                             lines_and_curves::Point::from(8.184293746948242, 5.500213623046875),
    //                             lines_and_curves::Point::from(8.181918621063232, 5.550131797790527),
    //                             lines_and_curves::Point::from(8.175220012664795, 5.593160629272461),
    //                             lines_and_curves::Point::from(8.164727210998535, 5.6301984786987305),
    //                             lines_and_curves::Point::from(8.150804042816162, 5.662044525146484),
    //                             lines_and_curves::Point::from(8.133635520935059, 5.689381122589111),
    //                             lines_and_curves::Point::from(8.111894607543945, 5.713229179382324),
    //                             lines_and_curves::Point::from(8.085999488830566, 5.731911659240723),
    //                             lines_and_curves::Point::from(8.055325508117676, 5.745706558227539),
    //                             lines_and_curves::Point::from(8.018941402435303, 5.754454135894775),
    //                             lines_and_curves::Point::from(7.975675582885742, 5.757566928863525),
    //                             lines_and_curves::Point::from(7.93914270401001, 5.755398273468018),
    //                             lines_and_curves::Point::from(7.907463073730469, 5.749246597290039),
    //                             lines_and_curves::Point::from(7.879938125610352, 5.739502906799316),
    //                             lines_and_curves::Point::from(7.855976104736328, 5.7263593673706055),
    //                             lines_and_curves::Point::from(7.827625751495361, 5.702956199645996),
    //                             lines_and_curves::Point::from(7.804403305053711, 5.673679351806641),
    //                             lines_and_curves::Point::from(7.786080360412598, 5.6376543045043945),
    //                             lines_and_curves::Point::from(7.7738142013549805, 5.598676681518555),
    //                             lines_and_curves::Point::from(7.765987873077393, 5.5532145500183105),
    //                             lines_and_curves::Point::from(7.763209342956543, 5.500213623046875),
    //             ], true)
    //                     .iter()
    //                     .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
    //                     .collect(),
    //             ),
    //             sign::Shape::from(
    //                     cnc_router::ToolType::FullCutText,
    //                     lines_and_curves::LineSegment::create_path(&vec![
    //                             lines_and_curves::Point::from(8.67463207244873, 5.186859130859375),
    //                             lines_and_curves::Point::from(8.464731216430664, 5.740039348602295),
    //                             lines_and_curves::Point::from(8.461311340332031, 5.740039348602295),
    //                             lines_and_curves::Point::from(8.464303970336914, 5.69728946685791),
    //                             lines_and_curves::Point::from(8.46644115447998, 5.6423563957214355),
    //                             lines_and_curves::Point::from(8.467296600341797, 5.5818657875061035),
    //                             lines_and_curves::Point::from(8.467296600341797, 5.186859130859375),
    //                             lines_and_curves::Point::from(8.399751663208008, 5.186859130859375),
    //                             lines_and_curves::Point::from(8.399751663208008, 5.811859130859375),
    //                             lines_and_curves::Point::from(8.507481098175049, 5.811859130859375),
    //                             lines_and_curves::Point::from(8.705411911010742, 5.292022705078125),
    //                             lines_and_curves::Point::from(8.708404541015625, 5.292022705078125),
    //                             lines_and_curves::Point::from(8.909327507019043, 5.811859130859375),
    //                             lines_and_curves::Point::from(9.016201972961426, 5.811859130859375),
    //                             lines_and_curves::Point::from(9.016201972961426, 5.186859130859375),
    //                             lines_and_curves::Point::from(8.944382667541504, 5.186859130859375),
    //                             lines_and_curves::Point::from(8.944382667541504, 5.586995601654053),
    //                             lines_and_curves::Point::from(8.945237159729004, 5.642784118652344),
    //                             lines_and_curves::Point::from(8.947374820709229, 5.695793628692627),
    //                             lines_and_curves::Point::from(8.949939727783203, 5.739184379577637),
    //                             lines_and_curves::Point::from(8.94651985168457, 5.739184379577637),
    //                             lines_and_curves::Point::from(8.73405408859253, 5.186859130859375),
    //                             lines_and_curves::Point::from(8.67463207244873, 5.186859130859375),
    //             ], true)
    //                     .iter()
    //                     .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
    //                     .collect(),
    //             ),
    //             sign::Shape::from(
    //                     cnc_router::ToolType::FullCutText,
    //                     lines_and_curves::LineSegment::create_path(&vec![
    //                             lines_and_curves::Point::from(9.53518295288086, 5.186859130859375),
    //                             lines_and_curves::Point::from(9.187200546264648, 5.186859130859375),
    //                             lines_and_curves::Point::from(9.187200546264648, 5.811859130859375),
    //                             lines_and_curves::Point::from(9.53518295288086, 5.811859130859375),
    //                             lines_and_curves::Point::from(9.53518295288086, 5.747734069824219),
    //                             lines_and_curves::Point::from(9.259875297546387, 5.747734069824219),
    //                             lines_and_curves::Point::from(9.259875297546387, 5.545528411865234),
    //                             lines_and_curves::Point::from(9.519365310668945, 5.545528411865234),
    //                             lines_and_curves::Point::from(9.519365310668945, 5.4822587966918945),
    //                             lines_and_curves::Point::from(9.259875297546387, 5.4822587966918945),
    //                             lines_and_curves::Point::from(9.259875297546387, 5.250983238220215),
    //                             lines_and_curves::Point::from(9.53518295288086, 5.250983238220215),
    //                             lines_and_curves::Point::from(9.53518295288086, 5.186859130859375),
    //             ], true)
    //                     .iter()
    //                     .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
    //                     .collect(),
    //             ),
    //             sign::Shape::from(
    //                     cnc_router::ToolType::FullCutText,
    //                     lines_and_curves::LineSegment::create_path(&vec![
    //                             lines_and_curves::Point::from(10.162320137023926, 5.186859130859375),
    //                             lines_and_curves::Point::from(10.078531265258789, 5.186859130859375),
    //                             lines_and_curves::Point::from(9.737815856933594, 5.711824417114258),
    //                             lines_and_curves::Point::from(9.734395980834961, 5.711824417114258),
    //                             lines_and_curves::Point::from(9.737388610839844, 5.664158821105957),
    //                             lines_and_curves::Point::from(9.740167617797852, 5.607301712036133),
    //                             lines_and_curves::Point::from(9.741235733032227, 5.545955657958984),
    //                             lines_and_curves::Point::from(9.741235733032227, 5.186859130859375),
    //                             lines_and_curves::Point::from(9.673691749572754, 5.186859130859375),
    //                             lines_and_curves::Point::from(9.673691749572754, 5.811859130859375),
    //                             lines_and_curves::Point::from(9.75705337524414, 5.811859130859375),
    //                             lines_and_curves::Point::from(10.09648609161377, 5.288602828979492),
    //                             lines_and_curves::Point::from(10.099477767944336, 5.288602828979492),
    //                             lines_and_curves::Point::from(10.09734058380127, 5.331138610839844),
    //                             lines_and_curves::Point::from(10.094989776611328, 5.390561103820801),
    //                             lines_and_curves::Point::from(10.093920707702637, 5.449769020080566),
    //                             lines_and_curves::Point::from(10.093920707702637, 5.811859130859375),
    //                             lines_and_curves::Point::from(10.162320137023926, 5.811859130859375),
    //                             lines_and_curves::Point::from(10.162320137023926, 5.186859130859375),
    //             ], true)
    //                     .iter()
    //                     .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
    //                     .collect(),
    //             ),
    //             ],
    //     )
    // ];

            let shapes = &mut vec![
        sign::Sign::from(
                lines_and_curves::Rectangle::from(
                        lines_and_curves::Point::from(1.0, 1.0),
                        lines_and_curves::Point::from(12.0, 7.0)
                ),
                vec![
                sign::Shape::from(
                        cnc_router::ToolType::FullCutText,
                        lines_and_curves::LineSegment::create_path(&vec![
                                lines_and_curves::Point::from(1.0, 1.0),
                                lines_and_curves::Point::from(11.0, 1.0),
                                lines_and_curves::Point::from(11.0, 3.799999952316284),
                                lines_and_curves::Point::from(1.0, 3.799999952316284),
                ], true)
                        .iter()
                        .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                        .collect(),
                ),
                sign::Shape::from(
                        cnc_router::ToolType::FullCutText,
                        lines_and_curves::LineSegment::create_path(&vec![
                                lines_and_curves::Point::from(3.7862682342529297, 2.481475591659546), // Softy
                                lines_and_curves::Point::from(3.7836318016052246, 2.532965898513794), // Softy
                                lines_and_curves::Point::from(3.7761292457580566, 2.5781662464141846), // Softy
                                lines_and_curves::Point::from(3.7642409801483154, 2.6178817749023438), // Softy
                                lines_and_curves::Point::from(3.7482635974884033, 2.6528213024139404), // Softy
                                lines_and_curves::Point::from(3.727102279663086, 2.685558795928955), // Softy
                                lines_and_curves::Point::from(3.702223062515259, 2.7136820554733276), // Softy
                                lines_and_curves::Point::from(3.6734256744384766, 2.7375906705856323), // Softy
                                lines_and_curves::Point::from(3.6403310298919678, 2.7574963569641113), // Softy
                                lines_and_curves::Point::from(3.605095386505127, 2.772490620613098), // Softy
                                lines_and_curves::Point::from(3.565920352935791, 2.783560276031494), // Softy
                                lines_and_curves::Point::from(3.522271156311035, 2.7904834747314453), // Softy
                                lines_and_curves::Point::from(3.473545789718628, 2.7928948402404785), // Softy
                                lines_and_curves::Point::from(3.282003402709961, 2.7928948402404785), // Softy
                                lines_and_curves::Point::from(3.282003402709961, 2.1578948497772217), // Softy
                                lines_and_curves::Point::from(3.454434871673584, 2.1578948497772217), // Softy
                                lines_and_curves::Point::from(3.5077226161956787, 2.160398483276367), // Softy
                                lines_and_curves::Point::from(3.5549521446228027, 2.167547106742859), // Softy
                                lines_and_curves::Point::from(3.596841812133789, 2.178899645805359), // Softy
                                lines_and_curves::Point::from(3.634033203125, 2.194161891937256), // Softy
                                lines_and_curves::Point::from(3.6691160202026367, 2.214688301086426), // Softy
                                lines_and_curves::Point::from(3.69940185546875, 2.239385724067688), // Softy
                                lines_and_curves::Point::from(3.7253406047821045, 2.268455743789673), // Softy
                                lines_and_curves::Point::from(3.747177839279175, 2.3023117780685425), // Softy
                                lines_and_curves::Point::from(3.763566017150879, 2.338520646095276), // Softy
                                lines_and_curves::Point::from(3.7757985591888428, 2.379915952682495), // Softy
                                lines_and_curves::Point::from(3.783541202545166, 2.427273988723755), // Softy
                                lines_and_curves::Point::from(3.7862682342529297, 2.481475591659546), // Softy
                ], true)
                        .iter()
                        .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                        .collect(),
                ),
                sign::Shape::from(
                        cnc_router::ToolType::FullCutText,
                        lines_and_curves::LineSegment::create_path(&vec![
                                lines_and_curves::Point::from(3.708521842956543, 2.4788695573806763), // Softy
                                lines_and_curves::Point::from(3.7049903869628906, 2.535441279411316), // Softy
                                lines_and_curves::Point::from(3.6952052116394043, 2.5819220542907715), // Softy
                                lines_and_curves::Point::from(3.680072784423828, 2.6200289726257324), // Softy
                                lines_and_curves::Point::from(3.664017915725708, 2.646289110183716), // Softy
                                lines_and_curves::Point::from(3.64490008354187, 2.6686108112335205), // Softy
                                lines_and_curves::Point::from(3.622563362121582, 2.687371253967285), // Softy
                                lines_and_curves::Point::from(3.596680164337158, 2.7027699947357178), // Softy
                                lines_and_curves::Point::from(3.559114456176758, 2.717331051826477), // Softy
                                lines_and_curves::Point::from(3.51473331451416, 2.726611018180847), // Softy
                                lines_and_curves::Point::from(3.4622530937194824, 2.7299160957336426), // Softy
                                lines_and_curves::Point::from(3.3558406829833984, 2.7299160957336426), // Softy
                                lines_and_curves::Point::from(3.3558406829833984, 2.2213079929351807), // Softy
                                lines_and_curves::Point::from(3.4457483291625977, 2.2213079929351807), // Softy
                                lines_and_curves::Point::from(3.4996941089630127, 2.2243669033050537), // Softy
                                lines_and_curves::Point::from(3.54518461227417, 2.2329306602478027), // Softy
                                lines_and_curves::Point::from(3.5834951400756836, 2.2462841272354126), // Softy
                                lines_and_curves::Point::from(3.615718364715576, 2.264012098312378), // Softy
                                lines_and_curves::Point::from(3.6427197456359863, 2.2860240936279297), // Softy
                                lines_and_curves::Point::from(3.6652185916900635, 2.31271755695343), // Softy
                                lines_and_curves::Point::from(3.683253526687622, 2.344421863555908), // Softy
                                lines_and_curves::Point::from(3.6967849731445313, 2.381953239440918), // Softy
                                lines_and_curves::Point::from(3.705437183380127, 2.426358222961426), // Softy
                                lines_and_curves::Point::from(3.708521842956543, 2.4788695573806763), // Softy
                ], true)
                        .iter()
                        .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                        .collect(),
                ),
                sign::Shape::from(
                        cnc_router::ToolType::FullCutText,
                        lines_and_curves::LineSegment::create_path(&vec![
                                lines_and_curves::Point::from(4.327017068862915, 2.396780014038086), // Softy
                                lines_and_curves::Point::from(4.323737382888794, 2.4480721950531006), // Softy
                                lines_and_curves::Point::from(4.314481258392334, 2.491915464401245), // Softy
                                lines_and_curves::Point::from(4.29987096786499, 2.5294697284698486), // Softy
                                lines_and_curves::Point::from(4.279002666473389, 2.5637080669403076), // Softy
                                lines_and_curves::Point::from(4.253760814666748, 2.591480255126953), // Softy
                                lines_and_curves::Point::from(4.223862171173096, 2.6135138273239136), // Softy
                                lines_and_curves::Point::from(4.190206527709961, 2.6294186115264893), // Softy
                                lines_and_curves::Point::from(4.152488946914673, 2.6392136812210083), // Softy
                                lines_and_curves::Point::from(4.109848737716675, 2.6426143646240234), // Softy
                                lines_and_curves::Point::from(4.065144777297974, 2.639192581176758), // Softy
                                lines_and_curves::Point::from(4.026315212249756, 2.6294246912002563), // Softy
                                lines_and_curves::Point::from(3.992360830307007, 2.613731026649475), // Softy
                                lines_and_curves::Point::from(3.962245464324951, 2.591774106025696), // Softy
                                lines_and_curves::Point::from(3.937089443206787, 2.5640755891799927), // Softy
                                lines_and_curves::Point::from(3.916569232940674, 2.5299041271209717), // Softy
                                lines_and_curves::Point::from(3.9023327827453613, 2.4923856258392334), // Softy
                                lines_and_curves::Point::from(3.8932864665985107, 2.4484052658081055), // Softy
                                lines_and_curves::Point::from(3.8900744915008545, 2.396780014038086), // Softy
                                lines_and_curves::Point::from(3.8919529914855957, 2.357905149459839), // Softy
                                lines_and_curves::Point::from(3.897336959838867, 2.323176622390747), // Softy
                                lines_and_curves::Point::from(3.9059276580810547, 2.2921048402786255), // Softy
                                lines_and_curves::Point::from(3.918250560760498, 2.262381076812744), // Softy
                                lines_and_curves::Point::from(3.9331777095794678, 2.2366130352020264), // Softy
                                lines_and_curves::Point::from(3.950664520263672, 2.2143585681915283), // Softy
                                lines_and_curves::Point::from(3.971144676208496, 2.194870948791504), // Softy
                                lines_and_curves::Point::from(3.9939396381378174, 2.1787803173065186), // Softy
                                lines_and_curves::Point::from(4.019289493560791, 2.1659300327301025), // Softy
                                lines_and_curves::Point::from(4.046237945556641, 2.1567559242248535), // Softy
                                lines_and_curves::Point::from(4.0752952098846436, 2.1511353254318237), // Softy
                                lines_and_curves::Point::from(4.106808423995972, 2.1492080688476563), // Softy
                                lines_and_curves::Point::from(4.140662670135498, 2.151169180870056), // Softy
                                lines_and_curves::Point::from(4.171133756637573, 2.1568230390548706), // Softy
                                lines_and_curves::Point::from(4.1986706256866455, 2.1659300327301025), // Softy
                                lines_and_curves::Point::from(4.224569797515869, 2.1788337230682373), // Softy
                                lines_and_curves::Point::from(4.247567176818848, 2.1949331760406494), // Softy
                                lines_and_curves::Point::from(4.267947196960449, 2.2143585681915283), // Softy
                                lines_and_curves::Point::from(4.28524923324585, 2.236618161201477), // Softy
                                lines_and_curves::Point::from(4.299887180328369, 2.262383222579956), // Softy
                                lines_and_curves::Point::from(4.31181526184082, 2.2921048402786255), // Softy
                                lines_and_curves::Point::from(4.320046901702881, 2.3231465816497803), // Softy
                                lines_and_curves::Point::from(4.325213193893433, 2.3578778505325317), // Softy
                                lines_and_curves::Point::from(4.327017068862915, 2.396780014038086), // Softy
                ], true)
                        .iter()
                        .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                        .collect(),
                ),
                sign::Shape::from(
                        cnc_router::ToolType::FullCutText,
                        lines_and_curves::LineSegment::create_path(&vec![
                                lines_and_curves::Point::from(3.964780330657959, 2.396780014038086), // Softy
                                lines_and_curves::Point::from(3.966620922088623, 2.3587218523025513), // Softy
                                lines_and_curves::Point::from(3.9718198776245117, 2.3257910013198853), // Softy
                                lines_and_curves::Point::from(3.9799821376800537, 2.297316789627075), // Softy
                                lines_and_curves::Point::from(3.9923102855682373, 2.270735502243042), // Softy
                                lines_and_curves::Point::from(4.007924795150757, 2.2492587566375732), // Softy
                                lines_and_curves::Point::from(4.026890516281128, 2.2321664094924927), // Softy
                                lines_and_curves::Point::from(4.049098968505859, 2.219770908355713), // Softy
                                lines_and_curves::Point::from(4.0759429931640625, 2.2119431495666504), // Softy
                                lines_and_curves::Point::from(4.108545780181885, 2.209146499633789), // Softy
                                lines_and_curves::Point::from(4.140855073928833, 2.2119301557540894), // Softy
                                lines_and_curves::Point::from(4.167645215988159, 2.219746708869934), // Softy
                                lines_and_curves::Point::from(4.18998384475708, 2.2321664094924927), // Softy
                                lines_and_curves::Point::from(4.20908260345459, 2.2492741346359253), // Softy
                                lines_and_curves::Point::from(4.224766254425049, 2.2707529067993164), // Softy
                                lines_and_curves::Point::from(4.237109422683716, 2.297316789627075), // Softy
                                lines_and_curves::Point::from(4.245271682739258, 2.3257908821105957), // Softy
                                lines_and_curves::Point::from(4.2504706382751465, 2.3587217330932617), // Softy
                                lines_and_curves::Point::from(4.2523112297058105, 2.396780014038086), // Softy
                                lines_and_curves::Point::from(4.250466346740723, 2.434548854827881), // Softy
                                lines_and_curves::Point::from(4.245264291763306, 2.4671103954315186), // Softy
                                lines_and_curves::Point::from(4.237109422683716, 2.4951571226119995), // Softy
                                lines_and_curves::Point::from(4.224798202514648, 2.5212903022766113), // Softy
                                lines_and_curves::Point::from(4.209189176559448, 2.542412042617798), // Softy
                                lines_and_curves::Point::from(4.190201044082642, 2.5592217445373535), // Softy
                                lines_and_curves::Point::from(4.168017625808716, 2.571355104446411), // Softy
                                lines_and_curves::Point::from(4.141039848327637, 2.579049825668335), // Softy
                                lines_and_curves::Point::from(4.108111381530762, 2.581807255744934), // Softy
                                lines_and_curves::Point::from(4.071592330932617, 2.5783716440200806), // Softy
                                lines_and_curves::Point::from(4.042198657989502, 2.5688118934631348), // Softy
                                lines_and_curves::Point::from(4.018435001373291, 2.553679347038269), // Softy
                                lines_and_curves::Point::from(3.99931001663208, 2.5327272415161133), // Softy
                                lines_and_curves::Point::from(3.985074758529663, 2.507762908935547), // Softy
                                lines_and_curves::Point::from(3.974275588989258, 2.477493643760681), // Softy
                                lines_and_curves::Point::from(3.9672932624816895, 2.4408985376358032), // Softy
                                lines_and_curves::Point::from(3.964780330657959, 2.396780014038086), // Softy
                ], true)
                        .iter()
                        .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                        .collect(),
                ),
                sign::Shape::from(
                        cnc_router::ToolType::FullCutText,
                        lines_and_curves::LineSegment::create_path(&vec![
                                lines_and_curves::Point::from(5.190043926239014, 2.1578948497772217), // Softy
                                lines_and_curves::Point::from(5.190043926239014, 2.7928948402404785), // Softy
                                lines_and_curves::Point::from(5.120550155639648, 2.7928948402404785), // Softy
                                lines_and_curves::Point::from(5.120550155639648, 2.4250118732452393), // Softy
                                lines_and_curves::Point::from(5.121635913848877, 2.364856243133545), // Softy
                                lines_and_curves::Point::from(5.124024391174316, 2.304483413696289), // Softy
                                lines_and_curves::Point::from(5.126196384429932, 2.2612669467926025), // Softy
                                lines_and_curves::Point::from(5.1231560707092285, 2.2612669467926025), // Softy
                                lines_and_curves::Point::from(4.778292655944824, 2.7928948402404785), // Softy
                                lines_and_curves::Point::from(4.693597316741943, 2.7928948402404785), // Softy
                                lines_and_curves::Point::from(4.693597316741943, 2.1578948497772217), // Softy
                                lines_and_curves::Point::from(4.7622222900390625, 2.1578948497772217), // Softy
                                lines_and_curves::Point::from(4.7622222900390625, 2.522737503051758), // Softy
                                lines_and_curves::Point::from(4.761136531829834, 2.5850647687911987), // Softy
                                lines_and_curves::Point::from(4.758313179016113, 2.642831563949585), // Softy
                                lines_and_curves::Point::from(4.75527286529541, 2.6912600994110107), // Softy
                                lines_and_curves::Point::from(4.758747577667236, 2.6912600994110107), // Softy
                                lines_and_curves::Point::from(5.104913711547852, 2.1578948497772217), // Softy
                                lines_and_curves::Point::from(5.190043926239014, 2.1578948497772217), // Softy
                ], true)
                        .iter()
                        .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                        .collect(),
                ),
                sign::Shape::from(
                        cnc_router::ToolType::FullCutText,
                        lines_and_curves::LineSegment::create_path(&vec![
                                lines_and_curves::Point::from(5.762933731079102, 2.396780014038086), // Softy
                                lines_and_curves::Point::from(5.7596540451049805, 2.4480721950531006), // Softy
                                lines_and_curves::Point::from(5.750397682189941, 2.491915464401245), // Softy
                                lines_and_curves::Point::from(5.735787391662598, 2.5294697284698486), // Softy
                                lines_and_curves::Point::from(5.714919567108154, 2.5637080669403076), // Softy
                                lines_and_curves::Point::from(5.689677715301514, 2.591480255126953), // Softy
                                lines_and_curves::Point::from(5.659778594970703, 2.6135138273239136), // Softy
                                lines_and_curves::Point::from(5.626122951507568, 2.6294186115264893), // Softy
                                lines_and_curves::Point::from(5.588405609130859, 2.6392136812210083), // Softy
                                lines_and_curves::Point::from(5.545765399932861, 2.6426143646240234), // Softy
                                lines_and_curves::Point::from(5.50106143951416, 2.639192581176758), // Softy
                                lines_and_curves::Point::from(5.462231636047363, 2.6294246912002563), // Softy
                                lines_and_curves::Point::from(5.428277492523193, 2.613731026649475), // Softy
                                lines_and_curves::Point::from(5.398161888122559, 2.591774106025696), // Softy
                                lines_and_curves::Point::from(5.3730058670043945, 2.5640755891799927), // Softy
                                lines_and_curves::Point::from(5.352485656738281, 2.5299041271209717), // Softy
                                lines_and_curves::Point::from(5.338249206542969, 2.492385745048523), // Softy
                                lines_and_curves::Point::from(5.329203128814697, 2.4484052658081055), // Softy
                                lines_and_curves::Point::from(5.325991153717041, 2.396780014038086), // Softy
                                lines_and_curves::Point::from(5.327869415283203, 2.357905149459839), // Softy
                                lines_and_curves::Point::from(5.333253860473633, 2.323176622390747), // Softy
                                lines_and_curves::Point::from(5.34184455871582, 2.2921048402786255), // Softy
                                lines_and_curves::Point::from(5.354167461395264, 2.262381076812744), // Softy
                                lines_and_curves::Point::from(5.369094371795654, 2.2366130352020264), // Softy
                                lines_and_curves::Point::from(5.3865814208984375, 2.2143585681915283), // Softy
                                lines_and_curves::Point::from(5.407061576843262, 2.194870948791504), // Softy
                                lines_and_curves::Point::from(5.429856300354004, 2.178780198097229), // Softy
                                lines_and_curves::Point::from(5.455206394195557, 2.1659300327301025), // Softy
                                lines_and_curves::Point::from(5.482154846191406, 2.1567559242248535), // Softy
                                lines_and_curves::Point::from(5.511211395263672, 2.1511354446411133), // Softy
                                lines_and_curves::Point::from(5.542725086212158, 2.1492080688476563), // Softy
                                lines_and_curves::Point::from(5.576579570770264, 2.151169180870056), // Softy
                                lines_and_curves::Point::from(5.60705041885376, 2.156822919845581), // Softy
                                lines_and_curves::Point::from(5.634587287902832, 2.1659300327301025), // Softy
                                lines_and_curves::Point::from(5.660486698150635, 2.1788337230682373), // Softy
                                lines_and_curves::Point::from(5.683484077453613, 2.1949331760406494), // Softy
                                lines_and_curves::Point::from(5.703864097595215, 2.2143585681915283), // Softy
                                lines_and_curves::Point::from(5.721165657043457, 2.236618161201477), // Softy
                                lines_and_curves::Point::from(5.735804080963135, 2.262383222579956), // Softy
                                lines_and_curves::Point::from(5.747732162475586, 2.2921048402786255), // Softy
                                lines_and_curves::Point::from(5.755963325500488, 2.3231465816497803), // Softy
                                lines_and_curves::Point::from(5.761129856109619, 2.3578778505325317), // Softy
                                lines_and_curves::Point::from(5.762933731079102, 2.396780014038086), // Softy
                ], true)
                        .iter()
                        .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                        .collect(),
                ),
                sign::Shape::from(
                        cnc_router::ToolType::FullCutText,
                        lines_and_curves::LineSegment::create_path(&vec![
                                lines_and_curves::Point::from(5.400696754455566, 2.396780014038086), // Softy
                                lines_and_curves::Point::from(5.402537822723389, 2.3587217330932617), // Softy
                                lines_and_curves::Point::from(5.407736778259277, 2.3257910013198853), // Softy
                                lines_and_curves::Point::from(5.41589879989624, 2.297316789627075), // Softy
                                lines_and_curves::Point::from(5.428226947784424, 2.270735502243042), // Softy
                                lines_and_curves::Point::from(5.443841457366943, 2.2492587566375732), // Softy
                                lines_and_curves::Point::from(5.4628071784973145, 2.2321664094924927), // Softy
                                lines_and_curves::Point::from(5.485015869140625, 2.219770908355713), // Softy
                                lines_and_curves::Point::from(5.511859893798828, 2.2119431495666504), // Softy
                                lines_and_curves::Point::from(5.544462203979492, 2.209146499633789), // Softy
                                lines_and_curves::Point::from(5.5767717361450195, 2.211930274963379), // Softy
                                lines_and_curves::Point::from(5.603561878204346, 2.219746708869934), // Softy
                                lines_and_curves::Point::from(5.6259002685546875, 2.2321664094924927), // Softy
                                lines_and_curves::Point::from(5.644999027252197, 2.2492741346359253), // Softy
                                lines_and_curves::Point::from(5.660682678222656, 2.2707529067993164), // Softy
                                lines_and_curves::Point::from(5.673026084899902, 2.297316789627075), // Softy
                                lines_and_curves::Point::from(5.681188583374023, 2.3257908821105957), // Softy
                                lines_and_curves::Point::from(5.686387062072754, 2.3587217330932617), // Softy
                                lines_and_curves::Point::from(5.688227653503418, 2.396780014038086), // Softy
                                lines_and_curves::Point::from(5.686383247375488, 2.434548854827881), // Softy
                                lines_and_curves::Point::from(5.681180953979492, 2.4671103954315186), // Softy
                                lines_and_curves::Point::from(5.673026084899902, 2.4951571226119995), // Softy
                                lines_and_curves::Point::from(5.660714626312256, 2.5212903022766113), // Softy
                                lines_and_curves::Point::from(5.645105838775635, 2.542412042617798), // Softy
                                lines_and_curves::Point::from(5.626117706298828, 2.5592217445373535), // Softy
                                lines_and_curves::Point::from(5.603934288024902, 2.571355104446411), // Softy
                                lines_and_curves::Point::from(5.576956748962402, 2.579049825668335), // Softy
                                lines_and_curves::Point::from(5.544028282165527, 2.581807255744934), // Softy
                                lines_and_curves::Point::from(5.507509231567383, 2.5783716440200806), // Softy
                                lines_and_curves::Point::from(5.478115558624268, 2.5688118934631348), // Softy
                                lines_and_curves::Point::from(5.454351902008057, 2.553679347038269), // Softy
                                lines_and_curves::Point::from(5.435226917266846, 2.5327272415161133), // Softy
                                lines_and_curves::Point::from(5.42099142074585, 2.507762908935547), // Softy
                                lines_and_curves::Point::from(5.410192012786865, 2.477493643760681), // Softy
                                lines_and_curves::Point::from(5.403209686279297, 2.4408984184265137), // Softy
                                lines_and_curves::Point::from(5.400696754455566, 2.396780014038086), // Softy
                ], true)
                        .iter()
                        .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                        .collect(),
                ),
                sign::Shape::from(
                        cnc_router::ToolType::FullCutText,
                        lines_and_curves::LineSegment::create_path(&vec![
                                lines_and_curves::Point::from(6.041343688964844, 2.208277940750122), // Softy
                                lines_and_curves::Point::from(6.019426345825195, 2.210880994796753), // Softy
                                lines_and_curves::Point::from(6.001413345336914, 2.218276619911194), // Softy
                                lines_and_curves::Point::from(5.9863996505737305, 2.2304290533065796), // Softy
                                lines_and_curves::Point::from(5.975533485412598, 2.2464523315429688), // Softy
                                lines_and_curves::Point::from(5.968417167663574, 2.2681576013565063), // Softy
                                lines_and_curves::Point::from(5.965768814086914, 2.297316789627075), // Softy
                                lines_and_curves::Point::from(5.965768814086914, 2.5774638652801514), // Softy
                                lines_and_curves::Point::from(6.105191230773926, 2.5774638652801514), // Softy
                                lines_and_curves::Point::from(6.105191230773926, 2.6339277029037476), // Softy
                                lines_and_curves::Point::from(5.965768814086914, 2.6339277029037476), // Softy
                                lines_and_curves::Point::from(5.965768814086914, 2.744249105453491), // Softy
                                lines_and_curves::Point::from(5.922335624694824, 2.744249105453491), // Softy
                                lines_and_curves::Point::from(5.893669128417969, 2.6408770084381104), // Softy
                                lines_and_curves::Point::from(5.825478553771973, 2.612645149230957), // Softy
                                lines_and_curves::Point::from(5.825478553771973, 2.5774638652801514), // Softy
                                lines_and_curves::Point::from(5.893234729766846, 2.5774638652801514), // Softy
                                lines_and_curves::Point::from(5.893234729766846, 2.2951451539993286), // Softy
                                lines_and_curves::Point::from(5.895616054534912, 2.2580355405807495), // Softy
                                lines_and_curves::Point::from(5.902039527893066, 2.2293550968170166), // Softy
                                lines_and_curves::Point::from(5.911694049835205, 2.207409143447876), // Softy
                                lines_and_curves::Point::from(5.925660133361816, 2.1880016326904297), // Softy
                                lines_and_curves::Point::from(5.942009449005127, 2.173192024230957), // Softy
                                lines_and_curves::Point::from(5.960990905761719, 2.1624553203582764), // Softy
                                lines_and_curves::Point::from(5.993059158325195, 2.152618646621704), // Softy
                                lines_and_curves::Point::from(6.029616355895996, 2.1492080688476563), // Softy
                                lines_and_curves::Point::from(6.073266983032227, 2.1531171798706055), // Softy
                                lines_and_curves::Point::from(6.09431791305542, 2.1577727794647217), // Softy
                                lines_and_curves::Point::from(6.108231544494629, 2.162672519683838), // Softy
                                lines_and_curves::Point::from(6.108231544494629, 2.218701958656311), // Softy
                                lines_and_curves::Point::from(6.077827453613281, 2.211318254470825), // Softy
                                lines_and_curves::Point::from(6.041343688964844, 2.208277940750122), // Softy
                ], true)
                        .iter()
                        .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                        .collect(),
                ),
                sign::Shape::from(
                        cnc_router::ToolType::FullCutText,
                        lines_and_curves::LineSegment::create_path(&vec![
                                lines_and_curves::Point::from(6.641596794128418, 2.1578948497772217), // Softy
                                lines_and_curves::Point::from(6.641596794128418, 2.7277443408966064), // Softy
                                lines_and_curves::Point::from(6.840522766113281, 2.7277443408966064), // Softy
                                lines_and_curves::Point::from(6.840522766113281, 2.7928948402404785), // Softy
                                lines_and_curves::Point::from(6.36752986907959, 2.7928948402404785), // Softy
                                lines_and_curves::Point::from(6.36752986907959, 2.7277443408966064), // Softy
                                lines_and_curves::Point::from(6.567324638366699, 2.7277443408966064), // Softy
                                lines_and_curves::Point::from(6.567324638366699, 2.1578948497772217), // Softy
                                lines_and_curves::Point::from(6.641596794128418, 2.1578948497772217), // Softy
                ], true)
                        .iter()
                        .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                        .collect(),
                ),
                sign::Shape::from(
                        cnc_router::ToolType::FullCutText,
                        lines_and_curves::LineSegment::create_path(&vec![
                                lines_and_curves::Point::from(7.3361005783081055, 2.396780014038086), // Softy
                                lines_and_curves::Point::from(7.332820892333984, 2.4480721950531006), // Softy
                                lines_and_curves::Point::from(7.323564529418945, 2.491915464401245), // Softy
                                lines_and_curves::Point::from(7.30895471572876, 2.5294697284698486), // Softy
                                lines_and_curves::Point::from(7.288086414337158, 2.5637080669403076), // Softy
                                lines_and_curves::Point::from(7.262844085693359, 2.591480255126953), // Softy
                                lines_and_curves::Point::from(7.232945442199707, 2.6135138273239136), // Softy
                                lines_and_curves::Point::from(7.199289798736572, 2.6294186115264893), // Softy
                                lines_and_curves::Point::from(7.161572456359863, 2.6392136812210083), // Softy
                                lines_and_curves::Point::from(7.118932247161865, 2.6426143646240234), // Softy
                                lines_and_curves::Point::from(7.074228286743164, 2.639192581176758), // Softy
                                lines_and_curves::Point::from(7.035398483276367, 2.6294246912002563), // Softy
                                lines_and_curves::Point::from(7.001444339752197, 2.613731026649475), // Softy
                                lines_and_curves::Point::from(6.9713287353515625, 2.591774106025696), // Softy
                                lines_and_curves::Point::from(6.946172714233398, 2.5640755891799927), // Softy
                                lines_and_curves::Point::from(6.925652503967285, 2.5299041271209717), // Softy
                                lines_and_curves::Point::from(6.911416530609131, 2.492385745048523), // Softy
                                lines_and_curves::Point::from(6.902369499206543, 2.4484052658081055), // Softy
                                lines_and_curves::Point::from(6.899158000946045, 2.396780014038086), // Softy
                                lines_and_curves::Point::from(6.901036262512207, 2.357905149459839), // Softy
                                lines_and_curves::Point::from(6.906420707702637, 2.323176622390747), // Softy
                                lines_and_curves::Point::from(6.915011405944824, 2.2921048402786255), // Softy
                                lines_and_curves::Point::from(6.927333831787109, 2.262381076812744), // Softy
                                lines_and_curves::Point::from(6.9422607421875, 2.2366130352020264), // Softy
                                lines_and_curves::Point::from(6.959748268127441, 2.2143585681915283), // Softy
                                lines_and_curves::Point::from(6.980228424072266, 2.194870948791504), // Softy
                                lines_and_curves::Point::from(7.003023624420166, 2.178780198097229), // Softy
                                lines_and_curves::Point::from(7.0283732414245605, 2.1659300327301025), // Softy
                                lines_and_curves::Point::from(7.05532169342041, 2.1567559242248535), // Softy
                                lines_and_curves::Point::from(7.084378719329834, 2.1511354446411133), // Softy
                                lines_and_curves::Point::from(7.115891933441162, 2.1492080688476563), // Softy
                                lines_and_curves::Point::from(7.149746417999268, 2.151169180870056), // Softy
                                lines_and_curves::Point::from(7.180217742919922, 2.156822919845581), // Softy
                                lines_and_curves::Point::from(7.207754135131836, 2.1659300327301025), // Softy
                                lines_and_curves::Point::from(7.233653545379639, 2.1788337230682373), // Softy
                                lines_and_curves::Point::from(7.256650924682617, 2.1949331760406494), // Softy
                                lines_and_curves::Point::from(7.277030944824219, 2.2143585681915283), // Softy
                                lines_and_curves::Point::from(7.294332504272461, 2.236618161201477), // Softy
                                lines_and_curves::Point::from(7.3089704513549805, 2.262383222579956), // Softy
                                lines_and_curves::Point::from(7.32089900970459, 2.2921048402786255), // Softy
                                lines_and_curves::Point::from(7.329130172729492, 2.3231465816497803), // Softy
                                lines_and_curves::Point::from(7.334296703338623, 2.3578778505325317), // Softy
                                lines_and_curves::Point::from(7.3361005783081055, 2.396780014038086), // Softy
                ], true)
                        .iter()
                        .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                        .collect(),
                ),
                sign::Shape::from(
                        cnc_router::ToolType::FullCutText,
                        lines_and_curves::LineSegment::create_path(&vec![
                                lines_and_curves::Point::from(6.97386360168457, 2.396780014038086), // Softy
                                lines_and_curves::Point::from(6.975704193115234, 2.3587217330932617), // Softy
                                lines_and_curves::Point::from(6.980903625488281, 2.3257910013198853), // Softy
                                lines_and_curves::Point::from(6.989065647125244, 2.297316789627075), // Softy
                                lines_and_curves::Point::from(7.001394271850586, 2.270735502243042), // Softy
                                lines_and_curves::Point::from(7.017008304595947, 2.2492587566375732), // Softy
                                lines_and_curves::Point::from(7.035974025726318, 2.2321664094924927), // Softy
                                lines_and_curves::Point::from(7.058182716369629, 2.219770908355713), // Softy
                                lines_and_curves::Point::from(7.085026741027832, 2.2119431495666504), // Softy
                                lines_and_curves::Point::from(7.117629051208496, 2.209146499633789), // Softy
                                lines_and_curves::Point::from(7.149939060211182, 2.2119301557540894), // Softy
                                lines_and_curves::Point::from(7.17672872543335, 2.219746708869934), // Softy
                                lines_and_curves::Point::from(7.19906759262085, 2.2321664094924927), // Softy
                                lines_and_curves::Point::from(7.218166351318359, 2.249274253845215), // Softy
                                lines_and_curves::Point::from(7.233850002288818, 2.2707529067993164), // Softy
                                lines_and_curves::Point::from(7.246192932128906, 2.297316789627075), // Softy
                                lines_and_curves::Point::from(7.254354476928711, 2.3257908821105957), // Softy
                                lines_and_curves::Point::from(7.259553909301758, 2.3587217330932617), // Softy
                                lines_and_curves::Point::from(7.261394500732422, 2.396780014038086), // Softy
                                lines_and_curves::Point::from(7.259550094604492, 2.434548854827881), // Softy
                                lines_and_curves::Point::from(7.254347801208496, 2.4671103954315186), // Softy
                                lines_and_curves::Point::from(7.246192932128906, 2.4951571226119995), // Softy
                                lines_and_curves::Point::from(7.233880996704102, 2.5212901830673218), // Softy
                                lines_and_curves::Point::from(7.218273162841797, 2.542412042617798), // Softy
                                lines_and_curves::Point::from(7.199284553527832, 2.5592217445373535), // Softy
                                lines_and_curves::Point::from(7.1771016120910645, 2.571355104446411), // Softy
                                lines_and_curves::Point::from(7.150123596191406, 2.579049825668335), // Softy
                                lines_and_curves::Point::from(7.117195129394531, 2.581807255744934), // Softy
                                lines_and_curves::Point::from(7.080676078796387, 2.5783716440200806), // Softy
                                lines_and_curves::Point::from(7.0512824058532715, 2.5688118934631348), // Softy
                                lines_and_curves::Point::from(7.0275187492370605, 2.553679347038269), // Softy
                                lines_and_curves::Point::from(7.00839376449585, 2.5327272415161133), // Softy
                                lines_and_curves::Point::from(6.9941582679748535, 2.507762908935547), // Softy
                                lines_and_curves::Point::from(6.983358860015869, 2.477493643760681), // Softy
                                lines_and_curves::Point::from(6.976376533508301, 2.4408984184265137), // Softy
                                lines_and_curves::Point::from(6.97386360168457, 2.396780014038086), // Softy
                ], true)
                        .iter()
                        .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                        .collect(),
                ),
                sign::Shape::from(
                        cnc_router::ToolType::FullCutText,
                        lines_and_curves::LineSegment::create_path(&vec![
                                lines_and_curves::Point::from(7.853829860687256, 2.6339277029037476), // Softy
                                lines_and_curves::Point::from(7.7812957763671875, 2.6339277029037476), // Softy
                                lines_and_curves::Point::from(7.7812957763671875, 2.3833155632019043), // Softy
                                lines_and_curves::Point::from(7.779573440551758, 2.345740556716919), // Softy
                                lines_and_curves::Point::from(7.774782657623291, 2.314252257347107), // Softy
                                lines_and_curves::Point::from(7.767396926879883, 2.2879786491394043), // Softy
                                lines_and_curves::Point::from(7.755893707275391, 2.263698697090149), // Softy
                                lines_and_curves::Point::from(7.74082088470459, 2.2443138360977173), // Softy
                                lines_and_curves::Point::from(7.72200870513916, 2.2291259765625), // Softy
                                lines_and_curves::Point::from(7.700119972229004, 2.21855092048645), // Softy
                                lines_and_curves::Point::from(7.67242956161499, 2.2116622924804688), // Softy
                                lines_and_curves::Point::from(7.637530326843262, 2.209146499633789), // Softy
                                lines_and_curves::Point::from(7.60988712310791, 2.2112696170806885), // Softy
                                lines_and_curves::Point::from(7.587644100189209, 2.2171319723129272), // Softy
                                lines_and_curves::Point::from(7.569758892059326, 2.226235508918762), // Softy
                                lines_and_curves::Point::from(7.555440902709961, 2.23846435546875), // Softy
                                lines_and_curves::Point::from(7.54128360748291, 2.259644865989685), // Softy
                                lines_and_curves::Point::from(7.531981945037842, 2.288456678390503), // Softy
                                lines_and_curves::Point::from(7.528512001037598, 2.327286124229431), // Softy
                                lines_and_curves::Point::from(7.528512001037598, 2.6339277029037476), // Softy
                                lines_and_curves::Point::from(7.455543518066406, 2.6339277029037476), // Softy
                                lines_and_curves::Point::from(7.455543518066406, 2.322074055671692), // Softy
                                lines_and_curves::Point::from(7.45796012878418, 2.2823996543884277), // Softy
                                lines_and_curves::Point::from(7.464625358581543, 2.2501049041748047), // Softy
                                lines_and_curves::Point::from(7.4748711585998535, 2.2239140272140503), // Softy
                                lines_and_curves::Point::from(7.489956855773926, 2.200528621673584), // Softy
                                lines_and_curves::Point::from(7.508994102478027, 2.1818665266036987), // Softy
                                lines_and_curves::Point::from(7.532421112060547, 2.167450189590454), // Softy
                                lines_and_curves::Point::from(7.558521270751953, 2.157707929611206), // Softy
                                lines_and_curves::Point::from(7.589757919311523, 2.1514501571655273), // Softy
                                lines_and_curves::Point::from(7.627106189727783, 2.1492080688476563), // Softy
                                lines_and_curves::Point::from(7.659493923187256, 2.151366710662842), // Softy
                                lines_and_curves::Point::from(7.689433574676514, 2.15767765045166), // Softy
                                lines_and_curves::Point::from(7.717458248138428, 2.1683478355407715), // Softy
                                lines_and_curves::Point::from(7.742205619812012, 2.183086395263672), // Softy
                                lines_and_curves::Point::from(7.763496398925781, 2.2019892930984497), // Softy
                                lines_and_curves::Point::from(7.780426979064941, 2.224782705307007), // Softy
                                lines_and_curves::Point::from(7.784336090087891, 2.224782705307007), // Softy
                                lines_and_curves::Point::from(7.794760227203369, 2.1578948497772217), // Softy
                                lines_and_curves::Point::from(7.853829860687256, 2.1578948497772217), // Softy
                                lines_and_curves::Point::from(7.853829860687256, 2.6339277029037476), // Softy
                ], true)
                        .iter()
                        .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                        .collect(),
                ),
                sign::Shape::from(
                        cnc_router::ToolType::FullCutText,
                        lines_and_curves::LineSegment::create_path(&vec![
                                lines_and_curves::Point::from(8.196955680847168, 2.1492080688476563), // Softy
                                lines_and_curves::Point::from(8.235944747924805, 2.1509673595428467), // Softy
                                lines_and_curves::Point::from(8.266666889190674, 2.1557230949401855), // Softy
                                lines_and_curves::Point::from(8.295005798339844, 2.163783073425293), // Softy
                                lines_and_curves::Point::from(8.319872856140137, 2.1743996143341064), // Softy
                                lines_and_curves::Point::from(8.319872856140137, 2.238681435585022), // Softy
                                lines_and_curves::Point::from(8.29425048828125, 2.228716492652893), // Softy
                                lines_and_curves::Point::from(8.2647123336792, 2.219787836074829), // Softy
                                lines_and_curves::Point::from(8.233269691467285, 2.213857650756836), // Softy
                                lines_and_curves::Point::from(8.196521282196045, 2.2117526531219482), // Softy
                                lines_and_curves::Point::from(8.16558837890625, 2.214338779449463), // Softy
                                lines_and_curves::Point::from(8.139620304107666, 2.221632242202759), // Softy
                                lines_and_curves::Point::from(8.11768913269043, 2.2332522869110107), // Softy
                                lines_and_curves::Point::from(8.098786354064941, 2.2493592500686646), // Softy
                                lines_and_curves::Point::from(8.083031177520752, 2.269894242286682), // Softy
                                lines_and_curves::Point::from(8.070346355438232, 2.2955795526504517), // Softy
                                lines_and_curves::Point::from(8.061872005462646, 2.323237657546997), // Softy
                                lines_and_curves::Point::from(8.056433200836182, 2.3558311462402344), // Softy
                                lines_and_curves::Point::from(8.054493427276611, 2.3941738605499268), // Softy
                                lines_and_curves::Point::from(8.056509017944336, 2.434282064437866), // Softy
                                lines_and_curves::Point::from(8.062116146087646, 2.4678354263305664), // Softy
                                lines_and_curves::Point::from(8.070780754089355, 2.4958086013793945), // Softy
                                lines_and_curves::Point::from(8.083837509155273, 2.521672487258911), // Softy
                                lines_and_curves::Point::from(8.100158214569092, 2.5422768592834473), // Softy
                                lines_and_curves::Point::from(8.119861125946045, 2.5583531856536865), // Softy
                                lines_and_curves::Point::from(8.142627716064453, 2.5698200464248657), // Softy
                                lines_and_curves::Point::from(8.16973876953125, 2.5770596265792847), // Softy
                                lines_and_curves::Point::from(8.202167510986328, 2.5796356201171875), // Softy
                                lines_and_curves::Point::from(8.227272987365723, 2.577972650527954), // Softy
                                lines_and_curves::Point::from(8.25537395477295, 2.572686195373535), // Softy
                                lines_and_curves::Point::from(8.305539608001709, 2.557050108909607), // Softy
                                lines_and_curves::Point::from(8.327256679534912, 2.6174228191375732), // Softy
                                lines_and_curves::Point::from(8.304255485534668, 2.6269350051879883), // Softy
                                lines_and_curves::Point::from(8.27274751663208, 2.635447859764099), // Softy
                                lines_and_curves::Point::from(8.239119529724121, 2.640802264213562), // Softy
                                lines_and_curves::Point::from(8.20390510559082, 2.6426143646240234), // Softy
                                lines_and_curves::Point::from(8.161406517028809, 2.6395368576049805), // Softy
                                lines_and_curves::Point::from(8.12312364578247, 2.630624294281006), // Softy
                                lines_and_curves::Point::from(8.088371753692627, 2.616119861602783), // Softy
                                lines_and_curves::Point::from(8.057222366333008, 2.595536708831787), // Softy
                                lines_and_curves::Point::from(8.030770778656006, 2.568660020828247), // Softy
                                lines_and_curves::Point::from(8.008670806884766, 2.534681797027588), // Softy
                                lines_and_curves::Point::from(7.996700286865234, 2.507145881652832), // Softy
                                lines_and_curves::Point::from(7.987647533416748, 2.4749135971069336), // Softy
                                lines_and_curves::Point::from(7.981849193572998, 2.437244176864624), // Softy
                                lines_and_curves::Point::from(7.979787349700928, 2.3933053016662598), // Softy
                                lines_and_curves::Point::from(7.983172416687012, 2.3387457132339478), // Softy
                                lines_and_curves::Point::from(7.992575168609619, 2.293669104576111), // Softy
                                lines_and_curves::Point::from(8.007150650024414, 2.2564892768859863), // Softy
                                lines_and_curves::Point::from(8.028221607208252, 2.222844123840332), // Softy
                                lines_and_curves::Point::from(8.053597450256348, 2.1961896419525146), // Softy
                                lines_and_curves::Point::from(8.083593845367432, 2.1757025718688965), // Softy
                                lines_and_curves::Point::from(8.117226600646973, 2.161238431930542), // Softy
                                lines_and_curves::Point::from(8.154760360717773, 2.1523077487945557), // Softy
                                lines_and_curves::Point::from(8.196955680847168, 2.1492080688476563), // Softy
                ], true)
                        .iter()
                        .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                        .collect(),
                ),
                sign::Shape::from(
                        cnc_router::ToolType::FullCutText,
                        lines_and_curves::LineSegment::create_path(&vec![
                                lines_and_curves::Point::from(8.50446605682373, 2.833722472190857), // Softy
                                lines_and_curves::Point::from(8.432365894317627, 2.833722472190857), // Softy
                                lines_and_curves::Point::from(8.432365894317627, 2.1578948497772217), // Softy
                                lines_and_curves::Point::from(8.50446605682373, 2.1578948497772217), // Softy
                                lines_and_curves::Point::from(8.50446605682373, 2.4067697525024414), // Softy
                                lines_and_curves::Point::from(8.506159782409668, 2.4443514347076416), // Softy
                                lines_and_curves::Point::from(8.510873794555664, 2.475919485092163), // Softy
                                lines_and_curves::Point::from(8.518147468566895, 2.5023237466812134), // Softy
                                lines_and_curves::Point::from(8.529528617858887, 2.526752471923828), // Softy
                                lines_and_curves::Point::from(8.54452896118164, 2.5462799072265625), // Softy
                                lines_and_curves::Point::from(8.563318252563477, 2.561610698699951), // Softy
                                lines_and_curves::Point::from(8.585223197937012, 2.5723063945770264), // Softy
                                lines_and_curves::Point::from(8.612913131713867, 2.5792665481567383), // Softy
                                lines_and_curves::Point::from(8.647797107696533, 2.581807255744934), // Softy
                                lines_and_curves::Point::from(8.675899505615234, 2.5796620845794678), // Softy
                                lines_and_curves::Point::from(8.698458194732666, 2.5737457275390625), // Softy
                                lines_and_curves::Point::from(8.71654224395752, 2.5645734071731567), // Softy
                                lines_and_curves::Point::from(8.730972290039063, 2.5522724390029907), // Softy
                                lines_and_curves::Point::from(8.745261192321777, 2.5309410095214844), // Softy
                                lines_and_curves::Point::from(8.754630088806152, 2.502044916152954), // Softy
                                lines_and_curves::Point::from(8.758118152618408, 2.463233470916748), // Softy
                                lines_and_curves::Point::from(8.758118152618408, 2.1578948497772217), // Softy
                                lines_and_curves::Point::from(8.829349517822266, 2.1578948497772217), // Softy
                                lines_and_curves::Point::from(8.829349517822266, 2.4680111408233643), // Softy
                                lines_and_curves::Point::from(8.826920509338379, 2.5082849264144897), // Softy
                                lines_and_curves::Point::from(8.820244312286377, 2.5408430099487305), // Softy
                                lines_and_curves::Point::from(8.81002140045166, 2.5670398473739624), // Softy
                                lines_and_curves::Point::from(8.794924259185791, 2.5904093980789185), // Softy
                                lines_and_curves::Point::from(8.775819778442383, 2.6090738773345947), // Softy
                                lines_and_curves::Point::from(8.752254962921143, 2.623503565788269), // Softy
                                lines_and_curves::Point::from(8.726009368896484, 2.6332294940948486), // Softy
                                lines_and_curves::Point::from(8.694426536560059, 2.639495372772217), // Softy
                                lines_and_curves::Point::from(8.65648365020752, 2.641745686531067), // Softy
                                lines_and_curves::Point::from(8.624519348144531, 2.6394823789596558), // Softy
                                lines_and_curves::Point::from(8.594807624816895, 2.6328418254852295), // Softy
                                lines_and_curves::Point::from(8.567076683044434, 2.6217715740203857), // Softy
                                lines_and_curves::Point::from(8.5429048538208, 2.6067816019058228), // Softy
                                lines_and_curves::Point::from(8.52216386795044, 2.5876764059066772), // Softy
                                lines_and_curves::Point::from(8.505334377288818, 2.5644338130950928), // Softy
                                lines_and_curves::Point::from(8.500556945800781, 2.5644338130950928), // Softy
                                lines_and_curves::Point::from(8.503597259521484, 2.5967918634414673), // Softy
                                lines_and_curves::Point::from(8.50446605682373, 2.6317559480667114), // Softy
                                lines_and_curves::Point::from(8.50446605682373, 2.833722472190857), // Softy
                ], true)
                        .iter()
                        .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                        .collect(),
                ),
                ],
        )
];


    gc.build_gcode_smart_path(
        true,
        &shapes,
        &vec![
            (cnc_router::ToolType::FullCutText, 0.009),
            (cnc_router::ToolType::Braille, 0.0),
        ],
    );
}
