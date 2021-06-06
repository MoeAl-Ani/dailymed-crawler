mod entity;
pub mod db;

use std::collections::{VecDeque, HashSet};
use chrono::{DateTime, Utc};
use reqwest::{Response, Error};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Write, Read, ErrorKind};
use std::sync::mpsc::{channel, RecvError, Sender, Receiver};
use std::sync::{Arc, Mutex, RwLock};
use std::borrow::{Borrow, BorrowMut};
use reqwest::header::{HeaderMap, HeaderValue, HeaderName};
use tokio::io::AsyncReadExt;
use serde::de::DeserializeOwned;
use serde_json::Value;
use crate::entity::{spl_detail, product_detail};
use spl_detail::{SplsData, SplRootObject, MetaData};
use product_detail::{Document};
use tokio::runtime::Handle;
use tokio::task::JoinHandle;
use std::ops::Add;
use tokio::time::{sleep, Duration};
use rand::Rng;
use futures::future::err;
use crate::entity::drug::Drug;
use futures::{StreamExt, TryFutureExt};
use std::option::Option::Some;
use std::collections::hash_map::RandomState;
use crate::entity::product_detail::{Ingredient, Quantity, FormCode};
use crate::db::connection_pool_manager::PoolInstantiate;
use std::process::exit;
use sqlx::Connection;
use std::future::Future;


struct XmlStringWrapper(String);

enum Type {
    DailyMed,
    Csv,
}

enum Command {
    Init,
    Load,
    FixErrors,
    Transform(Type),
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let command = Command::Transform(Type::DailyMed);
    //let command = Command::Load;
    //let command = Command::Init;
    //let command = Command::FixErrors;
    match command {
        Command::Init => {
            let data = crawl_spl_data().await.unwrap();
            match serde_json::to_string(&data) {
                Ok(json) => {
                    write_to_file("spl_data.json", &json);
                    let (drugs, errors) = crawl_drug_details(&data).await; // crawl data by spl set ids
                    let drugs_json = serde_json::to_string(&drugs).unwrap();
                    let errors = errors.join(",");
                    write_to_file("drug_details.json", &drugs_json);
                    write_to_file("errors.txt", &errors);
                }
                Err(e) => {
                    eprintln!("error {}", e);
                }
            }
        }
        Command::FixErrors => {
            let errors_str: String = read_from_file_without_mapping("errors.txt");
            let data = errors_str.split(",").map(|e| {
                SplsData { setid: format!("{}", e), title: format!("") }
            }).collect::<Vec<SplsData>>();
            let mut drugs_list: Vec<Document> = read_from_file("drug_details.json");
            let (drugs, errors) = crawl_drug_details(&data).await;
            for d in drugs {
                drugs_list.push(d);
            }
            let drugs_json = serde_json::to_string(&drugs_list).unwrap();
            let errors = errors.join(",");
            write_to_file("drug_details.json", &drugs_json);
            write_to_file("errors.txt", &errors);
        }
        Command::Transform(ref from) => {
            let mut drugs_list: Vec<Document> = read_from_file("drug_details.json");
            match from {
                Type::DailyMed => {
                    let mut transform = DailyMedTransform;
                    let drugs = transform.transform(drugs_list);
                    write_drugs_to_db(drugs).await;
                }
                Type::Csv => {}
            }
        }
        Command::Load => {
            let data = load_spl_data().unwrap();
            let (drugs, errors) = crawl_drug_details(&data).await; // crawl data by spl set ids
            let drugs_json = serde_json::to_string(&drugs).unwrap();
            let errors = errors.join(",");
            write_to_file("drug_details.json", &drugs_json);
            write_to_file("errors.txt", &errors);
        }
    }
    println!("DONE !");
    Ok(())
}

