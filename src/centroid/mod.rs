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

pub struct Centroid {
    pub mean: f64,
    pub weight: f64,
    pub sort_key1: isize,
    pub sort_key2: isize,
}

impl Centroid {
    pub fn to_string(&self) -> String {
        format!(
            "{{\"mean\": \"{mean}\",\"weight\": \"{weight}\"}}",
            mean = self.mean,
            weight = self.weight
        )
    }

    pub fn add(&mut self, r: &Centroid) -> String {
        if r.weight < 0.0 {
            return "centroid weight cannot be less than zero".to_string();
        }
        if self.weight != 0.0 {
            self.weight += r.weight;
            self.mean += r.weight * (r.mean - self.mean) / self.weight;
        } else {
            self.weight = r.weight;
            self.mean = r.mean;
        }
        "".to_string()
    }

    pub fn clone(&self) -> Centroid {
        Centroid {
            mean: self.mean,
            weight: self.weight,
            sort_key1: self.mean.floor() as isize,
            sort_key2: (self.mean.signum() * self.mean.fract()) as isize,
        }
    }
}
