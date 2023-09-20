#![allow(dead_code)]
mod utils;
use utils::*;
use utils::lines_and_curves::*;
use lines_and_curves::{LineSegment, Rectangle};
use utils::sign::*;

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
                radius: 0.25 / 2.0,
                length: 0.0,
                front_angle: 0.0,
                back_angle: 0.0,
                orientation: 0.0,
                tool_type: cnc_router::ToolType::FullCutBroad,
                // smoothness: cnc_router::Smoothness::Medium,
                smoothness:        cnc_router::Smoothness::Finish,
                feed_rate_of_cut: feed_rate_of_cut,
                feed_rate_of_drill: feed_rate_of_drill,
                offset: 0.5,
                pre_cut_gcode: String::from(""),
                force_retouch_off: true,
                suggested_length: 4.0,
            },
            // cnc_router::Tool {
            //     name: String::from("Space Between Small"),
            //     index_in_machine: 2,
            //     offset_length: 0.0,
            //     radius: 0.05 / 2.0,
            //     length: 0.0,
            //     front_angle: 0.0,
            //     back_angle: 0.0,
            //     orientation: 0.0,
            //     tool_type: cnc_router::ToolType::SpaceBetweenCutBroad(0.0),
            //     // smoothness: cnc_router::Smoothness::Medium,
            //     smoothness:        cnc_router::Smoothness::Finish,
            //     feed_rate_of_cut: feed_rate_of_cut,
            //     feed_rate_of_drill: feed_rate_of_drill,
            //     offset: 0.1
            // },
            cnc_router::Tool {
                name: String::from("Broad Text Contour"),
                index_in_machine: 1,
                offset_length: 0.0,
                radius: 0.25 / 2.0,
                length: 0.0,
                front_angle: 0.0,
                back_angle: 0.0,
                orientation: 0.0,
                tool_type: cnc_router::ToolType::FullCutText,
                smoothness:        cnc_router::Smoothness::Finish,
                feed_rate_of_cut: feed_rate_of_cut,
                feed_rate_of_drill: feed_rate_of_drill,
                offset: 1.0,
                pre_cut_gcode: String::from(""),
                force_retouch_off: true,
                suggested_length: 4.0,
            },
            cnc_router::Tool {
                name: String::from("Exact Text Contour"),
                index_in_machine: 3,
                offset_length: 0.0,
                radius: 0.,
                length: 0.0,
                front_angle: 0.0,
                back_angle: 0.0,
                orientation: 0.0,
                tool_type: cnc_router::ToolType::FullCutText,
                smoothness:        cnc_router::Smoothness::Finish,
                feed_rate_of_cut: feed_rate_of_cut,
                feed_rate_of_drill: feed_rate_of_drill,
                offset: 1.0,
                pre_cut_gcode: String::from(""),
                force_retouch_off: true,
                suggested_length: 4.0,
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
        "012345 (The Square)",
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
                                lines_and_curves::Point::from(1.9284896850585938, 2.428506851196289),
                                lines_and_curves::Point::from(2.19659423828125, 2.428506851196289),
                                lines_and_curves::Point::from(2.2404022216796875, 2.428506851196289),
                                lines_and_curves::Point::from(2.284740447998047, 2.4352121353149414),
                                lines_and_curves::Point::from(2.3246994018554688, 2.453857421875),
                                lines_and_curves::Point::from(2.359638214111328, 2.4701642990112305),
                                lines_and_curves::Point::from(2.3899497985839844, 2.4930343627929688),
                                lines_and_curves::Point::from(2.41375732421875, 2.5234289169311523),
                                lines_and_curves::Point::from(2.4376564025878906, 2.5539283752441406),
                                lines_and_curves::Point::from(2.4542083740234375, 2.5886878967285156),
                                lines_and_curves::Point::from(2.464794158935547, 2.6258697509765625),
                                lines_and_curves::Point::from(2.4762916564941406, 2.666271209716797),
                                lines_and_curves::Point::from(2.4813308715820313, 2.7080507278442383),
                                lines_and_curves::Point::from(2.4813308715820313, 2.791963577270508),
                                lines_and_curves::Point::from(2.4762916564941406, 2.833743095397949),
                                lines_and_curves::Point::from(2.464794158935547, 2.8741445541381836),
                                lines_and_curves::Point::from(2.4542083740234375, 2.9113264083862305),
                                lines_and_curves::Point::from(2.4376564025878906, 2.9460859298706055),
                                lines_and_curves::Point::from(2.41375732421875, 2.97658634185791),
                                lines_and_curves::Point::from(2.3899497985839844, 3.0069799423217773),
                                lines_and_curves::Point::from(2.359638214111328, 3.0298500061035156),
                                lines_and_curves::Point::from(2.3246994018554688, 3.046156883239746),
                                lines_and_curves::Point::from(2.284740447998047, 3.0648021697998047),
                                lines_and_curves::Point::from(2.2404022216796875, 3.071507453918457),
                                lines_and_curves::Point::from(2.19659423828125, 3.071507453918457),
                                lines_and_curves::Point::from(1.9284896850585938, 3.071507453918457),
                ], true)
                        .iter()
                        .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                        .collect(),
                ),
                sign::Shape::from(
                        cnc_router::ToolType::FullCutText,
                        lines_and_curves::LineSegment::create_path(&vec![
                                lines_and_curves::Point::from(2.055908203125, 2.535794258117676),
                                lines_and_curves::Point::from(2.1545791625976563, 2.535794258117676),
                                lines_and_curves::Point::from(2.1877899169921875, 2.535794258117676),
                                lines_and_curves::Point::from(2.2247848510742188, 2.539196014404297),
                                lines_and_curves::Point::from(2.2556991577148438, 2.5521602630615234),
                                lines_and_curves::Point::from(2.2792282104492188, 2.5620269775390625),
                                lines_and_curves::Point::from(2.3002891540527344, 2.576235771179199),
                                lines_and_curves::Point::from(2.3159446716308594, 2.596613883972168),
                                lines_and_curves::Point::from(2.3312301635742188, 2.6165151596069336),
                                lines_and_curves::Point::from(2.3407669067382813, 2.639314651489258),
                                lines_and_curves::Point::from(2.345775604248047, 2.663837432861328),
                                lines_and_curves::Point::from(2.3515586853027344, 2.692140579223633),
                                lines_and_curves::Point::from(2.353912353515625, 2.721144676208496),
                                lines_and_curves::Point::from(2.353912353515625, 2.77886962890625),
                                lines_and_curves::Point::from(2.3515586853027344, 2.8078737258911133),
                                lines_and_curves::Point::from(2.345775604248047, 2.836176872253418),
                                lines_and_curves::Point::from(2.3407669067382813, 2.8606996536254883),
                                lines_and_curves::Point::from(2.3312301635742188, 2.8834991455078125),
                                lines_and_curves::Point::from(2.3159446716308594, 2.903400421142578),
                                lines_and_curves::Point::from(2.3002891540527344, 2.923778533935547),
                                lines_and_curves::Point::from(2.2792282104492188, 2.9379873275756836),
                                lines_and_curves::Point::from(2.2556991577148438, 2.9478540420532227),
                                lines_and_curves::Point::from(2.2247848510742188, 2.960818290710449),
                                lines_and_curves::Point::from(2.1877899169921875, 2.9642200469970703),
                                lines_and_curves::Point::from(2.1545791625976563, 2.9642200469970703),
                                lines_and_curves::Point::from(2.055908203125, 2.9642200469970703),
                ], true)
                        .iter()
                        .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                        .collect(),
                ),
                sign::Shape::from(
                        cnc_router::ToolType::FullCutText,
                        lines_and_curves::LineSegment::create_path(&vec![
                                lines_and_curves::Point::from(4.749195098876953, 2.831653594970703),
                                lines_and_curves::Point::from(4.749195098876953, 3.071500778198242),
                                lines_and_curves::Point::from(4.621776580810547, 3.071500778198242),
                                lines_and_curves::Point::from(4.621776580810547, 2.428500175476074),
                                lines_and_curves::Point::from(4.906513214111328, 2.428500175476074),
                                lines_and_curves::Point::from(4.943840026855469, 2.428500175476074),
                                lines_and_curves::Point::from(4.984531402587891, 2.432866096496582),
                                lines_and_curves::Point::from(5.018806457519531, 2.448575019836426),
                                lines_and_curves::Point::from(5.0447540283203125, 2.460468292236328),
                                lines_and_curves::Point::from(5.069091796875, 2.4767284393310547),
                                lines_and_curves::Point::from(5.086902618408203, 2.499330520629883),
                                lines_and_curves::Point::from(5.1024932861328125, 2.5191221237182617),
                                lines_and_curves::Point::from(5.113685607910156, 2.5413150787353516),
                                lines_and_curves::Point::from(5.119651794433594, 2.5657958984375),
                                lines_and_curves::Point::from(5.124828338623047, 2.5870437622070313),
                                lines_and_curves::Point::from(5.128223419189453, 2.6090469360351563),
                                lines_and_curves::Point::from(5.128223419189453, 2.6527366638183594),
                                lines_and_curves::Point::from(5.124855041503906, 2.6745786666870117),
                                lines_and_curves::Point::from(5.1196441650390625, 2.6956968307495117),
                                lines_and_curves::Point::from(5.1136474609375, 2.72000789642334),
                                lines_and_curves::Point::from(5.102386474609375, 2.742039680480957),
                                lines_and_curves::Point::from(5.086902618408203, 2.7616968154907227),
                                lines_and_curves::Point::from(5.069103240966797, 2.784287452697754),
                                lines_and_curves::Point::from(5.044765472412109, 2.8003902435302734),
                                lines_and_curves::Point::from(5.01873779296875, 2.8120479583740234),
                                lines_and_curves::Point::from(4.9844207763671875, 2.8274173736572266),
                                lines_and_curves::Point::from(4.943744659423828, 2.831653594970703),
                                lines_and_curves::Point::from(4.906513214111328, 2.831653594970703),
                ], true)
                        .iter()
                        .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                        .collect(),
                ),
                sign::Shape::from(
                        cnc_router::ToolType::FullCutText,
                        lines_and_curves::LineSegment::create_path(&vec![
                                lines_and_curves::Point::from(3.6575164794921875, 2.7630395889282227),
                                lines_and_curves::Point::from(3.6602249145507813, 2.764031410217285),
                                lines_and_curves::Point::from(3.6628952026367188, 2.7651100158691406),
                                lines_and_curves::Point::from(3.66552734375, 2.7662487030029297),
                                lines_and_curves::Point::from(3.6783790588378906, 2.7718029022216797),
                                lines_and_curves::Point::from(3.6893959045410156, 2.780472755432129),
                                lines_and_curves::Point::from(3.6987991333007813, 2.790760040283203),
                                lines_and_curves::Point::from(3.7100067138671875, 2.80301570892334),
                                lines_and_curves::Point::from(3.7179794311523438, 2.8180274963378906),
                                lines_and_curves::Point::from(3.7237777709960938, 2.8334999084472656),
                                lines_and_curves::Point::from(3.731342315673828, 2.853665351867676),
                                lines_and_curves::Point::from(3.7335433959960938, 2.8765316009521484),
                                lines_and_curves::Point::from(3.7335433959960938, 2.930842399597168),
                                lines_and_curves::Point::from(3.735736846923828, 2.9641342163085938),
                                lines_and_curves::Point::from(3.741710662841797, 2.996527671813965),
                                lines_and_curves::Point::from(3.7452392578125, 3.0156383514404297),
                                lines_and_curves::Point::from(3.751903533935547, 3.041666030883789),
                                lines_and_curves::Point::from(3.7653427124023438, 3.0564451217651367),
                                lines_and_curves::Point::from(3.7790298461914063, 3.071500778198242),
                                lines_and_curves::Point::from(3.6363258361816406, 3.071500778198242),
                                lines_and_curves::Point::from(3.633697509765625, 3.067180633544922),
                                lines_and_curves::Point::from(3.6246490478515625, 3.0523147583007813),
                                lines_and_curves::Point::from(3.619525909423828, 3.035991668701172),
                                lines_and_curves::Point::from(3.61798095703125, 3.018657684326172),
                                lines_and_curves::Point::from(3.6165542602539063, 3.0026779174804688),
                                lines_and_curves::Point::from(3.6157569885253906, 2.98663330078125),
                                lines_and_curves::Point::from(3.6157569885253906, 2.9465713500976563),
                                lines_and_curves::Point::from(3.614635467529297, 2.9218311309814453),
                                lines_and_curves::Point::from(3.6106300354003906, 2.898122787475586),
                                lines_and_curves::Point::from(3.6077842712402344, 2.881291389465332),
                                lines_and_curves::Point::from(3.6024818420410156, 2.864259719848633),
                                lines_and_curves::Point::from(3.592803955078125, 2.850039482116699),
                                lines_and_curves::Point::from(3.5842857360839844, 2.8375320434570313),
                                lines_and_curves::Point::from(3.572864532470703, 2.8287487030029297),
                                lines_and_curves::Point::from(3.5588226318359375, 2.8231334686279297),
                                lines_and_curves::Point::from(3.540943145751953, 2.815980911254883),
                                lines_and_curves::Point::from(3.519542694091797, 2.814146041870117),
                                lines_and_curves::Point::from(3.500457763671875, 2.814146041870117),
                                lines_and_curves::Point::from(3.348388671875, 2.814146041870117),
                                lines_and_curves::Point::from(3.348388671875, 3.071500778198242),
                                lines_and_curves::Point::from(3.2209701538085938, 3.071500778198242),
                                lines_and_curves::Point::from(3.2209701538085938, 2.428500175476074),
                                lines_and_curves::Point::from(3.5284652709960938, 2.428500175476074),
                                lines_and_curves::Point::from(3.583850860595703, 2.428500175476074),
                                lines_and_curves::Point::from(3.6459426879882813, 2.4378700256347656),
                                lines_and_curves::Point::from(3.7285614013671875, 2.5092830657958984),
                                lines_and_curves::Point::from(3.7440452575683594, 2.555267333984375),
                                lines_and_curves::Point::from(3.7440452575683594, 2.632444381713867),
                                lines_and_curves::Point::from(3.7408485412597656, 2.6601457595825195),
                                lines_and_curves::Point::from(3.7301292419433594, 2.684086799621582),
                                lines_and_curves::Point::from(3.7220497131347656, 2.702134132385254),
                                lines_and_curves::Point::from(3.7115936279296875, 2.7197399139404297),
                                lines_and_curves::Point::from(3.6977195739746094, 2.7339611053466797),
                                lines_and_curves::Point::from(3.686981201171875, 2.7449722290039063),
                                lines_and_curves::Point::from(3.674518585205078, 2.7551889419555664),
                                lines_and_curves::Point::from(3.6604766845703125, 2.7616729736328125),
                                lines_and_curves::Point::from(3.6595001220703125, 2.76212215423584),
                                lines_and_curves::Point::from(3.6585121154785156, 2.7625789642333984),
                ], true)
                        .iter()
                        .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                        .collect(),
                ),
                sign::Shape::from(
                        cnc_router::ToolType::FullCutText,
                        lines_and_curves::LineSegment::create_path(&vec![
                                lines_and_curves::Point::from(4.749195098876953, 2.535787582397461),
                                lines_and_curves::Point::from(4.902133941650391, 2.535787582397461),
                                lines_and_curves::Point::from(4.915279388427734, 2.535787582397461),
                                lines_and_curves::Point::from(4.9284515380859375, 2.5370235443115234),
                                lines_and_curves::Point::from(4.941280364990234, 2.5399370193481445),
                                lines_and_curves::Point::from(4.952537536621094, 2.5424957275390625),
                                lines_and_curves::Point::from(4.962779998779297, 2.5472240447998047),
                                lines_and_curves::Point::from(4.9718170166015625, 2.554401397705078),
                                lines_and_curves::Point::from(4.981285095214844, 2.561922073364258),
                                lines_and_curves::Point::from(4.988006591796875, 2.571878433227539),
                                lines_and_curves::Point::from(4.9927978515625, 2.5828933715820313),
                                lines_and_curves::Point::from(4.999073028564453, 2.5973310470581055),
                                lines_and_curves::Point::from(5.000804901123047, 2.6144847869873047),
                                lines_and_curves::Point::from(5.000804901123047, 2.645315170288086),
                                lines_and_curves::Point::from(4.998737335205078, 2.661712646484375),
                                lines_and_curves::Point::from(4.991722106933594, 2.6754417419433594),
                                lines_and_curves::Point::from(4.986011505126953, 2.686614990234375),
                                lines_and_curves::Point::from(4.9783477783203125, 2.69635009765625),
                                lines_and_curves::Point::from(4.968170166015625, 2.703775405883789),
                                lines_and_curves::Point::from(4.9582977294921875, 2.7109804153442383),
                                lines_and_curves::Point::from(4.947444915771484, 2.7162208557128906),
                                lines_and_curves::Point::from(4.9356536865234375, 2.7194366455078125),
                                lines_and_curves::Point::from(4.924152374267578, 2.722574234008789),
                                lines_and_curves::Point::from(4.912322998046875, 2.724370002746582),
                                lines_and_curves::Point::from(4.900382995605469, 2.724370002746582),
                                lines_and_curves::Point::from(4.749195098876953, 2.724370002746582),
                ], true)
                        .iter()
                        .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                        .collect(),
                ),
                sign::Shape::from(
                        cnc_router::ToolType::FullCutText,
                        lines_and_curves::LineSegment::create_path(&vec![
                                lines_and_curves::Point::from(3.348388671875, 2.535787582397461),
                                lines_and_curves::Point::from(3.5170860290527344, 2.535787582397461),
                                lines_and_curves::Point::from(3.530200958251953, 2.535787582397461),
                                lines_and_curves::Point::from(3.5432968139648438, 2.5368499755859375),
                                lines_and_curves::Point::from(3.5562171936035156, 2.5391502380371094),
                                lines_and_curves::Point::from(3.567554473876953, 2.541165351867676),
                                lines_and_curves::Point::from(3.5781936645507813, 2.545185089111328),
                                lines_and_curves::Point::from(3.5877113342285156, 2.5517120361328125),
                                lines_and_curves::Point::from(3.5971527099609375, 2.5581893920898438),
                                lines_and_curves::Point::from(3.603912353515625, 2.5670909881591797),
                                lines_and_curves::Point::from(3.608715057373047, 2.577413558959961),
                                lines_and_curves::Point::from(3.6149940490722656, 2.590909957885742),
                                lines_and_curves::Point::from(3.6166305541992188, 2.607484817504883),
                                lines_and_curves::Point::from(3.6166305541992188, 2.6470203399658203),
                                lines_and_curves::Point::from(3.611400604248047, 2.672945022583008),
                                lines_and_curves::Point::from(3.5728416442871094, 2.7083940505981445),
                                lines_and_curves::Point::from(3.5444679260253906, 2.7129878997802734),
                                lines_and_curves::Point::from(3.5188369750976563, 2.7129878997802734),
                                lines_and_curves::Point::from(3.348388671875, 2.7129878997802734),
                ], true)
                        .iter()
                        .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                        .collect(),
                ),
                sign::Shape::from(
                        cnc_router::ToolType::FullCutText,
                        lines_and_curves::LineSegment::create_path(&vec![
                                lines_and_curves::Point::from(1.0, 1.0), // Softy
                                lines_and_curves::Point::from(5.875, 1.0), // Softy
                                lines_and_curves::Point::from(5.875, 4.125), // Softy
                                lines_and_curves::Point::from(1.0, 4.125), // Softy
                ], true)
                        .iter()
                        .map(|x| lines_and_curves::AllIntersections::LineSegment(x.clone()))
                        .collect(),
                ),
                ],
        )
    ];

    eprintln!("{:?}", shapes);

    gc.build_gcode_smart_path(
        true,
        &shapes,
        &vec![
            (cnc_router::ToolType::FullCutText, 0.009),
            (cnc_router::ToolType::Braille, 0.0),
        ],
    );
}