async fn write_drugs_to_db(drugs: Vec<Drug>) {
    let mut queries = vec![];
    drugs.iter().map(|d| {
        let mut dosage = d.dosage.clone().unwrap();
        let mut name = d.name.clone().unwrap();
        dosage = dosage.replace("'", "");
        name = name.replace("'", "");
        let query = format!("insert into drug(set_id,code,name,form,dosage,route) values(\'{}\',\'{}\',\'{}\',\'{}\',\'{}\',\'{}\')",
                d.set_id.clone().unwrap(),
                d.code.clone().unwrap(),
                name,
                d.form.clone().unwrap(),
                dosage,
                d.route.clone().unwrap());
        queries.push(query);
        ()
    }).collect::<()>();
    let pool = PoolInstantiate::init().await;
    let mut transaction = pool.begin().await.unwrap();
    for statement in queries {
        let result = sqlx::query(statement.as_str())
            .execute(&mut transaction).await;
        match result {
            Ok(_) => {
            }
            Err(e) => {
                transaction.rollback();
                eprintln!("{:?}", e);
                eprintln!("error statement = {}", statement);
                panic!("rolled back");
            }
        }
    }
    transaction.commit().await;

}

pub trait Transformer {
    type Input;
    fn transform(&mut self, input: Self::Input) -> Vec<Drug>;
}

pub struct DailyMedTransform;

pub struct CsvTransform;

impl Transformer for DailyMedTransform {
    type Input = Vec<Document>;

