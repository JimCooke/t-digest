/*
 * Licensed to Jim Cooke under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
extern crate serde_json;
extern crate tdigest;

// The compression parameter.  100 is a common value for normal uses.  1000 is extremely large.
const COMPRESSION: f64 = 100.0;

fn main() {
    let mut t = tdigest::Tdigest::new(COMPRESSION);
    t.load_digest("centroids/large-normal.json".to_string());
    println!("Dummy main program.  Load digest worked");
}
