use crate::solutions::day17::map::geometry::Point;
use crate::solutions::day17::map::Map;
use crate::solutions::day17::map::Tile::Land;

impl Map {
    pub fn intersections<'a>(&'a self) -> impl Iterator<Item=Point> + 'a {
        self.all_points().filter(|&p| self.is_intersection(p))
    }

    fn is_intersection(&self, p: Point) -> bool {
        let num_adj_land = self.neighbors(p).filter(|&p2| self[p2] == Land).count();

        self[p] == Land && num_adj_land >= 3
    }
}
