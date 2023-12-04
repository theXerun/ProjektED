// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use core::f64;
use std::collections::HashSet;
use serde::ser::{Serialize, Serializer, SerializeStruct};

use tauri_plugin_log::LogTarget;

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_log::Builder::default().targets([
        LogTarget::LogDir,
        LogTarget::Stdout,
        LogTarget::Webview,
    ]).build())
    .invoke_handler(tauri::generate_handler![parse_csv, parse_csv_headers, parse_csv_reg, mae, mape, error_matrix, roc_data, mse, calculate_auc])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn parse_csv(unparsed: String) -> Vec<Vec<String>> {
  let mut reader = csv::ReaderBuilder::new().from_reader(unparsed.as_bytes());
  let records: Vec<Vec<String>> = reader
        .records()
        .map(|record| {
            record
                .expect("Error reading CSV record")
                .iter()
                .map(|field| field.to_string())
                .collect()
        })
        .collect();
  
  return records;
}

#[tauri::command]
async fn parse_csv_reg(unparsed: String) -> Vec<Vec<f64>> {
  let mut reader = csv::ReaderBuilder::new().from_reader(unparsed.as_bytes());
  let records: Vec<Vec<f64>> = reader
        .records()
        .map(|record| {
            record
                .expect("Error reading CSV record")
                .iter()
                .map(|field| {
                  field
                      .parse::<f64>()
                      .expect("Error parsing CSV field to f64")
                })
                .collect()
        })
        .collect();
      return records;
}

#[tauri::command]
async fn error_matrix(unparsed: String) -> Vec<Vec<i32>> {
  let data = parse_csv(unparsed).await;
  let mut TN1 = 0;
  let mut TP1 = 0;
  let mut FN1 = 0;
  let mut FP1 = 0;

  let mut TN2 = 0;
  let mut TP2 = 0;
  let mut FN2 = 0;
  let mut FP2 = 0;
  // zakładam że positive[0] to pozytywna wartość, a [1] to negatywna
  let values: Vec<String> = data.clone().into_iter().map(|r| r[0].to_string()).collect::<HashSet<String>>().into_iter().collect();

  for row in data {
    if row[1] == values[0] && row[1] == row[0] {
      TP1 += 1;
    } else if row[1] == values[1] && row[1] == row[0] {
      TN1 += 1;
    } else if row[1] == values[0] && row[1] != row[0] {
      FP1 += 1;
    } else if row[1] == values[1] && row[1] != row[0] {
      FN1 += 1;
    }

    if row[3] == values[0] && row[3] == row[0] {
      TP2 += 1;
    } else if row[3] == values[1] && row[3] == row[0] {
      TN2 += 1;
    } else if row[3] == values[0] && row[3] != row[0] {
      FP2 += 1;
    } else if row[3] == values[1] && row[3] != row[0] {
      FN2 += 1;
    }
  }
  return vec![vec![TN1,FP1,FN1,TP1], vec![TN2,FP2,FN2,TP2]];
}

async fn error_matrix_threshold(unparsed: &String, threshold: f64) -> Vec<Vec<i32>> {
  let data = parse_csv(unparsed.to_string()).await;
  let mut TN1 = 0;
  let mut TP1 = 0;
  let mut FN1 = 0;
  let mut FP1 = 0;

  let mut TN2 = 0;
  let mut TP2 = 0;
  let mut FN2 = 0;
  let mut FP2 = 0;
  for row in data {
    let model1prob = row[2].parse::<f64>().expect("err parsing probability");
    let model2prob = row[4].parse::<f64>().expect("err parsing probability");
    if model1prob >= threshold && row[1] == row[0] {
      TP1 += 1;
    } else if model1prob <= threshold  && row[1] == row[0] {
      TN1 += 1;
    } else if model1prob >= threshold  && row[1] != row[0] {
      FP1 += 1;
    } else if model1prob <= threshold  && row[1] != row[0] {
      FN1 += 1;
    }

    if model2prob >= threshold && row[3] == row[0] {
      TP2 += 1;
    } else if model2prob <= threshold && row[3] == row[0] {
      TN2 += 1;
    } else if model2prob >= threshold && row[3] != row[0] {
      FP2 += 1;
    } else if model2prob <= threshold && row[3] != row[0] {
      FN2 += 1;
    }
  }
  return vec![vec![TN1,TP1,FN1,FP1], vec![TN2,TP2,FN2,FP2]];
}

