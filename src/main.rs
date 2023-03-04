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
            cnc_router::Tool {
                name: String::from("Exact Bit"),
                index_in_machine: 1,
                offset_length: 0.5,
                radius: 0.0,
                length: 0.0,
                front_angle: 0.0,
                back_angle: 0.0,
                orientation: 0.0,
                tool_type: cnc_router::ToolType::Text,
                // smoothness: cnc_router::Smoothness::Medium,
                smoothness:        cnc_router::Smoothness::Finish,
                feed_rate_of_cut: feed_rate_of_cut,
                feed_rate_of_drill: feed_rate_of_drill,
                offset: 1.0,
            },
            cnc_router::Tool {
                name: String::from("Quarter Inch Bit"),
                index_in_machine: 2,
                offset_length: 0.5,
                radius: 0.25/2.0,
                length: 0.0,
                front_angle: 0.0,
                back_angle: 0.0,
                orientation: 0.0,
                tool_type: cnc_router::ToolType::PartialCutBroad,
                // smoothness: cnc_router::Smoothness::Medium,
                smoothness:        cnc_router::Smoothness::Finish,
                feed_rate_of_cut: feed_rate_of_cut,
                feed_rate_of_drill: feed_rate_of_drill,
                offset: 0.8,
            },
            cnc_router::Tool {
                name: String::from("Quarter Inch"),
                index_in_machine: 2,
                offset_length: 0.5,
                radius: 0.25/2.0,
                length: 0.0,
                front_angle: 0.0,
                back_angle: 0.0,
                orientation: 0.0,
                tool_type: cnc_router::ToolType::Text,
                // smoothness: cnc_router::Smoothness::Medium,
                smoothness:        cnc_router::Smoothness::Finish,
                feed_rate_of_cut: feed_rate_of_cut,
                feed_rate_of_drill: feed_rate_of_drill,
                offset: 0.8,
            },

            // cnc_router::Tool {
            //     name: String::from("Text Bit"),
            //     index_in_machine: 1,
            //     offset_length: 0.5,
            //     radius: 1.0,
            //     length: 0.0,
            //     front_angle: 0.0,
            //     back_angle: 0.0,
            //     orientation: 0.0,
            //     tool_type: cnc_router::ToolType::Text,
            //     // smoothness: cnc_router::Smoothness::Medium,
            //     smoothness:        cnc_router::Smoothness::Finish,
            //     feed_rate_of_cut: feed_rate_of_cut,
            //     feed_rate_of_drill: feed_rate_of_drill,
            //     offset: 1.0,
            // },



            // cnc_router::Tool {
            //     name: String::from("Quarter Inch Bit"),
            //     index_in_machine: 4,
            //     offset_length: 0.5,
            //     radius: 0.25/2.0,
            //     length: 0.008,
            //     front_angle: 0.0,
            //     back_angle: 0.0,
            //     orientation: 0.0,
            //     tool_type: cnc_router::ToolType::PartialCutBroad,
            //     // smoothness: cnc_router::Smoothness::Medium,
            //     smoothness:        cnc_router::Smoothness::Finish,
            //     feed_rate_of_cut: feed_rate_of_cut,
            //     feed_rate_of_drill: feed_rate_of_drill,
            //     offset: 0.2,
            // },
            // cnc_router::Tool {
            //     name:              String::from("1/8 Inch Bit"),
            //     index_in_machine:  3,
            //     offset_length:     0.5,
            //     radius:            0.125/2.0,
            //     length:            0.0,
            //     front_angle:       0.0,
            //     back_angle:        0.0,
            //     orientation:       0.0,
            //     tool_type:         cnc_router::ToolType::FullCutBroad,
            //     // smoothness:        cnc_router::Smoothness::Medium,
            //     smoothness:        cnc_router::Smoothness::Finish,
            //     feed_rate_of_cut: feed_rate_of_cut,
            //     feed_rate_of_drill: feed_rate_of_drill,
            //     offset: 0.2,
            // },
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
            // cnc_router::Tool {
            //     name:              String::from("Text / Lettering Bit"),
            //     index_in_machine:  7,
            //     offset_length:     0.6,
            //     radius:            0.005/2.0,
            //     length:            0.0,
            //     front_angle:       0.0,
            //     back_angle:        0.0,
            //     orientation:       0.0,
            //     tool_type:         cnc_router::ToolType::Text,
            //     smoothness:        cnc_router::Smoothness::Finish,
            //     feed_rate_of_cut:  feed_rate_of_cut,
            //     feed_rate_of_drill:feed_rate_of_drill,
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

    // let shapes = &mut vec![
    //     sign::Sign::from(
    //         lines_and_curves::Rectangle::from(
    //             lines_and_curves::Point::from(1.0, 1.0),
    //             lines_and_curves::Point::from(7.0, 7.0)
    //         ),
    //         vec![
    //             sign::Shape::from(
    //                 cnc_router::ToolType::Text,
    //                 lines_and_curves::LineSegment::create_path(&vec![
    //                     lines_and_curves::Point::from(4.0, 2.0), // Top
    //                     lines_and_curves::Point::from(6.0, 4.0), // Right
    //                     lines_and_curves::Point::from(4.0, 6.0), // Bottom
    //                     lines_and_curves::Point::from(2.0, 4.0), // Left
    //                 ], true)
    //                 .iter()
    //                 .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
    //                 .collect(),
    //             ),
    //         ],
    //     ),
    // ];

    // let shapes = &mut vec![
    //     sign::Sign::from(
    //         lines_and_curves::Rectangle::from(
    //             lines_and_curves::Point::from(1.0, 1.0),
    //             lines_and_curves::Point::from(7.0, 7.0)
    //         ),
    //         vec![
    //             sign::Shape::from(
    //                 cnc_router::ToolType::Text,
    //                 lines_and_curves::LineSegment::create_path(&vec![
    //                     lines_and_curves::Point::from(4.0, 4.0), // Top
    //                     lines_and_curves::Point::from(2.0, 2.0), // Bottom Left
    //                     lines_and_curves::Point::from(6.0, 2.0), // Bottom Right
    //                 ], true)
    //                 .iter()
    //                 .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
    //                 .collect(),
    //             ),
    //             sign::Shape::from(
    //                 cnc_router::ToolType::Text,
    //                 lines_and_curves::LineSegment::create_path(&vec![
    //                     lines_and_curves::Point::from(4.0, 4.0), // Bottom
    //                     lines_and_curves::Point::from(2.0, 6.0), // Top Left
    //                     lines_and_curves::Point::from(6.0, 6.0), // Top Right
    //                 ], true)
    //                 .iter()
    //                 .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
    //                 .collect(),
    //             ),
    //         ],
    //     ),
    // ];

    /*
        let shapes = &mut vec![
            sign::Sign::from(
                lines_and_curves::Rectangle::from(
                        lines_and_curves::Point::from(1.0, 1.0),
                        lines_and_curves::Point::from(10.0, 7.0)
                ),
                vec![
                    sign::Shape::from(
                            cnc_router::ToolType::Text,
                            lines_and_curves::LineSegment::create_path(&vec![
                                lines_and_curves::Point::from(2.0, 6.0),
                                lines_and_curves::Point::from(5.0, 6.0),
                                lines_and_curves::Point::from(5.0, 5.0),
                                lines_and_curves::Point::from(3.0, 5.0),
                                lines_and_curves::Point::from(3.0, 3.0),
                                lines_and_curves::Point::from(5.0, 3.0),
                                lines_and_curves::Point::from(5.0, 2.0),
                                lines_and_curves::Point::from(2.0, 2.0), // 8
                            ], true,)
                                .iter()
                                .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                                .collect(),
                    ),
                    sign::Shape::from(
                            cnc_router::ToolType::Text,
                            lines_and_curves::LineSegment::create_path(&vec![
                                lines_and_curves::Point::from(9.0, 5.8),
                                lines_and_curves::Point::from(5.01, 5.8),
                                lines_and_curves::Point::from(5.01, 5.4),
                                lines_and_curves::Point::from(8.0, 5.4),
                                lines_and_curves::Point::from(8.0, 2.8),
                                lines_and_curves::Point::from(5.01, 2.8),
                                lines_and_curves::Point::from(5.01, 2.4),
                                lines_and_curves::Point::from(9.0, 2.4), // 8
                            ], true,)
                                .iter()
                                .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                                .collect(),
                    ),
                ],
            )
        ];
    */

        /*
        let shapes = &mut vec![
            sign::Sign::from(
                lines_and_curves::Rectangle::from(
                    lines_and_curves::Point::from(1.0, 1.0),
                    lines_and_curves::Point::from(10.0, 7.0)
                ),
                vec![
                    sign::Shape::from(
                        cnc_router::ToolType::Text,
                        lines_and_curves::LineSegment::create_path(&vec![
                            lines_and_curves::Point::from(2.0, 6.0-0.01),
                            lines_and_curves::Point::from(5.0, 6.0+0.01),
                            lines_and_curves::Point::from(5.0, 5.0+0.01),
                            lines_and_curves::Point::from(3.0, 5.0),
                            lines_and_curves::Point::from(3.0, 3.0),
                            lines_and_curves::Point::from(5.0, 3.0+0.01),
                            lines_and_curves::Point::from(4.95, 2.0+0.00),
                            lines_and_curves::Point::from(2.0, 2.0-0.01),
                        ], true,)
                            .iter()
                            .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                            .collect(),
                    ),
                    sign::Shape::from(
                        cnc_router::ToolType::Text,
                        lines_and_curves::LineSegment::create_path(&vec![
                            lines_and_curves::Point::from(4.0, 2.5),
                            lines_and_curves::Point::from(4.0, 5.5),
                            lines_and_curves::Point::from(6.0, 5.5+0.01),
                            lines_and_curves::Point::from(6.0, 2.5+0.01),
                        ], true,)
                            .iter()
                            .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                            .collect(),
                    ),
                ],
            )
        ];
        */

    let odd_numbers_dy = 0.0;

    let shapes = vec![
vec![
AllIntersections::SoftLineSegment(LineSegment::from(Point::from(1.0, 6.25), Point::from(8.625, 6.25))),
AllIntersections::SoftLineSegment(LineSegment::from(Point::from(8.625, 6.25), Point::from(8.625, 1.0))),
AllIntersections::SoftLineSegment(LineSegment::from(Point::from(8.625, 1.0), Point::from(1.0, 1.0))),
AllIntersections::SoftLineSegment(LineSegment::from(Point::from(1.0, 1.0), Point::from(1.0, 6.25))),
],
vec![
AllIntersections::LineSegment(LineSegment::from(Point::from(3.6921167373657227, 3.125+odd_numbers_dy), Point::from(3.5751538276672363, 3.125+odd_numbers_dy))),
AllIntersections::LineSegment(LineSegment::from(Point::from(3.5751538276672363, 3.125+odd_numbers_dy), Point::from(3.5751538276672363, 4.022400856018066+odd_numbers_dy))),
AllIntersections::LineSegment(LineSegment::from(Point::from(3.5751538276672363, 4.022400856018066+odd_numbers_dy), Point::from(3.2605161666870117, 4.022400856018066+odd_numbers_dy))),
AllIntersections::LineSegment(LineSegment::from(Point::from(3.2605161666870117, 4.022400856018066+odd_numbers_dy), Point::from(3.2605161666870117, 4.125+odd_numbers_dy))),
AllIntersections::LineSegment(LineSegment::from(Point::from(3.2605161666870117, 4.125+odd_numbers_dy), Point::from(4.0053863525390625, 4.125+odd_numbers_dy))),
AllIntersections::LineSegment(LineSegment::from(Point::from(4.0053863525390625, 4.125+odd_numbers_dy), Point::from(4.0053863525390625, 4.022400856018066+odd_numbers_dy))),
AllIntersections::LineSegment(LineSegment::from(Point::from(4.0053863525390625, 4.022400856018066+odd_numbers_dy), Point::from(3.6921167373657227, 4.022400856018066+odd_numbers_dy))),
AllIntersections::LineSegment(LineSegment::from(Point::from(3.6921167373657227, 4.022400856018066+odd_numbers_dy), Point::from(3.6921167373657227, 3.125+odd_numbers_dy))),
AllIntersections::LineSegment(LineSegment::from(Point::from(3.6921167373657227, 3.125+odd_numbers_dy), Point::from(3.6921167373657227, 3.125+odd_numbers_dy))),
],
vec![
AllIntersections::LineSegment(LineSegment::from(Point::from(4.713320732116699, 3.125), Point::from(4.156548976898193, 3.125))),
AllIntersections::LineSegment(LineSegment::from(Point::from(4.156548976898193, 3.125), Point::from(4.156548976898193, 4.125))),
AllIntersections::LineSegment(LineSegment::from(Point::from(4.156548976898193, 4.125), Point::from(4.713320732116699, 4.125))),
AllIntersections::LineSegment(LineSegment::from(Point::from(4.713320732116699, 4.125), Point::from(4.713320732116699, 4.022400856018066))),
AllIntersections::LineSegment(LineSegment::from(Point::from(4.713320732116699, 4.022400856018066), Point::from(4.272828102111816, 4.022400856018066))),
AllIntersections::LineSegment(LineSegment::from(Point::from(4.272828102111816, 4.022400856018066), Point::from(4.272828102111816, 3.698871612548828))),
AllIntersections::LineSegment(LineSegment::from(Point::from(4.272828102111816, 3.698871612548828), Point::from(4.688013076782227, 3.698871612548828))),
AllIntersections::LineSegment(LineSegment::from(Point::from(4.688013076782227, 3.698871612548828), Point::from(4.688013076782227, 3.597640037536621))),
AllIntersections::LineSegment(LineSegment::from(Point::from(4.688013076782227, 3.597640037536621), Point::from(4.272828102111816, 3.597640037536621))),
AllIntersections::LineSegment(LineSegment::from(Point::from(4.272828102111816, 3.597640037536621), Point::from(4.272828102111816, 3.2275991439819336))),
AllIntersections::LineSegment(LineSegment::from(Point::from(4.272828102111816, 3.2275991439819336), Point::from(4.713320732116699, 3.2275991439819336))),
AllIntersections::LineSegment(LineSegment::from(Point::from(4.713320732116699, 3.2275991439819336), Point::from(4.713320732116699, 3.125))),
AllIntersections::LineSegment(LineSegment::from(Point::from(4.713320732116699, 3.125), Point::from(4.713320732116699, 3.125))),
],
vec![
AllIntersections::LineSegment(LineSegment::from(Point::from(5.602513313293457, 3.125+odd_numbers_dy), Point::from(5.470502853393555, 3.125+odd_numbers_dy))),
AllIntersections::LineSegment(LineSegment::from(Point::from(5.470502853393555, 3.125+odd_numbers_dy), Point::from(5.200325012207031, 3.565492630004883+odd_numbers_dy))),
AllIntersections::LineSegment(LineSegment::from(Point::from(5.200325012207031, 3.565492630004883+odd_numbers_dy), Point::from(4.9260430335998535, 3.125+odd_numbers_dy))),
AllIntersections::LineSegment(LineSegment::from(Point::from(4.9260430335998535, 3.125+odd_numbers_dy), Point::from(4.802239894866943, 3.125+odd_numbers_dy))),
AllIntersections::LineSegment(LineSegment::from(Point::from(4.802239894866943, 3.125+odd_numbers_dy), Point::from(5.135345458984375, 3.6462039947509766+odd_numbers_dy))),
AllIntersections::LineSegment(LineSegment::from(Point::from(5.135345458984375, 3.6462039947509766+odd_numbers_dy), Point::from(4.825495719909668, 4.125+odd_numbers_dy))),
AllIntersections::LineSegment(LineSegment::from(Point::from(4.825495719909668, 4.125+odd_numbers_dy), Point::from(4.954771041870117, 4.125+odd_numbers_dy))),
AllIntersections::LineSegment(LineSegment::from(Point::from(4.954771041870117, 4.125+odd_numbers_dy), Point::from(5.204428672790527, 3.726231098175049+odd_numbers_dy))),
AllIntersections::LineSegment(LineSegment::from(Point::from(5.204428672790527, 3.726231098175049+odd_numbers_dy), Point::from(5.4554548263549805, 4.125+odd_numbers_dy))),
AllIntersections::LineSegment(LineSegment::from(Point::from(5.4554548263549805, 4.125+odd_numbers_dy), Point::from(5.578573703765869, 4.125+odd_numbers_dy))),
AllIntersections::LineSegment(LineSegment::from(Point::from(5.578573703765869, 4.125+odd_numbers_dy), Point::from(5.269408226013184, 3.6482558250427246+odd_numbers_dy))),
AllIntersections::LineSegment(LineSegment::from(Point::from(5.269408226013184, 3.6482558250427246+odd_numbers_dy), Point::from(5.602513313293457, 3.125+odd_numbers_dy))),
AllIntersections::LineSegment(LineSegment::from(Point::from(5.602513313293457, 3.125+odd_numbers_dy), Point::from(5.602513313293457, 3.125+odd_numbers_dy))),
],
vec![
AllIntersections::LineSegment(LineSegment::from(Point::from(6.051214218139648, 3.125), Point::from(5.934250831604004, 3.125))),
AllIntersections::LineSegment(LineSegment::from(Point::from(5.934250831604004, 3.125), Point::from(5.934250831604004, 4.022400856018066))),
AllIntersections::LineSegment(LineSegment::from(Point::from(5.934250831604004, 4.022400856018066), Point::from(5.6196136474609375, 4.022400856018066))),
AllIntersections::LineSegment(LineSegment::from(Point::from(5.6196136474609375, 4.022400856018066), Point::from(5.6196136474609375, 4.125))),
AllIntersections::LineSegment(LineSegment::from(Point::from(5.6196136474609375, 4.125), Point::from(6.364482879638672, 4.125))),
AllIntersections::LineSegment(LineSegment::from(Point::from(6.364482879638672, 4.125), Point::from(6.364482879638672, 4.022400856018066))),
AllIntersections::LineSegment(LineSegment::from(Point::from(6.364482879638672, 4.022400856018066), Point::from(6.051214218139648, 4.022400856018066))),
AllIntersections::LineSegment(LineSegment::from(Point::from(6.051214218139648, 4.022400856018066), Point::from(6.051214218139648, 3.125))),
AllIntersections::LineSegment(LineSegment::from(Point::from(6.051214218139648, 3.125), Point::from(6.051214218139648, 3.125))),
],
];



    let shapes : Vec<sign::Shape<AllIntersections>> = 
        shapes.iter().map(
            |x| sign::Shape::from(
                cnc_router::ToolType::Text,
                x.clone()
            )
        ).collect();

    let shapes : Vec<sign::Sign<AllIntersections>> = vec![
        sign::Sign::from(
            lines_and_curves::Rectangle::from(
                lines_and_curves::Point::from(1.0, 1.0),
                lines_and_curves::Point::from(12.0, 12.0)
            ),
            shapes,
        )
    ];

    // gc.build_gcode(
    //     false,
    //     bit_path::Path::spiral_in_out,
    //     // bit_path::Path::path_x_then_y,
    //     shapes,
    // );

    gc.build_gcode_smart_path(
        true,
        &shapes,
    );
}
