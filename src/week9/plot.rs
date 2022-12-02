use std::{collections::HashMap, ops::Range};

use crate::{week5::scalar::Scalar, week8::expression::Expression};

trait LevelCurves {
    // fn level_curves(&self, z_values: &T) where T: Iterator<f64> {

    // }
}

impl Expression {
    pub fn with(&self, values: &HashMap<char, Scalar>) -> Expression {
        match self {
            Expression::Add(lhs, rhs) => lhs.with(values) + rhs.with(values),
            Expression::Subtract(lhs, rhs) => lhs.with(values) - rhs.with(values),
            Expression::Multiply(lhs, rhs) => lhs.with(values) * rhs.with(values),
            Expression::Divide(lhs, rhs) => lhs.with(values) / rhs.with(values),
            Expression::Exponentiate(lhs, rhs) => lhs.with(values) ^ rhs.with(values),
            Expression::Logarithm(lhs, rhs) => {
                Expression::Logarithm(Box::new(lhs.with(values)), Box::new(rhs.with(values)))
            }
            Expression::Constant(v) => Expression::Constant(v.clone()),

            Expression::Variable(name) => {
                if let Some(value) = values.get(name) {
                    Expression::Constant(value.clone())
                } else {
                    Expression::Variable(*name)
                }
            }

            Expression::DerivableFunctionExpression(_, _) => todo!(),
        }
        .simplified()
    }

    pub fn draw_2d(&self, variable: char, domain: Range<f64>, grain: f64, save_path: &str) {
        use plotters::prelude::*;

        let (w, h) = (640, 480);
        let margin = 10;

        let series: Vec<(f64, f64)> =
            { ((domain.start * grain) as i64)..((domain.end * grain) as i64) }
                .map(|x| x as f64 / grain)
                .map(|x| {
                    (x, {
                        let mut values = HashMap::new();
                        values.insert(variable, Scalar(x));

                        if let Expression::Constant(Scalar(v)) = self.with(&values) {
                            v
                        } else {
                            panic!()
                        }
                    })
                })
                .collect();

        let range = {
            let mut sorted = series.clone();
            sorted.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
            sorted.first().unwrap().1.clone()..sorted.last().unwrap().1.clone()
        };

        let root_drawing_area = BitMapBackend::new(save_path, (w, h)).into_drawing_area();

        root_drawing_area.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root_drawing_area)
            .margin(margin)
            .set_label_area_size(
                LabelAreaPosition::Left,
                map_from_range_to_range(&domain, 0., &(0.0..-((w - 2 * margin) as f64))) as i32,
            )
            .set_label_area_size(
                LabelAreaPosition::Top,
                map_from_range_to_range(&range, 0., &(-((h - 2 * margin) as f64)..0.0)) as i32,
            )
            .build_cartesian_2d(domain, range)
            .unwrap();

        chart
            .configure_mesh()
            .set_tick_mark_size(LabelAreaPosition::Left, 5)
            .set_tick_mark_size(LabelAreaPosition::Top, 5)
            .draw()
            .unwrap();
        chart.draw_series(LineSeries::new(series, &RED)).unwrap();
    }
}

pub fn map_from_range_to_range(
    source: &Range<f64>,
    value_in_source: f64,
    dest: &Range<f64>,
) -> f64 {
    dest.start
        + (((value_in_source - source.start) / (source.end - source.start))
            * (dest.end - dest.start))
}

#[cfg(test)]
mod test {
    use crate::{week5::scalar::Scalar, week8::expression::Expression};

    #[test]
    fn linear_plot_auto() {
        let exp = (Expression::Constant(Scalar(0.75)) * Expression::Variable('x'))
            + Expression::Constant(Scalar(1.5));

        exp.draw_2d(
            'x',
            -4.0..3.0,
            100.,
            "/home/oralekin/Code/rust/vectors/plots/test/linear.png",
        );
    }

    #[test]
    fn parabola_graph() {
        let exp = Expression::Variable('x') ^ Expression::Constant(Scalar(2.));

        exp.draw_2d(
            'x',
            -5.0..5.0,
            100.,
            "/home/oralekin/Code/rust/vectors/plots/test/parabola.png",
        );
    }
}
