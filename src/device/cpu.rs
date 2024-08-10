// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::sync::Arc;

use async_trait::async_trait;

use super::Device;

pub struct Cpu {
    cores: usize,
    threads: usize,
    frequency: f64,
}

impl Cpu {
    pub fn new(cores: usize, threads: usize, frequency: f64) -> Self {
        Cpu {
            cores,
            threads,
            frequency,
        }
    }

    pub fn cores(&self) -> usize {
        self.cores
    }

    pub fn threads(&self) -> usize {
        self.threads
    }

    pub fn frequency(&self) -> f64 {
        self.frequency
    }
}

#[async_trait]
impl Device for Cpu {
    async fn execute<F: Send, R: Send>(&self, computation: F) -> crate::Result<Arc<R>>
    where F: FnOnce() -> crate::Result<Arc<R>> {
        computation()
    }
}
