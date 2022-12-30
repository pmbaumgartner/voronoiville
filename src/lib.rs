use pyo3::{exceptions::PyRuntimeError, prelude::*};
use voronoice::*;

#[derive(Clone)]
#[pyclass(name = "BoundingBox")]
struct BoundingBoxPy {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

#[pymethods]
impl BoundingBoxPy {
    #[new]
    fn init(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        BoundingBoxPy { x1, y1, x2, y2 }
    }
    fn __repr__(&self) -> String {
        format!(
            "BoundingBox({}, {}, {}, {})",
            self.x1, self.y1, self.x2, self.y2
        )
    }
    fn __str__(&self) -> String {
        self.__repr__()
    }
}

impl From<BoundingBoxPy> for BoundingBox {
    fn from(value: BoundingBoxPy) -> Self {
        let width = value.x2 - value.x1;
        let height = value.y2 - value.y1;
        let center = Point {
            x: (value.x1 + value.x2) / 2.0,
            y: (value.y1 + value.y2) / 2.0,
        };
        BoundingBox::new(center, width, height)
    }
}

#[allow(dead_code)]
#[pyclass(name = "VoronoiCell")]
struct VoronoiCellPy {
    #[pyo3(get)]
    position: (f64, f64),
    #[pyo3(get)]
    site: usize,
    // These are originally Iterator<Item = &Point>, but we'll collect + convert to tuples
    #[pyo3(get)]
    vertices: Vec<(f64, f64)>,
    #[pyo3(get)]
    neighbors: Option<Vec<usize>>,
    #[pyo3(get)]
    is_on_hull: bool,
}

impl From<VoronoiCell<'_>> for VoronoiCellPy {
    fn from(cell: VoronoiCell) -> Self {
        let position: (f64, f64) = {
            let pos = cell.site_position();
            (pos.x, pos.y)
        };
        let site: usize = cell.site();
        let vertices: Vec<(f64, f64)> = cell
            .iter_vertices()
            .map(|point| (point.x, point.y))
            .collect();
        let is_on_hull = cell.is_on_hull();
        let neighbors: Option<Vec<usize>> = Some(cell.iter_neighbors().collect());
        VoronoiCellPy {
            position,
            site,
            vertices,
            neighbors,
            is_on_hull,
        }
    }
}

impl VoronoiCellPy {
    fn into_no_neighbors(cell: VoronoiCell) -> Self {
        let position: (f64, f64) = {
            let pos = cell.site_position();
            (pos.x, pos.y)
        };
        let site: usize = cell.site();
        let vertices: Vec<(f64, f64)> = cell
            .iter_vertices()
            .map(|point| (point.x, point.y))
            .collect();
        let is_on_hull = cell.is_on_hull();
        VoronoiCellPy {
            position,
            site,
            vertices,
            neighbors: None,
            is_on_hull,
        }
    }
}

fn capitalize_bool(value: bool) -> String {
    match value {
        true => "True".to_string(),
        false => "False".to_string(),
    }
}

#[pymethods]
impl VoronoiCellPy {
    fn __repr__(&self) -> String {
        format!(
            "VoronoiCell(site={}, pos=({:.3}, {:.3}), on_hull={})",
            self.site,
            self.position.0,
            self.position.1,
            capitalize_bool(self.is_on_hull)
        )
    }
    fn __str__(&self) -> String {
        self.__repr__()
    }
}

#[allow(clippy::or_fun_call)]
#[pyfunction(return_neighbors = true, lloyd_relaxation_iterations = 0)]
fn voronoi(
    points: Vec<(f64, f64)>,
    bounding_box: BoundingBoxPy,
    return_neighbors: bool,
    lloyd_relaxation_iterations: usize,
) -> PyResult<Vec<VoronoiCellPy>> {
    let sites: Vec<Point> = points.iter().map(|(x, y)| Point { x: *x, y: *y }).collect();
    let bounding_box: BoundingBox = bounding_box.into();
    let v: Voronoi = VoronoiBuilder::default()
        .set_sites(sites)
        .set_bounding_box(bounding_box)
        .set_lloyd_relaxation_iterations(lloyd_relaxation_iterations)
        .build()
        // clippy generates the or_fun_call warning here
        .ok_or(PyRuntimeError::new_err(
            "Can't build Voronoi diagram from given points.",
        ))?;
    let cells: Vec<VoronoiCellPy> = match return_neighbors {
        true => v.iter_cells().map(|cell| cell.into()).collect(),
        false => v
            .iter_cells()
            .map(VoronoiCellPy::into_no_neighbors)
            .collect(),
    };
    Ok(cells)
}

#[pymodule]
fn voronoiville(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(voronoi, m)?)?;
    m.add_class::<BoundingBoxPy>()?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use voronoice::BoundingBox;

    use crate::BoundingBoxPy;

    #[test]
    fn bbox_conversion() {
        let voronoi_bbox = BoundingBox::new_centered_square(5.0);
        let python_bbox: BoundingBox = BoundingBoxPy::init(-2.5, -2.5, 2.5, 2.5).into();
        assert_eq!(voronoi_bbox.corners(), python_bbox.corners());
    }
}