#[derive(Clone)]
struct ROCPoint {
  x: f64,
  y: f64
}

impl Serialize for ROCPoint {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
      S: Serializer,
  {
      let mut state = serializer.serialize_struct("ROCPoint", 2)?;
      state.serialize_field("x", &self.x)?;
      state.serialize_field("y", &self.y)?;
      state.end()
  }
}

#[tauri::command]
async fn roc_data(unparsed: String) -> Vec<Vec<ROCPoint>> {
  let mut result1: Vec<ROCPoint> = Vec::new();
  let mut result2: Vec<ROCPoint> = Vec::new();
  for n in 0..100 {
    let error_matrix = error_matrix_threshold(&unparsed, n as f64 / 100.0).await;
    let TPR1: f64 = error_matrix[0][1] as f64 / (error_matrix[0][1] as f64 + error_matrix[0][2] as f64);
    let FPR1: f64 = error_matrix[0][3] as f64 / (error_matrix[0][3] as f64 + error_matrix[0][0] as f64);
    let TPR2: f64 = error_matrix[1][1] as f64 / (error_matrix[1][1] as f64 + error_matrix[1][2] as f64);
    let FPR2: f64 = error_matrix[1][3] as f64 / (error_matrix[1][3] as f64 + error_matrix[1][0] as f64);
    let point1 = ROCPoint {
      x: FPR1,
      y: TPR1
    };
    let point2 = ROCPoint {
      x: FPR2,
      y: TPR2
    };
    result1.push(point1);
    result2.push(point2);
  }
  

  return vec![result1, result2];
}

#[tauri::command]
async fn calculate_auc(unparsed: String) -> Vec<f64> {
  let roc_points: Vec<Vec<ROCPoint>> = roc_data(unparsed).await;
  let mut sorted_points1 = roc_points[0].clone();
  let mut sorted_points2 = roc_points[1].clone();
  sorted_points1.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
  sorted_points2.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

  let mut auc1 = 0.0;
  let mut auc2 = 0.0;
  for i in 1..sorted_points1.len() {
      let x11 = sorted_points1[i - 1].x;
      let x12 = sorted_points1[i].x;
      let y11 = sorted_points1[i - 1].y;
      let y12 = sorted_points1[i].y;
      let x21 = sorted_points2[i - 1].x;
      let x22 = sorted_points2[i].x;
      let y21 = sorted_points2[i - 1].y;
      let y22 = sorted_points2[i].y;

      auc1 += 0.5 * (x12 - x11) * (y11 + y12);
      auc2 += 0.5 * (x22 - x21) * (y21 + y22);
  }
  return vec![auc1, auc2];
}

#[tauri::command]
async fn parse_csv_headers(unparsed: String) -> Vec<String> {
  let mut reader = csv::ReaderBuilder::new().from_reader(unparsed.as_bytes());
  return reader.headers().expect("Niepowodzenie czytania nagłówków").into_iter().map(|s| s.to_string()).collect();
}

#[tauri::command]
async fn mae(unparsed: String) -> Vec<f64> {
  let records = parse_csv_reg(unparsed).await;
  let mut sum1: f64 = 0.0;
  let mut sum2: f64 = 0.0;
  let n = records.len() as f64;
  for row in records {
      sum1 += (row[0] - row[1]).abs();
      sum2 += (row[0] - row[2]).abs();
  }
  return vec![sum1/n, sum2/n]
}

#[tauri::command]
async fn mape(unparsed: String) -> Vec<f64> {
  let records = parse_csv_reg(unparsed).await;
  let mut sum1: f64 = 0.0;
  let mut sum2: f64 = 0.0;
  let n = records.len() as f64;
  for row in records {
      sum1 += (row[0] - row[1]).abs()/row[1];
      sum2 += (row[0] - row[2]).abs()/row[1];
  }
  return vec![sum1*100.0/n, sum2*100.0/n]
}

#[tauri::command]
async fn mse(unparsed: String) -> Vec<f64> {
  let records = parse_csv_reg(unparsed).await;
  let n = records.len() as f64;
  let mut sum1: f64 = 0.0;
  let mut sum2: f64 = 0.0;
  for row in records {
    sum1 += (row[0] - row[1])*(row[0] - row[1]);
    sum2 += (row[0] - row[2])*(row[0] - row[2]);
  }
  return vec![sum1/n, sum2/n];
}
