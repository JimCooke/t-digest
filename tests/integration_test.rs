extern crate tdigest;

use std::fs::File;
use std::io::{BufRead, BufReader};
use tdigest::Tdigest;

// The compression parameter.  100 is a common value for normal uses.  1000 is extremely large.
const COMPRESSION: f64 = 100.0;

#[test]
fn integation_test_save_digest() {
    let mut t = Tdigest::new(COMPRESSION);
    t.add(469.20, 1.0);
    let fspec = "centroids/centroid_test.json";
    match t.save_digest(fspec.to_string()) {
        Ok(_) => (),
        Err(e) => {
            panic!("Failed to save centroid {}: {}", fspec, e);
        }
    }
}

fn integration_test_digest(fname: String) {
    println!("Testing {} ...", fname);
    // Load the tdigest
    let mut t = Tdigest::new(COMPRESSION);
    let fspec = format!("data/{}.dat", fname);
    let f = File::open(&fspec);
    let f = match f {
        Ok(file) => file,
        Err(e) => {
            panic!("Failed to open dataset {}: {}", fspec, e);
        }
    };
    for line in BufReader::new(f).lines() {
        match line {
            Err(e) => {
                panic!("Failed reading data line in {}: {}", fspec, e);
            }
            Ok(string) => match string.trim().parse::<f64>() {
                Err(e) => {
                    panic!("Non-float input data in {}: {}", fspec, e);
                }
                Ok(number) => {
                    t.add(number, 1.0);
                }
            },
        }
    }

    // Load the statistics file ... the test 'answers', if you will, generated with R
    let fspec = format!(
        "data/{}.sta",
        fname.replace("-asc", "").replace("-desc", "")
    );
    let f = File::open(&fspec);
    let f = match f {
        Ok(file) => file,
        Err(e) => {
            panic!(
                "Failed to open dataset statistics check file {}: {}",
                fspec, e
            );
        }
    };
    let mut idx: usize = 0;
    let mut headervec: Vec<String> = Vec::new();
    let mut answervec: Vec<f64> = Vec::new();
    for line in BufReader::new(f).lines() {
        idx += 1;
        let mut str = match line {
            Err(e) => {
                panic!("Failed reading answer line in {}: {}", fspec, e);
            }
            Ok(s) => s,
        };
        let mut split = str.split_whitespace();
        for s in split {
            let clone = s.to_string().to_lowercase().clone();
            if idx == 1 {
                headervec.push(clone);
            } else {
                match clone.parse::<f64>() {
                    Err(e) => {
                        panic!("Non-float answer data in {}: {}", fspec, e);
                    }
                    Ok(number) => {
                        answervec.push(number);
                    }
                }
            }
        }
    }
    assert_eq!(headervec.len(), answervec.len());
    // Save the centroid details in case you need to start investigating a mismatch
    match t.save_digest("centroids/centroid_calc_test.json".to_string()) {
        Ok(_) => (),
        Err(_err) => {
            panic!("Failed to save centroid");
        }
    }

    // Iterate through the answers, checking each labelled answer against its attribute in the t-digest
    idx = 0;
    let mut doing_percentiles: bool = false;
    for stat in headervec {
        match Some(stat) {
            Some(ref stat) if stat.starts_with("pctl") => {
                if !doing_percentiles {
                    doing_percentiles = true;
                    println!("   Percentiles test");
                }
                let pctl: f64 = match stat.replace("pctl", "").parse::<f64>() {
                    Err(_e) => {
                        panic!("Non-float percentile given for test {}", stat);
                    }
                    Ok(number) => number,
                };
                if (t.quantile(pctl / 100.0) - answervec[idx]).abs() / answervec[idx].abs() < 0.05 {
                    //  println!(
                    //      "      ... percentile {} correct {} ~= {}",
                    //      pctl,
                    //      t.quantile(pctl / 100.0),
                    //      answervec[idx]
                    //  );
                } else {
                    println!(
                        "      ... percentile {} incorrect {} !~= {}",
                        pctl,
                        t.quantile(pctl / 100.0),
                        answervec[idx]
                    );
                }
            }
            Some(ref stat) if stat == "mean" => {
                println!("   Mean test");
                // For large sample sets as some amount of rounding error is expected.
                // Test for success as the rounding error percent, not the absolute value of the rounding error
                assert!(
                    (t.mean() - answervec[idx]).abs() / answervec[idx].abs() < 0.001,
                    format!(
                        "       ... mean test FAILED! {} ~<> {}",
                        t.mean(),
                        answervec[idx]
                    )
                );
            }
            Some(ref stat) if stat == "var" => {
                println!("   Variance test");
                // For large sample sets as some amount of rounding error is expected.
                // Test for success as the rounding error percent, not the absolute value of the rounding error
                assert!(
                    (t.variance() - answervec[idx]).abs() / answervec[idx].abs() < 0.01,
                    format!(
                        "       ... variance test FAILED! {} ~<> {}",
                        t.variance(),
                        answervec[idx]
                    )
                );
            }
            Some(ref stat) if stat == "stdev" => {
                println!("   Standard deviation test");
                // For large sample sets as some amount of rounding error is expected.
                // Test for success as the rounding error percent, not the absolute value of the rounding error
                assert!(
                    (t.stdev() - answervec[idx]).abs() / answervec[idx].abs() < 0.01,
                    format!(
                        "       ... standard deviation test FAILED! {} ~<> {}",
                        t.stdev(),
                        answervec[idx]
                    )
                );
            }
            Some(ref stat) if stat == "sum" => {
                println!("   Sum test");
                // For large sample sets as some amount of rounding error is expected.
                // Test for success as the rounding error percent, not the absolute value of the rounding error
                assert!(
                    (t.sum() - answervec[idx]).abs() / answervec[idx].abs() < 0.001,
                    format!(
                        "    ... sum test FAILED! {} ~<> {}",
                        t.sum(),
                        answervec[idx]
                    )
                );
            }
            Some(ref stat) if stat == "count" => {
                println!("   Count test");
                assert!(
                    (t.count() - answervec[idx]).abs() < 0.001,
                    format!(
                        "       ... count test FAILED! {} <> {}",
                        t.count(),
                        answervec[idx]
                    )
                );
            }
            x => println!("Unknown statistic in answer file {:?}", x),
        }
        idx += 1;
    }
}

