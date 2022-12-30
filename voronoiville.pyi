from typing import Tuple, List, Optional

class VoronoiCell:
    """A cell of a Voronoi diagram.

    Attributes:
        position (Tuple[float, float]): The point associated with this cell. Will not be equal to input points if
          any iterations of Lloyd's relaxation algorithm are applied.
        site (int): The site index.
        vertices (List[Tuple[float, float]]): The vertices of the points that make up the boundary of the cell.
        neighbors (Optional[List[int]]): The site index of neighboring cells. Only exists if `return_neighbors=True`
          when calling `voronoi`.
        is_on_hull (bool): Whether this cell touches the bounding box given.
    """

    position: Tuple[float, float]
    site: int
    vertices: List[Tuple[float, float]]
    neighbors: Optional[List[int]]
    is_on_hull: bool

class BoundingBox:
    """A bounding box with bottom-left corner (x1, y1)
    and upper-right corner (x2, y2), for use with `voronoi`.
    """

    x1: float
    y1: float
    x2: float
    y2: float
    def __init__(self, x1: float, y1: float, x2: float, y2: float) -> "BoundingBox":
        """Creates a bounding box with bottom-left corner (x1, y1)
        and upper-right corner (x2, y2)

        Args:
            x1 (float): Bottom-left x-coordinate
            y1 (float): Bottom-left y-coordinate
            x2 (float): Upper-right x-coordinate
            y2 (float): Upper-right y-coordiante
        Returns:
            BoundingBox: A BoudingBox for use with `voronoi`
        """
        ...

def voronoi(
    points: List[Tuple[float, float]],
    bounding_box: BoundingBox,
    return_neighbors: bool = True,
    lloyd_relaxation_iterations: int = 0,
) -> List[VoronoiCell]:
    """Generates a voronoi diagram with the given points.

    Args:
        points (List[Tuple[float, float]]): The list of 2d (x, y) input points.
        bounding_box (BoundingBox): A bounding box for the diagram
        return_neighbors (bool, optional): Whether to return the neighboring cell indices (cell.site) for each attribute. Defaults to True.
        lloyd_relaxation_iterations (int, optional): The number of iterations to perform Lloyd's relaxation algorithm. Defaults to 0.

    Returns:
        List[VoronoiCell]: All cells contained within the Voronoi diagram with associated data.
    """
    ...