    fn transform(&mut self, input: Self::Input) -> Vec<Drug> {
        let mut vec: Vec<Drug> = vec![];
        input.into_iter().for_each(|d| {
            if let Some(ref set_id) = d.set_id.root {
                if let Some(ref component) = d.component {
                    if let Some(ref structured_body) = component.structured_body {
                        if let Some(sub_comp) = structured_body.component.as_ref() {
                            sub_comp.iter().for_each(|s_body| {
                                if let Some(ref section) = s_body.section {
                                    if let Some(ref subjects) = section.subject {
                                        for subject in subjects {
                                            if let Some(manufacturedProduct) = &subject.manufactured_product {
                                                if let Some(ref t) = &manufacturedProduct.manufactured_medicine {
                                                    match &t.ingredient {
                                                        None => {}
                                                        Some(ref ing) => {
                                                            for i in ing.iter() {
                                                                match &i.quantity {
                                                                    None => {}
                                                                    Some(ref q) => {
                                                                        let n_unit = q.numerator.unit.clone().unwrap();
                                                                        let n_value = q.numerator.value.clone().unwrap();
                                                                        let d_unit = q.denominator.unit.clone().unwrap();
                                                                        let d_value = q.denominator.value.clone().unwrap();
                                                                        let mut route = format!("");
                                                                        let mut routeStop = 3;
                                                                        if let Some(c) = &manufacturedProduct.consumed_in {
                                                                            for r in c.iter() {
                                                                                if let Some (rr) = &r.substance_administration {
                                                                                    route.push_str(rr.route_code.display_name.clone().unwrap().as_str());
                                                                                    route.push_str("\n");
                                                                                    if routeStop == 0 {
                                                                                        break;
                                                                                    }
                                                                                    routeStop-=1;
                                                                                }
                                                                            }
                                                                        }
                                                                        let mut form = format!("");
                                                                        match &t.form_code {
                                                                            None => {}
                                                                            Some(ff) => {
                                                                                form.push_str(ff.display_name.clone().unwrap().as_str())
                                                                            }
                                                                        }

                                                                        let drug = Drug::new(
                                                                            None,
                                                                            Some(set_id.clone()),
                                                                            Some(t.code.clone().code),
                                                                            Some(t.name.clone().unwrap_or(format!(""))),
                                                                            Some(form),
                                                                            Some(format!("{}-{} {}-{}", d_value, d_unit, n_value, n_unit)),
                                                                            Some(route.clone()),
                                                                            None);
                                                                        if set_id == "b362b3bd-dda4-f0af-e053-2a95a90aaa49" {
                                                                            println!("{:?}", drug);
                                                                        }
                                                                        vec.push(drug);
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            });
                        }
                    }
                }
            }
        });
        vec
    }
}

impl Transformer for CsvTransform {
    type Input = Vec<String>;

    fn transform(&mut self, input: Self::Input) -> Vec<Drug> {
        vec![]
    }
}


fn process_receives(h_size: usize, rx: Arc<Mutex<Receiver<String>>>, xml_array_clone: Arc<Mutex<Vec<String>>>) -> JoinHandle<()> {
    let handle = tokio::spawn(async move {
        for _ in 0..h_size {
            let rx1_clone = Arc::clone(&rx);
            let xml_array_clone = Arc::clone(&xml_array_clone);
            let result = rx1_clone.lock().unwrap().recv();
            match result {
                Ok(json) => {
                    let mut guard = xml_array_clone.lock().unwrap();
                    guard.push(json.clone());
                    println!("total processed = {}", guard.len());
                }
                Err(e) => {
                    println!("error = {}", e.to_string());
                }
            }
        }
    });
    //handles.clear();
    handle
}

fn spawn(set_id: String, data: Arc<Mutex<Vec<Document>>>, errors: Arc<Mutex<Vec<String>>>) -> JoinHandle<()> {
    tokio::spawn(async move {
        let content_result = crawl_spl_data_info(set_id.clone()).await;
        match content_result {
            Ok(mut content) => {
                content = content.replace("<?xml version=\"1.0\" encoding=\"UTF-8\"?><?xml-stylesheet href=\"https://www.accessdata.fda.gov/spl/stylesheet/spl.xsl\" type=\"text/xsl\"?>", "");
                let d_result = serde_xml_rs::from_str::<Document>(&content);
                match d_result {
                    Ok(drug) => {
                        let mut guard = data.lock().unwrap();
                        guard.push(drug);
                        println!("total processed = {}", guard.len());
                    }
                    Err(e) => {
                        eprintln!("failed to deserialize drug,  {:?}", e);
                        errors.lock().unwrap().push(set_id.clone());
                    }
                }
            }
            _ => {
                errors.lock().unwrap().push(set_id.clone());
            }
        }
    })
}

async fn crawl_spl_data() -> std::io::Result<Vec<SplsData>> {
    let front = format!("https://dailymed.nlm.nih.gov/dailymed/services/v2/spls?page=1&pagesize=100");
    let back = format!("https://dailymed.nlm.nih.gov/dailymed/services/v2/spls?page=1383&pagesize=100");
    let mut visited = Arc::new(Mutex::new(HashSet::new()));
    let mut front_queue = Arc::new(Mutex::new(VecDeque::new()));
    let mut back_queue = Arc::new(Mutex::new(VecDeque::new()));
    let mut total_data = Arc::new(Mutex::new(vec![]));

    let v1 = Arc::clone(&visited);
    let v2 = Arc::clone(&visited);
    let q1 = Arc::clone(&front_queue);
    let q2 = Arc::clone(&back_queue);
    let td1 = Arc::clone(&total_data);
    let td2 = Arc::clone(&total_data);

    let h1 = tokio::spawn(async move {
        {
            let mut q1 = &mut q1.lock().unwrap();
            q1.push_front(front.clone());
            let mut visited = v1.lock().unwrap();
            visited.insert(front.clone());
        }
        while !q1.lock().unwrap().is_empty() {
            let current_url = q1.lock().unwrap().pop_back().unwrap();
            let result = reqwest::get(&current_url.clone()).await;
            match result {
                Ok(response) => {
                    if let Ok(content) = response.text().await {
                        let content_result = serde_json::from_str(&content).unwrap_or(SplRootObject::new());
                        for d in content_result.data {
                            td1.lock().unwrap().push(d.clone());
                        }
                        match content_result.metadata.next_page_url {
                            None => {}
                            Some(ref next_url) => {
                                let mut visited = v1.lock().unwrap();
                                if !visited.contains(next_url) {
                                    visited.insert(next_url.clone());
                                    q1.lock().unwrap().push_front(next_url.clone());
                                }
                            }
                        }
                    }
                }
                Err(_) => {}
            }
        }
    });

    let h2 = tokio::spawn(async move {
        {
            let mut q2 = &mut q2.lock().unwrap();
            q2.push_front(back.clone());
            let mut visited = v2.lock().unwrap();
            visited.insert(back.clone());
        }
        while !q2.lock().unwrap().is_empty() {
            let current_url = q2.lock().unwrap().pop_back().unwrap();
            let result = reqwest::get(&current_url.clone()).await;
            match result {
                Ok(response) => {
                    if let Ok(content) = response.text().await {
                        let content_result = serde_json::from_str(&content).unwrap_or(SplRootObject::new());
                        for d in content_result.data {
                            td2.lock().unwrap().push(d.clone());
                        }
                        match content_result.metadata.previous_page_url {
                            None => {}
                            Some(ref prev_url) => {
                                let mut visited = v2.lock().unwrap();
                                if !visited.contains(prev_url) {
                                    visited.insert(prev_url.clone());
                                    q2.lock().unwrap().push_front(prev_url.clone());
                                }
                            }
                        }
                    }
                }
                Err(_) => {}
            }
        }
    });

    futures::future::join(h1, h2).await;
    let guard = total_data.lock().unwrap();
    println!("crawling set ids DONE!");
    Ok(guard.to_vec())
}

#[derive(Clone, Copy)]
enum Format {
    Xml,
    Json,
}

async fn crawl_drug_details(data: &Vec<SplsData>) -> (Vec<Document>, Vec<String>) {
    let mut drugs = Arc::new(Mutex::new(vec![]));
    let mut errors = Arc::new(Mutex::new(vec![]));
    let mut handles = vec![];
    let mut batch_size = 100;
    let mut left = 0;
    let mut right = data.len() - 1;

    //let data = vec!["930fcbdd-b5fd-4fea-ba4e-008c0317eadd".to_string(), "875f4962-557e-4e12-ad7b-cb5fb6f2a65f".to_string()];
    while left < right {
        let left_set_id = data.get(left).as_ref().unwrap().setid.clone();
        //let left_set_id = data.get(left).unwrap().clone();
        //let right_set_id = data.get(right).unwrap().clone();
        let right_set_id = data.get(right).as_ref().unwrap().setid.clone();
        let mut drugs_c_1 = Arc::clone(&drugs);
        let mut drugs_c_2 = Arc::clone(&drugs);
        let mut errors_c_1 = Arc::clone(&errors);
        let mut errors_c_2 = Arc::clone(&errors);
        handles.push(spawn(left_set_id, drugs_c_1, errors_c_1));
        handles.push(spawn(right_set_id, drugs_c_2, errors_c_2));
        let h_size = handles.len();
        if h_size % batch_size == 0 {
            for h in &mut handles {
                h.await;
            }
            handles.clear();
        }
        left += 1;
        right -= 1;
    }

    let h_size = handles.len();
    if h_size > 0 {
        for h in &mut handles {
            h.await;
        }
        handles.clear();
    }

    let v1 = drugs.lock().unwrap().to_vec();
    let v2 = errors.lock().unwrap().to_vec();
    (v1, v2)
}

fn load_spl_data() -> std::io::Result<Vec<SplsData>> {
    Ok(read_from_file("spl_data2.json"))
}

fn load_drug_data() -> std::io::Result<Vec<Document>> {
    Ok(read_from_file("drug_details.json"))
}

async fn crawl_spl_data_info(set_id: String) -> std::io::Result<String> {
    let url = format!("https://dailymed.nlm.nih.gov/dailymed/services/v2/spls/{}", set_id);
    let client = reqwest::Client::new();
    let result = client.get(url).header("content-type", "application/xml").send().await;

    match result {
        Ok(response) => {
            let content_result = response.text().await;
            match content_result {
                Ok(content) => {
                    Ok(content)
                }
                Err(_) => {
                    Err(std::io::Error::new(ErrorKind::Other, "whatever"))
                }
            }
        }
        Err(_) => {
            Err(std::io::Error::new(ErrorKind::Other, "whatever"))
        }
    }
}

fn write_to_file(file_name: &str, content: &String) {
    let mut file = File::create(file_name).unwrap();
    file.write_all(content.as_bytes());
}

fn read_from_file<T: DeserializeOwned>(file_name: &str) -> T {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    contents = contents.replace(" ", "");
    serde_json::from_str(&contents).unwrap()
}

fn read_from_file_without_mapping(file_name: &str) -> String {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    contents
}

// fn convert_from_xml_to_json(xml: &String) -> std::io::Result<String> {
//     let json = xml_string_to_json(xml.clone(), &Config::new_with_defaults());
//     match json {
//         Ok(value) => {
//             Ok(value.to_string())
//         }
//         Err(_) => {
//             Err(std::io::Error::new(ErrorKind::Other, "failed to parse xml"))
//         }
//     }
// }


