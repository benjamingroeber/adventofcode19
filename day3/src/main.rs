// TODO read from STDIN
// No time today, there's some bug in the intersection code
static INPUT: & str = "R991,U847,L239,U883,L224,D359,L907,D944,L79,U265,L107,D183,R850,U203,R828,D95,L258,D931,R792,U117,L309,U182,L633,D567,L828,D454,L660,U652,L887,D341,L497,D857,L299,U191,L882,D476,L968,U913,R453,D776,R169,D1,L193,D187,L564,U306,R815,U9,L434,U879,L816,D142,R16,U663,L54,D347,L557,U828,R597,D328,L636,U200,L383,D256,R162,U159,R37,D748,R440,D260,R48,D755,R762,U73,L357,U132,L745,D426,L797,U744,R945,D788,R585,U948,L20,D983,L335,U709,R488,U715,R229,D672,L13,D930,R903,D71,R620,U146,L835,U936,R542,D311,R375,U91,R362,U613,L78,D451,R220,D493,R404,D516,L550,U647,L908,U254,R827,D180,R902,U972,R56,U761,R912,U356,L921,D461,L65,D651,L230,U534,R143,D614,L526,D100,R76,D135,L572,U971,L219,D793,R638,U676,L58,D882,R299,D922,L198,D872,R736,D433,L999,U157,R795,U344,R213,D205,L928,D319,L775,U288,L903,U735,R128,D835,R496,U992,L875,D823,L833,D635,L700,U586,L587,U753,R849,U433,R473,U369,R891,U10,L152,U26,L893,U752,L258,D384,L491,U314,R722,U783,R801,U551,R141,U870,L662,D572,R671,U285,L435,D83,L260,U371,R849,U741,R661,U774,L583,U947,L460,U677,R809,D130,L288,D58,R107,U597,R21,U17,R99,U202,L324,U493,R824,U207,L460,D734,L154,D689,L366,D879,L353,U548,L307,D691,R70,U470,R649,D948,L346,U16,L257,D800,R954,D165,R376,D312,R491,D175,R426,U920,L532,U2,L556,D553,R320,D861,L129,D42,R112,U101,R455,D930,R122,D443,R28,D72,L670,U133,L599,D813,R169,D827,R235,D644,L297,U261,R405,D887,R218,D647,R108,D928,L779,D961,L110,U690,L214,U342,R449,D737,L651,U940,L370,D882,R10,D605,R369,U408,R167,D542,L819
L994,U274,R468,D607,R236,D712,R825,D228,L812,U796,R806,D874,L742,D297,L269,D853,R229,U319,R616,U77,L30,D879,L831,U241,R751,D20,R577,D949,L333,D520,L249,D165,R831,U965,L229,D412,L312,U31,L624,U593,L508,D359,R187,D682,R536,D266,L761,U412,R136,D296,L334,D180,R683,U93,L323,D864,L912,U262,L150,U437,L961,U224,R684,D62,R733,U302,R700,D417,R861,U394,L647,D564,R588,U184,L344,D812,L412,U409,R853,D548,L401,D670,R973,U490,R791,D784,R569,U852,R753,U510,R394,D517,R253,D418,R665,D742,L233,D311,L266,D395,L23,U595,R248,D243,L944,U830,L846,U44,L231,D399,R131,D825,R975,U476,L306,U716,L764,D730,L455,U27,L764,D274,R403,D376,L474,D724,R237,U870,R206,U172,R857,D993,R348,U591,R228,U534,L968,U722,L891,U656,L645,U831,L838,D641,R886,U185,R760,U531,R397,D849,L790,U839,L937,U508,L802,U166,L571,D153,L600,U356,R273,D185,L862,D159,L806,U503,R612,U324,R745,D398,L905,D31,L14,U965,R586,U808,L334,U390,R44,D132,R605,U999,R880,U579,R732,D717,L489,D577,R373,D913,R238,U532,R614,U518,R197,U129,R627,U5,R774,D922,L761,D540,R418,U419,R120,U637,R237,D73,L648,D162,L324,D911,L916,D886,L60,D961,R207,U102,R872,D884,R611,U360,R679,U974,R30,U895,L327,U256,L520,U977,R792,D356,R376,D39,L689,U159,R270,D621,L197,U138,L811,U100,L776,U936,R514,D69,R625,U99,L970,D519,R831,U227,L307,D271,R940,U690,L978,D257,R500,D971,R149,U291,L706,U177,L694,U230,R780,U604,R987,U222,L941,D511,R591,U156,L511,U207,L423,U324,R508,U338,L257,U547,R952,U927,L205,U476,L713,D170,L462,D848,R666,D836,R352,U414,L653,D657,R721,U807,L182,U823,L826";

