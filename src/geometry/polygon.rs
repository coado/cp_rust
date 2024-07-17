use super::objects::{Point2D, Vector2D, EPS};
use std::cmp::Ordering;
use std::f64::consts::PI;

pub fn is_convex(points: Vec<Point2D>) -> bool {
    let n = points.len();

    if n <= 3 {
        return false;
    }

    let first_turn = Vector2D::ccw(&points[0], &points[1], &points[2]);

    for (i, _) in points.iter().enumerate().skip(1) {
        let j = (i + 1) % n;
        let k = (i + 2) % n;

        if Vector2D::ccw(&points[i], &points[j], &points[k]) != first_turn {
            return false;
        }
    }

    true
}

#[derive(Debug, PartialEq)]
pub enum InPolygon {
    Inside,
    OnEdge,
    Outside,
}

pub fn in_polygon(pt: &Point2D, points: Vec<Point2D>) -> InPolygon {
    let n = points.len();
    if n <= 3 {
        return InPolygon::Outside;
    }

    for i in 0..n {
        let j = (i + 1) % n;

        let a = points[i];
        let b = points[j];

        if a.distance(pt) + b.distance(pt) - a.distance(&b) < EPS {
            return InPolygon::OnEdge;
        }
    }

    let mut sum = 0.0;
    for i in 0..n {
        let j = (i + 1) % n;

        let a = points[i];
        let b = points[j];
        let vector_a = Vector2D::from_points(pt, &a);
        let vector_b = Vector2D::from_points(pt, &b);
        let angle = vector_a.angle(&vector_b);

        if Vector2D::ccw(pt, &a, &b) {
            sum += angle;
        } else {
            sum -= angle;
        }
    }

    if (sum - 2.0 * PI).abs() < EPS {
        InPolygon::Inside
    } else {
        InPolygon::Outside
    }
}

pub fn convex_hull(mut pts: Vec<Point2D>) -> Vec<Point2D> {
    pts.sort_by(|a, b| {
        if a.x < b.x {
            Ordering::Less
        } else if a.x > b.x {
            Ordering::Greater
        } else {
            a.y.partial_cmp(&b.y).unwrap()
        }
    });

    let dummy = Point2D::default();
    let mut hull: Vec<Point2D> = vec![dummy; 2 * pts.len()];
    let mut i = 0;

    for pt in pts.iter() {
        while i >= 2 && !Vector2D::ccw(&hull[i - 1], &hull[i - 2], pt) {
            i -= 1
        }
        hull[i] = *pt;
        i += 1;
    }

    let j = i + 1;
    for pt in pts.iter().rev() {
        while i >= j && !Vector2D::ccw(&hull[i - 1], &hull[i - 2], pt) {
            i -= 1
        }
        hull[i] = *pt;
        i += 1;
    }

    hull.resize(i - 1, dummy);
    hull
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_convex() {
        let points = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(1.0, 0.0),
            Point2D::new(1.0, 1.0),
            Point2D::new(0.0, 1.0),
        ];

        assert_eq!(is_convex(points), true);

        let points = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(1.0, 0.0),
            Point2D::new(0.5, 0.5),
            Point2D::new(0.0, 1.0),
        ];

        assert_eq!(is_convex(points), false);
    }

    #[test]
    fn test_in_polygon() {
        let points = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(1.0, 0.0),
            Point2D::new(1.0, 1.0),
            Point2D::new(0.0, 1.0),
        ];

        let pt = Point2D::new(0.5, 0.5);
        assert_eq!(in_polygon(&pt, points.clone()), InPolygon::Inside);

        let pt = Point2D::new(0.0, 0.0);
        assert_eq!(in_polygon(&pt, points.clone()), InPolygon::OnEdge);

        let pt = Point2D::new(0.5, 0.0);
        assert_eq!(in_polygon(&pt, points.clone()), InPolygon::OnEdge);

        let pt = Point2D::new(0.5, 1.0);
        assert_eq!(in_polygon(&pt, points.clone()), InPolygon::OnEdge);

        let pt = Point2D::new(0.0, 0.5);
        assert_eq!(in_polygon(&pt, points.clone()), InPolygon::OnEdge);

        let pt = Point2D::new(1.0, 0.5);
        assert_eq!(in_polygon(&pt, points.clone()), InPolygon::OnEdge);

        let pt = Point2D::new(1.0, 1.0);
        assert_eq!(in_polygon(&pt, points.clone()), InPolygon::OnEdge);

        let pt = Point2D::new(0.0, 1.0);
        assert_eq!(in_polygon(&pt, points.clone()), InPolygon::OnEdge);

        let pt = Point2D::new(1.5, 0.5);
        assert_eq!(in_polygon(&pt, points.clone()), InPolygon::Outside);

        let pt = Point2D::new(0.5, 1.5);
        assert_eq!(in_polygon(&pt, points.clone()), InPolygon::Outside);

        let pt = Point2D::new(-0.5, 0.5);
        assert_eq!(in_polygon(&pt, points.clone()), InPolygon::Outside);
    }

    #[test]
    fn test_convex_hull() {
        let points = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(1.0, 0.0),
            Point2D::new(1.0, 1.0),
            Point2D::new(0.0, 1.0),
            Point2D::new(0.5, 0.5),
        ];

        let mut hull = convex_hull(points);
        let mut expected = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(1.0, 0.0),
            Point2D::new(1.0, 1.0),
            Point2D::new(0.0, 1.0),
        ];

        hull.sort();
        expected.sort();

        assert_eq!(hull, expected);
    }

    #[test]
    fn test_convex_hull2() {
        let points = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(1.0, 0.0),
            Point2D::new(1.0, 1.0),
            Point2D::new(0.0, 1.0),
            Point2D::new(0.5, 0.5),
            Point2D::new(0.5, 0.0),
            Point2D::new(0.5, 1.0),
            Point2D::new(0.0, 0.5),
            Point2D::new(1.0, 0.5),
        ];

        let mut hull = convex_hull(points);
        let mut expected = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(1.0, 0.0),
            Point2D::new(1.0, 1.0),
            Point2D::new(0.0, 1.0),
        ];

        hull.sort();
        expected.sort();

        assert_eq!(hull, expected);
    }
}