#[test]
fn integation_test_digests_with_datasets() {
    //integration_test_digest("large-normal".to_string());
    //integration_test_digest("large-normal-asc".to_string());
    //integration_test_digest("large-normal-desc".to_string());
    //integration_test_digest("large-skew".to_string());
    //integration_test_digest("large-skew-asc".to_string());
    //integration_test_digest("large-skew-desc".to_string());
    //integration_test_digest("large-uniform".to_string());
    //integration_test_digest("large-uniform-asc".to_string());
    //integration_test_digest("large-uniform-desc".to_string());
    //integration_test_digest("mass-point-left".to_string());
    //integration_test_digest("mass-point-left-asc".to_string());
    //integration_test_digest("mass-point-left-desc".to_string());
    //integration_test_digest("mass-point-right".to_string());
    //integration_test_digest("mass-point-right-asc".to_string());
    //integration_test_digest("mass-point-right-desc".to_string());
    integration_test_digest("small-normal".to_string());
    //integration_test_digest("small-normal-asc".to_string());
    //integration_test_digest("small-normal-desc".to_string());
    //integration_test_digest("small-skew".to_string());
    //integration_test_digest("small-skew-asc".to_string());
    //integration_test_digest("small-skew-desc".to_string());
    //integration_test_digest("small-uniform".to_string());
    //integration_test_digest("small-uniform-asc".to_string());
    //integration_test_digest("small-uniform-desc".to_string());
}

