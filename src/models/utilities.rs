use rand_distr::{Normal, Distribution};
use chrono::offset::{Local, TimeZone};
use chrono::{Date, Duration, NaiveDate};

use rand::SeedableRng;
use rand_xorshift::XorShiftRng;

use plotters::prelude::*;

pub const THRESHOLD: f64 = 0.2;

/// randomly generate floats based on a mean quality ranking
pub fn random_gen_quality(quality: f64) -> f64 {

    let normal = Normal::new(quality as f64, 0.15).unwrap();
    let mut v = normal.sample(&mut rand::thread_rng());

    if v < 0.0 {
        v = 0.0;
    } else if v > 1.0 {
        v = 1.0;
    };

    v

}

fn parse_time(t: &str) -> Date<Local> {
    Local
        .datetime_from_str(&format!("{} 0:0", t), "%Y-%m-%d %H:%M")
        .unwrap()
        .date()
}

pub fn test_plot_2() -> Result<(), Box<dyn std::error::Error>> {
    // Plot results
    /*
    let p = test_plot();
    
    let p = match p {
        Ok(_plot) => println!("Plot complete"),
        Err(error) => panic!("Problem plotting: {:}", error),
    };
    */
    
    let data: Vec<_> = {
        let norm_dist = Normal::new(500.0, 100.0).unwrap();
        let mut x_rand = XorShiftRng::from_seed(*b"MyFragileSeed123");
        let x_iter = norm_dist.sample_iter(&mut x_rand);
        x_iter
            .filter(|x| *x < 1500.0)
            .take(100)
            .zip(0..)
            .map(|(x, b)| x + (b as f64).powf(1.2))
            .collect()
    };

    let root =
        BitMapBackend::new("./area-chart.png", (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 60)
        .set_label_area_size(LabelAreaPosition::Bottom, 60)
        .caption("Area Chart Demo", ("sans-serif", 40))
        .build_ranged(0..(data.len() - 1), 0.0..1500.0)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    chart.draw_series(
        AreaSeries::new(
            (0..).zip(data.iter()).map(|(x, y)| (x, *y)),
            0.0,
            &RED.mix(0.2),
        )
        .border_style(&RED),
    )?;

    Ok(())
}

pub fn test_plot() -> Result<(), Box<dyn std::error::Error>> {
    let data = get_data();
    let root = BitMapBackend::new("stock.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let (to_date, from_date) = (
        parse_time(&data[0].0) + Duration::days(1),
        parse_time(&data[29].0) - Duration::days(1),
    );

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption("MSFT Stock Price", ("sans-serif", 50.0).into_font())
        .build_ranged(from_date..to_date, 110f32..135f32)?;

    chart.configure_mesh().line_style_2(&WHITE).draw()?;

    chart.draw_series(
        data.iter()
            .map(|x| CandleStick::new(parse_time(x.0), x.1, x.2, x.3, x.4, &GREEN, &RED, 15)),
    )?;

    Ok(())
}

fn get_data() -> Vec<(&'static str, f32, f32, f32, f32)> {
    return vec![
        ("2019-04-25", 130.0600, 131.3700, 128.8300, 129.1500),
        ("2019-04-24", 125.7900, 125.8500, 124.5200, 125.0100),
        ("2019-04-23", 124.1000, 125.5800, 123.8300, 125.4400),
        ("2019-04-22", 122.6200, 124.0000, 122.5700, 123.7600),
        ("2019-04-18", 122.1900, 123.5200, 121.3018, 123.3700),
        ("2019-04-17", 121.2400, 121.8500, 120.5400, 121.7700),
        ("2019-04-16", 121.6400, 121.6500, 120.1000, 120.7700),
        ("2019-04-15", 120.9400, 121.5800, 120.5700, 121.0500),
        ("2019-04-12", 120.6400, 120.9800, 120.3700, 120.9500),
        ("2019-04-11", 120.5400, 120.8500, 119.9200, 120.3300),
        ("2019-04-10", 119.7600, 120.3500, 119.5400, 120.1900),
        ("2019-04-09", 118.6300, 119.5400, 118.5800, 119.2800),
        ("2019-04-08", 119.8100, 120.0200, 118.6400, 119.9300),
        ("2019-04-05", 119.3900, 120.2300, 119.3700, 119.8900),
        ("2019-04-04", 120.1000, 120.2300, 118.3800, 119.3600),
        ("2019-04-03", 119.8600, 120.4300, 119.1500, 119.9700),
        ("2019-04-02", 119.0600, 119.4800, 118.5200, 119.1900),
        ("2019-04-01", 118.9500, 119.1085, 118.1000, 119.0200),
        ("2019-03-29", 118.0700, 118.3200, 116.9600, 117.9400),
        ("2019-03-28", 117.4400, 117.5800, 116.1300, 116.9300),
        ("2019-03-27", 117.8750, 118.2100, 115.5215, 116.7700),
        ("2019-03-26", 118.6200, 118.7050, 116.8500, 117.9100),
        ("2019-03-25", 116.5600, 118.0100, 116.3224, 117.6600),
        ("2019-03-22", 119.5000, 119.5900, 117.0400, 117.0500),
        ("2019-03-21", 117.1350, 120.8200, 117.0900, 120.2200),
        ("2019-03-20", 117.3900, 118.7500, 116.7100, 117.5200),
        ("2019-03-19", 118.0900, 118.4400, 116.9900, 117.6500),
        ("2019-03-18", 116.1700, 117.6100, 116.0500, 117.5700),
        ("2019-03-15", 115.3400, 117.2500, 114.5900, 115.9100),
        ("2019-03-14", 114.5400, 115.2000, 114.3300, 114.5900),
    ];
}