fn main() {
    let mut lines = INPUT.lines();
    let first_line = lines.next().unwrap();
    let first_directions: Vec<_> = first_line.split(",").map(direction_from_str).collect();
    let first_lines = lines_from_directions(&first_directions);

    let second_line = lines.next().unwrap();
    let second_directions: Vec<_> = first_line.split(",").map(direction_from_str).collect();
    let second_lines = lines_from_directions(&second_directions);

    let intersections = compute_intersections(&first_lines, &second_lines);
    println!("{:?}", intersections);
    let mut closest = intersections[0];
    for intersection in intersections {
        if intersection.distance() < closest.distance() {
            closest = intersection;
        }
    }
    println!("Closest: {:?}({})", closest, closest.distance())
}

type Unit = i64;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point{
    x: Unit,
    y: Unit
}

impl Point {
    fn distance(&self) -> Unit {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Line {
    Horizontal{ left: Point, right: Point},
    Vertical{bottom: Point, top: Point}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction{
    Right(Unit),
    Up(Unit),
    Left(Unit),
    Down(Unit)
}

fn direction_from_str(s: &str) -> Direction {
    let (direction, steps) = s.split_at(1);
    let steps = steps.parse().expect("Oops, that's not a number");
    match direction {
        "R" => Direction::Right(steps),
        "U" => Direction::Up(steps),
        "L" => Direction::Left(steps),
        "D" => Direction::Down(steps),
        _ => panic!("Oops, that's not a valid direction: {}", direction)
    }
}

fn lines_from_directions(directions: &[Direction]) -> Vec<Line> {
    let mut lines = Vec::new();
    let mut origin = Point{ x: 0, y: 0 };
    for &direction in directions {
        let (line, new_origin) = match direction {
            Direction::Right(offset) => {
                let destination = Point{x: origin.x + offset, y: origin.y};
                (Line::Horizontal{left: origin, right: destination}, destination)
            },
            Direction::Left(offset) => {
                let destination = Point{x: origin.x - offset, y: origin.y};
                (Line::Horizontal{left: destination, right: origin}, destination)
            },
            Direction::Up(offset) => {
                let destination = Point{x: origin.x, y: origin.y + offset};
                (Line::Vertical{bottom: origin, top: destination}, destination)
            },
            Direction::Down(offset) => {
                let destination = Point{x: origin.x, y: origin.y - offset};
                (Line::Vertical{bottom: destination, top: origin}, destination)
            },
        };
        lines.push(line);
        origin =new_origin;
    }

    lines
}

fn compute_intersections(first: &[Line], second: &[Line]) -> Vec<Point> {
    let mut intersections = Vec::new();

    for fline in first {
        for sline in second {
            if let Some(point) = fline.intersect(sline) {
                intersections.push(point);
            }
        }
    }
    intersections
}

impl Line {
    fn intersect(&self, other: &Line) -> Option<Point> {
        match self {
            Line::Horizontal { left, right } => {
                match other {
                    Line::Horizontal { .. } => None,
                    Line::Vertical { bottom, top } => straight_line_intersection(left, top, right, bottom),
                }
            },
//                +
//                |
//            +---X---+
//                |
//                +


            Line::Vertical { bottom, top } => {
                match other {
                    Line::Horizontal { left, right } => straight_line_intersection(left, top, right, bottom),
                    Line::Vertical { .. } => None,
                }
            },
        }
    }
}

fn straight_line_intersection(left: &Point, top: &Point, right: &Point, bottom: &Point) -> Option<Point>{
    if left.y > bottom.y && bottom.x < left.x && right.y < top.y && top.x < right.x  {
        Some(Point{ x: bottom.x, y: left.y })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr::eq;

    #[test]
    fn test_parse_directions() {
        let up = "U1";
        let left = "L2";
        let right = "R3";
        let down = "D4";

        let d_up = direction_from_str(up);
        let d_left = direction_from_str(left);
        let d_right = direction_from_str(right);
        let d_down = direction_from_str(down);

        assert_eq!(Direction::Up(1), d_up);
        assert_eq!(Direction::Left(2), d_left);
        assert_eq!(Direction::Right(3), d_right);
        assert_eq!(Direction::Down(4), d_down);
    }

    #[test]
    fn test_parse_lines() {
        let directions = vec![Direction::Right(4), Direction::Up(3), Direction::Left(2), Direction::Down(1)];
        let lines = lines_from_directions(&directions);

        let expected = vec![
            Line::Horizontal {left:Point{ x: 0, y: 0 }, right: Point { x: 4, y: 0 } },
            Line::Vertical { bottom: Point { x: 4, y: 0 }, top: Point { x: 4, y: 3 } },
            Line::Horizontal {left:Point{ x: 2, y: 3 }, right: Point { x: 4, y: 3 } },
            Line::Vertical { bottom: Point { x: 2, y: 2 }, top: Point { x: 2, y: 3 } },
        ];
        assert_eq!(&expected, &lines)
    }
}