fn integration_test_merge(fname: String) {
    let mut answervec: Vec<f64> = Vec::new();
    // Load the single tdigest and each separate chunk and write to json
    for fnam in vec![
        format!("{}", fname),
        format!("{}-chunk1", fname),
        format!("{}-chunk2", fname),
    ] {
        let fspec = format!("data/{}.dat", fnam);
        println!("{}", fspec);
        let mut t = Tdigest::new(COMPRESSION);
        let f = File::open(&fspec);
        let f = match f {
            Ok(file) => file,
            Err(e) => {
                panic!("Failed to open dataset {}: {}", fspec, e);
            }
        };
        for line in BufReader::new(f).lines() {
            match line {
                Err(e) => {
                    panic!("Failed reading data line in {}: {}", fspec, e);
                }
                Ok(string) => match string.trim().parse::<f64>() {
                    Err(e) => {
                        panic!("Non-float input data in {}: {}", fspec, e);
                    }
                    Ok(number) => {
                        t.add(number, 1.0);
                    }
                },
            }
        }
        // Save the digest to a centroids file
        let fspec = format!("centroids/{}.json", fnam);
        match t.save_digest(fspec.to_string()) {
            Ok(_) => (),
            Err(e) => {
                panic!("Failed to save centroids {}: {}", fspec, e);
            }
        }
        // If you are working with the single large data file then save the answers into memory for later
        // checking against a digest created by merging two separate digests created by reading chunks of the
        // single data load separately, ie, record the results from the non-chunk file.
        if !fnam.contains("chunk") {
            answervec.push(t.quantile(00.5 / 100.0));
            answervec.push(t.quantile(01.0 / 100.0));
            answervec.push(t.quantile(05.0 / 100.0));
            answervec.push(t.quantile(10.0 / 100.0));
            answervec.push(t.quantile(25.0 / 100.0));
            answervec.push(t.quantile(50.0 / 100.0));
            answervec.push(t.quantile(75.0 / 100.0));
            answervec.push(t.quantile(90.0 / 100.0));
            answervec.push(t.quantile(95.0 / 100.0));
            answervec.push(t.quantile(99.0 / 100.0));
            answervec.push(t.quantile(99.5 / 100.0));
            answervec.push(t.stdev());
            answervec.push(t.sum());
            answervec.push(t.count());
            answervec.push(t.min());
            answervec.push(t.max());
        }
    }
    // Reload the single tdigest from json and compare it to the one that was individually populated
    let mut t1 = Tdigest::new(COMPRESSION);
    let fspec = format!("centroids/{}.json", fname);
    t1.load_digest(fspec.to_string());
    assert_eq!(t1.quantile(00.5 / 100.0), answervec[0]);
    assert_eq!(t1.quantile(01.0 / 100.0), answervec[1]);
    assert_eq!(t1.quantile(05.0 / 100.0), answervec[2]);
    assert_eq!(t1.quantile(10.0 / 100.0), answervec[3]);
    assert_eq!(t1.quantile(25.0 / 100.0), answervec[4]);
    assert_eq!(t1.quantile(50.0 / 100.0), answervec[5]);
    assert_eq!(t1.quantile(75.0 / 100.0), answervec[6]);
    assert_eq!(t1.quantile(90.0 / 100.0), answervec[7]);
    assert_eq!(t1.quantile(95.0 / 100.0), answervec[8]);
    assert_eq!(t1.quantile(99.0 / 100.0), answervec[9]);
    assert_eq!(t1.quantile(99.5 / 100.0), answervec[10]);
    assert_eq!(t1.stdev(), answervec[11]);
    assert_eq!(t1.sum(), answervec[12]);
    assert_eq!(t1.count(), answervec[13]);
    assert_eq!(t1.min(), answervec[14]);
    assert_eq!(t1.max(), answervec[15]);

    // Load a new digest from the first chunk, merge the digest from the second chunk and compare it to the single
    let mut t1 = Tdigest::new(COMPRESSION);
    let fspec = format!("centroids/{}-chunk1.json", fname);
    t1.load_digest(fspec.to_string());
    let mut t2 = Tdigest::new(COMPRESSION);
    let fspec = format!("centroids/{}-chunk2.json", fname);
    t2.load_digest(fspec.to_string());
    t1.merge_digest(t2);

    // Test that our results are CLOSE (not exact) to the single monolith digest
    println!("   Percentiles test (against monolith, not answer file)");
    let mut pos: usize = 0;
    for pctl in vec![
        0.005, 0.01, 0.05, 0.1, 0.25, 0.5, 0.75, 0.9, 0.95, 0.99, 0.995,
    ] {
        if (t1.quantile(pctl) - answervec[pos]).abs() / answervec[pos].abs() >= 0.05 {
            println!(
                "      ... percentile {} incorrect {} !~= {}",
                pctl,
                t1.quantile(pctl),
                answervec[pos]
            );
        }
        pos += 1;
    }
    println!("   Standard deviation test (against monolith, not answerfile)");
    if (t1.stdev() - answervec[11]).abs() / answervec[11].abs() < 0.01 {
        //println!(
        //    "      ... standard deviation test passed! {} ~= {}",
        //    t1.stdev(),
        //    answervec[11]
        //);
    } else {
        println!(
            "      ... standard deviation test failed! {} !~= {}",
            t1.stdev(),
            answervec[11]
        );
    }
    println!("   Sum test (against monolith, not answerfile)");
    if (t1.sum() - answervec[12]).abs() / answervec[12].abs() < 0.01 {
        //println!(
        //    "      ... sum test passed! {} ~= {}",
        //    t1.sum(),
        //    answervec[12]
        //);
    } else {
        println!(
            "      ... sum test failed! {} !~= {}",
            t1.sum(),
            answervec[12]
        );
    }
    println!("   Count (against monolith, not answerfile)");
    if (t1.count() - answervec[13]).abs() / answervec[13].abs() < 0.01 {
        //println!(
        //    "      ... count test passed! {} ~= {}",
        //    t1.count(),
        //    answervec[13]
        //);
    } else {
        println!(
            "      ... count test failed! {} !~= {}",
            t1.count(),
            answervec[13]
        );
    }
    println!("   Min (against monolith, not answerfile)");
    if (t1.min() - answervec[14]).abs() / answervec[14].abs() < 0.01 {
        //println!(
        //    "      ... min test passed! {} ~= {}",
        //    t1.min(),
        //    answervec[14]
        //);
    } else {
        println!(
            "      ... min test failed! {} !~= {}",
            t1.min(),
            answervec[14]
        );
    }
    println!("   Max (against monolith, not answerfile)");
    if (t1.max() - answervec[15]).abs() / answervec[15].abs() >= 0.01 {
        println!(
            "      ... max test failed! {} !~= {}",
            t1.max(),
            answervec[15]
        );
    }
}

#[test]
fn run_merge_test() {
    integration_test_merge("large-normal".to_string());
}
