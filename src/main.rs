use matfile;
use ndarray::{Array2, s, Axis};
use ndarray_stats::{QuantileExt};
use std::time::Instant;

const INV_SBOX: [i32; 256] = [
    082,009,106,213,048,054,165,056,191,064,163,158,129,243,215,251,
    124,227,057,130,155,047,255,135,052,142,067,068,196,222,233,203,
    084,123,148,050,166,194,035,061,238,076,149,011,066,250,195,078,
    008,046,161,102,040,217,036,178,118,091,162,073,109,139,209,037,
    114,248,246,100,134,104,152,022,212,164,092,204,093,101,182,146,
    108,112,072,080,253,237,185,218,094,021,070,087,167,141,157,132,
    144,216,171,000,140,188,211,010,247,228,088,005,184,179,069,006,
    208,044,030,143,202,063,015,002,193,175,189,003,001,019,138,107,
    058,145,017,065,079,103,220,234,151,242,207,206,240,180,230,115,
    150,172,116,034,231,173,053,133,226,249,055,232,028,117,223,110,
    071,241,026,113,029,041,197,137,111,183,098,014,170,024,190,027,
    252,086,062,075,198,210,121,032,154,219,192,254,120,205,090,244,
    031,221,168,051,136,007,199,049,177,018,16,089,039,128,236,095,
    096,081,127,169,025,181,074,013,045,229,122,159,147,201,156,239,
    160,224,059,077,174,042,245,176,200,235,187,060,131,083,153,097,
    023,043,004,126,186,119,214,038,225,105,020,099,085,033,012,125,
];

const NB_TRACES: i32 = 2000;
const X_MIN: i32 = 5820; // Exclu
const X_MAX: i32 = 6620;

fn build_cto_inv(cto_data: &Vec<f64>) -> Array2<i32> {
    let mut cto_inv = Array2::<i32>::zeros((2000, 16));
    for i in 0..NB_TRACES {
        cto_inv[[i as usize, 0]] = cto_data[i as usize] as i32;
        cto_inv[[i as usize, 1]] = cto_data[(i + 2000 * 13) as usize] as i32;
        cto_inv[[i as usize, 2]] = cto_data[(i + 2000 * 10) as usize] as i32;
        cto_inv[[i as usize, 3]] = cto_data[(i + 2000 * 7) as usize] as i32;
        cto_inv[[i as usize, 4]] = cto_data[(i + 2000 * 4) as usize] as i32;
        cto_inv[[i as usize, 5]] = cto_data[(i + 2000 * 1) as usize] as i32;
        cto_inv[[i as usize, 6]] = cto_data[(i + 2000 * 14) as usize] as i32;
        cto_inv[[i as usize, 7]] = cto_data[(i + 2000 * 11) as usize] as i32;
        cto_inv[[i as usize, 8]] = cto_data[(i + 2000 * 8) as usize] as i32;
        cto_inv[[i as usize, 9]] = cto_data[(i + 2000 * 5) as usize] as i32;
        cto_inv[[i as usize, 10]]  = cto_data[(i + 2000) as usize] as i32;
        cto_inv[[i as usize, 11]]  = cto_data[(i + 2000 * 15) as usize] as i32;
        cto_inv[[i as usize, 12]]  = cto_data[(i + 2000 * 12) as usize] as i32;
        cto_inv[[i as usize, 13]]  = cto_data[(i + 2000 * 9) as usize] as i32;
        cto_inv[[i as usize, 14]]  = cto_data[(i + 2000 * 6) as usize] as i32;
        cto_inv[[i as usize, 15]]  = cto_data[(i + 2000 * 3) as usize] as i32;
    }
    return cto_inv
}

fn get_cto(cto_data: &Vec<f64>) -> Array2<i32> {
    let mut cto = Array2::<i32>::zeros((2000, 16));
    for j in 0..16 {
        for i in 0..NB_TRACES {
            cto[[i as usize, j]] = cto_data[(i + (j as i32) * 2000) as usize] as i32;
        }
    }
    return cto
}

fn get_traces(traces_data: &Vec<f64>) -> Array2<f64> {
    let mut traces = Array2::<f64>::zeros((2000, 10000));
    for j in 0..10000 {
        for i in 0..2000 {
            traces[[i as usize, j]] = traces_data[(i + 2000 * (j as i32)) as usize];
        }
    }
    return traces
}

