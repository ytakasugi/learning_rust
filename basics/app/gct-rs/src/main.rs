use plotters::prelude::*;
use chrono::*;

const Y_AXIS_MAX: i64 = 400000000;
const Y_AXIS_MIN: i64 = 0;
const OUTPUT_FILE_NAME: &str = "plot.png";
const GRAPH_NAME: &str = "SAMPLE";

fn main() -> Result<(), Box<dyn std::error::Error>> {

    /* (1) プロット用データの準備 */

    // データを取得。この時点では(日付,値)のタプルのVector型になっている
    let data = gct_rs::get_data().unwrap();

    /* x軸とy軸で個別のVector型にする */

    // x軸 : 日付のVector
    let x: Vec<DateTime<Local>> = data.iter()
                                    .map(|(_, _, x, _, _ , _)| gct_rs::parse_time(x))
                                    .collect();

    // y軸: 値のVector
    let y_total: Vec<i64> = data.iter()
                            .map(|(_, _, _, y, _, _)| *y)
                            .collect();

    // y軸: 値のVector
    let y_used: Vec<i64> = data.iter()
                            .map(|(_, _, _, _, y, _)| *y)
                            .collect();

  /* (2) 描画先の情報を設定 */

  // 描画先を指定。画像出力する場合はBitMapBackend
    let root = BitMapBackend::new(OUTPUT_FILE_NAME, (1080, 720)).into_drawing_area();
    // 背景を白にする
    root.fill(&WHITE)?;

    /* (3) グラフ全般の設定 */

    // グラフのフォント、サイズ
    let font = ("sans-serif", 20);

    let mut chart = ChartBuilder::on(&root)
        // キャプションのフォントやサイズ
        .caption(GRAPH_NAME, font.into_font())
        // 上下左右全ての余白
        .margin(10)
        // x軸ラベル部分の余白
        .x_label_area_size(40)
        // y軸ラベル部分の余白
        .y_label_area_size(90)
        // x軸とy軸の数値の範囲を指定する
        .build_cartesian_2d(
        // x軸の範囲
            *x.first().unwrap()..*x.last().unwrap(),
        // y軸の範囲
            Y_AXIS_MIN..Y_AXIS_MAX
        )?
        .set_secondary_coord(
            *x.first().unwrap()..*x.last().unwrap(),
            Y_AXIS_MIN..Y_AXIS_MAX
        );

    /* (4) グラフの描画 */

    // 折れ線グラフの定義＆描画
    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .x_desc("TIME")
        .y_desc("USED")
        .draw()?;

    // 折れ線グラフの定義＆描画
    let line_series_used = LineSeries::new(
                x.iter()
                .zip(y_used.iter())
                .map(|(x, y)| (*x, *y)),
                &RED
    );

    chart.draw_series(line_series_used)?;

    // 折れ線グラフの定義＆描画
    let line_series_total = LineSeries::new(
                x.iter()
                .zip(y_total.iter())
                .map(|(x, y)| (*x, *y)),
                &BLUE
    );
    chart.draw_secondary_series(line_series_total)?;

    Ok(())
}
