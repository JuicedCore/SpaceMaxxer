#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rect {
    pub fn minimum_side(&self) -> &f64 {
        if self.w <= self.h { &self.w } else { &self.h }
    }
}

//this function will be used to normalize and get the area occupied by each rectangle.
//Input will be the canvas with rect(0,0, 1000,1000) or whatever length and width of your canvas is
//going to be
fn normaliza(rect: &Rect, weights: &[f64]) -> Vec<f64> {
    let mut normalized_area = Vec::with_capacity(weights.len());
    let total_area = rect.w * rect.h;
    let weights_sum: f64 = weights.iter().sum();
    for weight in weights {
        normalized_area.push((weight / weights_sum) * total_area);
    }
    normalized_area
}

//Calculates how square like a row would or could be

fn worst_ratio(areas_in_row: &[f64], current_container: &Rect) -> f64 {
    if areas_in_row.is_empty() {
        return f64::MAX;
    }

    let shortest_side = current_container.minimum_side();
    let shortest_side_squared = shortest_side.powi(2);
    let sum: f64 = areas_in_row.iter().sum();
    let sum_squared = sum.powi(2);
    // Find max and min using fold
    let max = areas_in_row
        .iter()
        .fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let min = areas_in_row.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let ratio1 = (shortest_side_squared * max) / sum_squared;
    let ratio2 = sum_squared / (shortest_side_squared * min);

    if ratio1 > ratio2 { ratio1 } else { ratio2 }
}

fn layout_row(container: &Rect, row_areas: &[f64]) -> (Vec<Rect>, Rect) {
    let mut row_of_rect = Vec::with_capacity(row_areas.len());
    let sum_row_areas: f64 = row_areas.iter().sum();
    //thicknes = total area / side it is placed on
    let side = container.minimum_side();
    let thickness = sum_row_areas / side;

    //Vertical case, items stacked top to bottom
    if container.w >= container.h {
        let mut current_y = container.y;

        for &area in row_areas {
            let item_h = area / thickness;
            row_of_rect.push(Rect {
                x: container.x,
                y: current_y,
                w: thickness,
                h: item_h,
            });
            current_y += item_h;
        }
        let remaining_container = Rect {
            x: container.x + thickness,
            y: container.y,
            w: container.w - thickness,
            h: container.h,
        };

        (row_of_rect, remaining_container)
    }
    //items stacked left to right
    else {
        let mut current_x = container.x;
        for &area in row_areas {
            let item_w = area / thickness;
            row_of_rect.push(Rect {
                x: current_x,
                y: container.y,
                w: item_w,
                h: thickness,
            });
            current_x += item_w;
        }
        let remaining_container = Rect {
            x: container.x,
            y: container.y + thickness,
            w: container.w,
            h: container.h - thickness,
        };

        (row_of_rect, remaining_container)
    }
}

pub fn squarify(mut canvas: Rect, weights: &[f64]) -> Vec<Rect> {
    let mut areas = normaliza(&canvas, weights);
    areas.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));

    let mut final_rects = Vec::new();
    let mut current_row = Vec::new();

    for area in areas {
        if current_row.is_empty() {
            current_row.push(area);
        } else {
            let current_ratio = worst_ratio(&current_row, &canvas);

            let mut next_row = current_row.clone();
            next_row.push(area);
            let next_ratio = worst_ratio(&next_row, &canvas);

            if current_ratio >= next_ratio {
                current_row.push(area);
            } else {
                //stop adding as the aspect ratio ngetss worse
                let (rects, remaining_canvas) = layout_row(&canvas, &current_row);
                final_rects.extend(rects);

                canvas = remaining_canvas;
                current_row = vec![area];
            }
        }
    }

    if !current_row.is_empty() {
        let (rects, _) = layout_row(&canvas, &current_row);
        final_rects.extend(rects);
    }
    final_rects
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_squarify() {
        let canvas = Rect {
            x: 0.0,
            y: 0.0,
            w: 100.0,
            h: 100.0,
        };
        let weights = vec![10.0, 10.0, 5.0, 5.0];

        let rects = squarify(canvas, &weights);

        for (i, r) in rects.iter().enumerate() {
            println!("Rect {}: {:?}", i, r);
        }
        // This ensures the list isn't empty
        assert!(!rects.is_empty());
    }
}