fn read_mat_file(path: String, array_name: &str) -> Option<Vec<f64>> {
    if let Ok(file) = std::fs::File::open(path) {
        if let Ok(mat_file) = matfile::MatFile::parse(file) {
            if let Some(arr) = mat_file.find_by_name(array_name) {
                if let matfile::NumericData::Double { real: arr_data, imag: _ } = arr.data() {
                    return Some(arr_data.clone());
                }
            }
        }
    }
    return None;
}

fn pearson_correlation(x: &Array2<f64>, y: &Array2<f64>) -> Option<Array2<f64>> {
    let y_cols = y.ncols();
    let x_rows = x.nrows();
    let x_cols = x.ncols();
    if x_rows != y.nrows() {
        return None
    }
    let x_mean = x.mean_axis(Axis(0)).unwrap();
    let y_mean = y.mean_axis(Axis(0)).unwrap();

    let mut repx = Array2::<f64>::zeros((x_rows, x_cols));
    for i in 0..repx.nrows() {
        for j in 0..repx.ncols() {
            repx[[i,j]] = x_mean[j];
        }
    }

    let mut repy = Array2::<f64>::zeros((y.nrows(), y_cols));
    for i in 0..repy.nrows() {
        for j in 0..repy.ncols() {
            repy[[i,j]] = y_mean[j];
        }
    }
    // remove mean
    let x_less_mean = x - &repx;
    let y_less_mean = y - &repy;
    // (n-1)cov(x,y)
    let mut pear_corr = x_less_mean.t().dot(&y_less_mean);
    let x_square = &x_less_mean * &x_less_mean;
    let y_square = &y_less_mean * &y_less_mean;
    let sx = x_square.sum_axis(Axis(0)).mapv(f64::sqrt);
    let sxt = sx.t();
    let mut rep_sxt = Array2::<f64>::zeros((sxt.len(), y_cols));
    for i in 0..rep_sxt.nrows() {
        for j in 0..rep_sxt.ncols() {
            rep_sxt[[i,j]] = sxt[i];
        }
    }

    let sy = y_square.sum_axis(Axis(0)).mapv(f64::sqrt);
    let mut rep_sy = Array2::<f64>::zeros((x_cols, sy.len()));
    for i in 0..rep_sy.nrows() {
        for j in 0..rep_sy.ncols() {
            rep_sy[[i,j]] = sy[j];
        }
    }

    for i in 0..pear_corr.nrows() {
        for j in 0..pear_corr.ncols() {
            pear_corr[[i,j]] /= rep_sxt[[i,j]];
            pear_corr[[i,j]] /= rep_sy[[i,j]];
        }
    }

    return Some(pear_corr)
}

fn main() {
    let mut bytes: Vec<usize> = vec![];
    println!("Initialisation...");
    let initialisation_time = Instant::now();
    if let Some(cto_data) = read_mat_file("res/CTO.mat".to_string(), "CTO") {
        if let Some(traces_data) = read_mat_file("res/Traces.mat".to_string(), "Traces") {
            let cto_inv = build_cto_inv(&cto_data);
            let cto = get_cto(&cto_data);
            let traces = get_traces(&traces_data);
            let sub_traces = traces.slice(s![.., X_MIN..X_MAX]).to_owned();
            println!("Time of initialisation (in seconds) : {}", initialisation_time.elapsed().as_secs());
            println!("Decrypting the key...");
            for byte_nb in 0..16 {
                let byte_time = Instant::now();
                let mut vi: Array2<i32> = Array2::<i32>::zeros((2000, 256));
                let mut h: Array2<f64> = Array2::<f64>::zeros((2000, 256));
                for i in 0..NB_TRACES {
                    for k in 0..256 {
                        let xor1 = cto_inv[[i as usize, byte_nb as usize]] ^ k;
                        vi[[i as usize, k as usize]] = INV_SBOX[xor1 as usize];
                        let xor2 = vi[[i as usize, k as usize]] ^ cto[[i as usize, byte_nb as usize]];
                        h[[i as usize, k as usize]] = hamming::weight(&xor2.to_be_bytes()) as f64;
                    }
                }

                if let Some(correlation) = pearson_correlation(&h, &sub_traces) {
                    let min_pos = correlation.argmin().unwrap();
                    println!("found byte {} => {} in {}", byte_nb, min_pos.0,byte_time.elapsed().as_secs());
                    bytes.push(min_pos.0);
                }
            }
            println!("The decrypted key is {:?}", bytes);
        }
    }
}
