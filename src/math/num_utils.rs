
pub struct NumUtils;

impl NumUtils {
    pub fn order_triangle_vertices_by_y<T: PartialOrd>(
        x1: &mut T,
        y1: &mut T,
        x2: &mut T,
        y2: &mut T,
        x3: &mut T,
        y3: &mut T,
    ) {
        if *y1 > *y2 {
            std::mem::swap(x1, x2);
            std::mem::swap(y1, y2);
        }

        if *y2 > *y3 {
            std::mem::swap(x2, x3);
            std::mem::swap(y2, y3);
        }

        if *y1 > *y2 {
            std::mem::swap(x1, x2);
            std::mem::swap(y1, y2);
        }
    }

    pub fn order_asc<T: Ord + Copy>(a: T, b: T) -> (T, T) {
        (T::min(a, b), T::max(a, b))
    }
}
